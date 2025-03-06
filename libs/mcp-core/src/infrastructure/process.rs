use std::collections::HashMap;
use async_trait::async_trait;
use log::info;
use serde_json::{json, Value};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt},
    process::{Child, ChildStdin, ChildStdout},
    time::Duration,
};

use crate::domain::traits::ProcessManager;
use crate::domain::errors::DomainError;
use crate::dm_process::DMProcess;
use crate::models::models::{ToolConfiguration, ToolId, ToolType};

pub struct TokioProcessManager;

impl TokioProcessManager {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl ProcessManager for TokioProcessManager {
    async fn spawn_process(
        &self,
        configuration: &Value,
        tool_id: &str,
        tool_type: &str,
        env_vars: Option<&HashMap<String, String>>,
    ) -> Result<(Child, ChildStdin, ChildStdout), DomainError> {
        let command = configuration
            .get("command")
            .and_then(|v| v.as_str())
            .ok_or_else(|| "Configuration missing 'command' field or not a string".to_string())?;

        let args = configuration
            .get("args")
            .and_then(|v| v.as_array())
            .map(|args| {
                args.iter()
                    .filter_map(|arg| arg.as_str().map(|s| s.to_string()))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let config = ToolConfiguration {
            command: command.to_string(),
            args: Some(args),
        };

        let tool_type = match tool_type {
            "node" => ToolType::Node,
            "python" => ToolType::Python,
            "docker" => ToolType::Docker,
            _ => return Err(DomainError::InvalidToolConfiguration(format!("Unsupported tool type: {}", tool_type))),
        };

        let tool_id = ToolId::new(tool_id.to_string());
        let dm_process = DMProcess::new(&tool_id, &tool_type, &config, env_vars).await?;
        Ok((dm_process.child, dm_process.stdin, dm_process.stdout))
    }

    async fn kill_process(&self, process: &mut Child) -> Result<(), DomainError> {
        match process.kill().await {
            Ok(_) => Ok(()),
            Err(e) => Err(DomainError::ProcessError(format!("Failed to kill process: {}", e))),
        }
    }

    async fn discover_server_tools(
        &self,
        server_id: &str,
        process_ios: &mut HashMap<String, (ChildStdin, ChildStdout)>,
    ) -> Result<Vec<Value>, DomainError> {
        // Get the stdin/stdout handles for the server
        let (stdin, stdout) = match process_ios.get_mut(server_id) {
            Some(io) => io,
            None => return Err(DomainError::ServerNotFound(format!("Server {} not found or not running", server_id))),
        };

        info!("Discovering tools from server {}", server_id);

        // According to MCP specification, the correct method is "tools/list"
        let discover_cmd = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/list",
            "params": {}
        });

        // Send the command to the process
        let cmd_str = serde_json::to_string(&discover_cmd)
            .map_err(|e| DomainError::SerializationError(format!("Failed to serialize command: {}", e)))?
            + "\n";

        info!("Command: {}", cmd_str.trim());

        // Write command to stdin
        stdin
            .write_all(cmd_str.as_bytes())
            .await
            .map_err(|e| DomainError::StdinWriteError(format!("Failed to write to process stdin: {}", e)))?;
        stdin
            .flush()
            .await
            .map_err(|e| DomainError::StdinFlushError(format!("Failed to flush stdin: {}", e)))?;

        // Read the response with a timeout
        let mut reader = tokio::io::BufReader::new(&mut *stdout);
        let mut response_line = String::new();

        let read_result = tokio::time::timeout(
            Duration::from_secs(10),
            reader.read_line(&mut response_line),
        )
        .await;

        match read_result {
            Ok(Ok(0)) => return Err(DomainError::ServerClosedConnection),
            Ok(Ok(_)) => info!(
                "Received response from server {}: {}",
                server_id,
                response_line.trim()
            ),
            Ok(Err(e)) => return Err(DomainError::StdoutReadError(format!("Failed to read from process stdout: {}", e))),
            Err(_) => {
                return Err(DomainError::TimeoutError(format!(
                    "Timeout waiting for response from server {}",
                    server_id
                )))
            }
        }

        if response_line.is_empty() {
            return Err(DomainError::NoResponse);
        }

        // Parse the response
        let response: Value = match serde_json::from_str(&response_line) {
            Ok(json) => json,
            Err(e) => return Err(DomainError::JsonParseError(format!("Failed to parse response as JSON: {}", e))),
        };

        // Check for error in the response
        if let Some(error) = response.get("error") {
            return Err(DomainError::ToolExecutionError(format!("Server returned error: {:?}", error)));
        }

        // According to MCP spec, tools should be in the result field
        if let Some(result) = response.get("result") {
            // MCP returns tools directly in the result field as array
            if let Some(tools_array) = result.as_array() {
                info!("Found {} tools in result array", tools_array.len());
                return Ok(tools_array.clone());
            }

            // Some implementations might nest it under a tools field
            if let Some(tools) = result.get("tools") {
                if let Some(tools_array) = tools.as_array() {
                    info!("Found {} tools in result.tools array", tools_array.len());
                    return Ok(tools_array.clone());
                }
            }

            // If there's a result but we couldn't find tools array, try to use the entire result
            info!("No tools array found, using entire result as fallback");
            return Ok(vec![result.clone()]);
        }

        // If the server doesn't fully comply with MCP but has a tools field at root
        if let Some(tools) = response.get("tools") {
            if let Some(tools_array) = tools.as_array() {
                info!("Found {} tools in root tools array", tools_array.len());
                return Ok(tools_array.clone());
            }
        }

        // If initialization hasn't completed yet or tools are not supported,
        // return an empty array as fallback
        info!("No tools found in response: {}", response_line.trim());
        Ok(Vec::new())
    }

