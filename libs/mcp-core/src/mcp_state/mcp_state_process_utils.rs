use std::collections::HashMap;

use serde_json::Value;
use tokio::process::Child;

use crate::{
    mcp_protocol::initialize_server_connection,
    types::{ServerConfiguration, ServerEnvironment, ServerId, ToolType},
    SpawnedProcess,
};

/// Spawn an MCP server process using DMProcess
pub async fn spawn_process(
    configuration: &Value,
    tool_id: &str,
    tools_type: &str,
    env_vars: Option<&HashMap<String, String>>,
) -> Result<
    (
        Child,
        tokio::process::ChildStdin,
        tokio::process::ChildStdout,
    ),
    String,
> {
    let command = configuration
        .get("command")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "Configuration missing 'command' field or not a string".to_string())?;

    // Special handling for "uv run" command
    let (command_str, mut command_args) = if command == "uv run" {
        ("uv".to_string(), vec!["run".to_string()])
    } else {
        (command.to_string(), Vec::new())
    };

    // Get additional args from configuration
    let config_args = configuration
        .get("args")
        .and_then(|v| v.as_array())
        .map(|args| {
            args.iter()
                .filter_map(|arg| arg.as_str().map(|s| s.to_string()))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    
    // Combine command args with config args
    command_args.extend(config_args);
    
    // Add tool_id as an argument for Python/UV projects
    if command == "uv run" {
        command_args.push("--tool-id".to_string());
        command_args.push(tool_id.to_string());
    }

    let config = ServerConfiguration {
        command: Some(command_str),
        args: Some(command_args),
        env: env_vars.map(|vars| {
            vars.iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        ServerEnvironment {
                            description: "".to_string(),
                            default: Some(v.clone()),
                            required: false,
                        },
                    )
                })
                .collect()
        }),
    };

    let tools_type = match tools_type {
        "node" => ToolType::Node,
        "python" => ToolType::Python,
        "docker" => ToolType::Docker,
        _ => return Err(format!("Unsupported tool type: {}", tools_type)),
    };

    let tool_id = ServerId::new(tool_id.to_string());
    let mut dm_process = SpawnedProcess::new(&tool_id, &tools_type, &config, env_vars).await?;
    let _ = initialize_server_connection(
        tool_id.as_str(),
        &mut dm_process.stdin,
        &mut dm_process.stdout,
    )
    .await;
    Ok((dm_process.child, dm_process.stdin, dm_process.stdout))
}

/// Kill a running process
pub async fn kill_process(process: &mut Child) -> Result<(), String> {
    match process.kill().await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to kill process: {}", e)),
    }
}