    async fn execute_server_tool(
        &self,
        server_id: &str,
        tool_id: &str,
        parameters: Value,
        process_ios: &mut HashMap<String, (ChildStdin, ChildStdout)>,
    ) -> Result<Value, DomainError> {
        let (stdin, stdout) = process_ios
            .get_mut(server_id)
            .ok_or_else(|| DomainError::ServerNotFound(server_id.to_string()))?;

        let execute_cmd = json!({
            "jsonrpc": "2.0",
            "id": format!("execute_{}_{}", server_id, tool_id),
            "method": "tools/call",
            "params": { "name": tool_id, "arguments": parameters }
        });

        let cmd_str = serde_json::to_string(&execute_cmd)
            .map_err(|e| DomainError::SerializationError(e.to_string()))?
            + "\n";

        stdin
            .write_all(cmd_str.as_bytes())
            .await
            .map_err(|e| DomainError::StdinWriteError(e.to_string()))?;
        stdin
            .flush()
            .await
            .map_err(|e| DomainError::StdinFlushError(e.to_string()))?;

        let mut reader = tokio::io::BufReader::new(&mut *stdout);
        let mut response_line = String::new();

        let read_result = tokio::time::timeout(
            Duration::from_secs(30),
            reader.read_line(&mut response_line),
        )
        .await;

        match read_result {
            Ok(Ok(0)) => return Err(DomainError::ServerClosedConnection),
            Ok(Ok(_)) => {}
            Ok(Err(e)) => return Err(DomainError::StdoutReadError(e.to_string())),
            Err(_) => return Err(DomainError::TimeoutError(server_id.to_string())),
        }

        if response_line.is_empty() {
            return Err(DomainError::NoResponse);
        }

        let response: Value = serde_json::from_str(&response_line)
            .map_err(|e| DomainError::JsonParseError(e.to_string()))?;

        if let Some(error) = response.get("error") {
            return Err(DomainError::ToolExecutionError(error.to_string()));
        }

        response
            .get("result")
            .cloned()
            .ok_or(DomainError::NoResultField)
    }
}
