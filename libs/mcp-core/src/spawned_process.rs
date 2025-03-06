use crate::{
    error::{MCPError, MCPResult},
    models::types::{ToolConfiguration, ToolId, ToolType},
};
use log::{error, info};
use serde_json::json;
use std::collections::HashMap;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt},
    process::{Child, ChildStdin, ChildStdout, Command},
    time::Duration,
};

pub struct SpawnedProcess {
    pub stdin: ChildStdin,
    pub stdout: ChildStdout,
    pub child: Child,
}

impl SpawnedProcess {
    pub async fn new(
        tool_id: &ToolId,
        tool_type: &ToolType,
        config: &ToolConfiguration,
        env_vars: Option<&HashMap<String, String>>,
    ) -> Result<Self, String> {
        match tool_type {
            ToolType::Node => Self::spawn_nodejs_process(tool_id, config, env_vars).await,
            ToolType::Python => Self::spawn_python_process(tool_id, config, env_vars).await,
            ToolType::Docker => Self::spawn_docker_process(tool_id, config, env_vars).await,
        }
    }

    pub async fn kill(&mut self) -> Result<(), String> {
        self.child
            .kill()
            .await
            .map_err(|e| format!("Failed to kill process: {}", e))
    }

    async fn spawn_nodejs_process(
        tool_id: &ToolId,
        config: &ToolConfiguration,
        env_vars: Option<&HashMap<String, String>>,
    ) -> Result<Self, String> {
        info!("Spawning Node.js process for tool ID: {}", tool_id);

        let command = &config.command;
        if !command.contains("npx") && !command.contains("node") {
            error!("Entry point doesn't exist and doesn't look like an npm package or node command for tool {}: {}", tool_id, command);
            return Err(format!("Entry point file '{}' does not exist", command));
        }

        info!(
            "Using command to start process for tool {}: {}",
            tool_id, command
        );
        let mut cmd = Command::new(command);

        if let Some(args) = &config.args {
            info!(
                "Adding {} arguments to command for tool {}",
                args.len(),
                tool_id
            );
            for (i, arg) in args.iter().enumerate() {
                info!("Arg {}: {}", i, arg);
                cmd.arg(arg);
            }
        } else {
            info!("No arguments found in configuration for tool {}", tool_id);
        }

        info!("Adding tool-id argument: {}", tool_id);
        cmd.arg("--tool-id").arg(tool_id.as_str());

        Self::setup_process(cmd, tool_id, env_vars).await
    }

    async fn spawn_python_process(
        tool_id: &ToolId,
        config: &ToolConfiguration,
        env_vars: Option<&HashMap<String, String>>,
    ) -> Result<Self, String> {
        info!("Spawning Python process for tool ID: {}", tool_id);
        info!("Using Python command: {}", config.command);

        let mut cmd = Command::new(&config.command);

        if let Some(args) = &config.args {
            info!("Args: {:?}", args);
            for arg in args {
                cmd.arg(arg);
            }
        }

        Self::setup_process(cmd, tool_id, env_vars).await
    }

    async fn spawn_docker_process(
        tool_id: &ToolId,
        config: &ToolConfiguration,
        env_vars: Option<&HashMap<String, String>>,
    ) -> Result<Self, String> {
        info!("Spawning Docker process for tool ID: {}", tool_id);

        if config.command != "docker" {
            return Err(format!(
                "Expected 'docker' command for Docker runtime, got '{}'",
                config.command
            ));
        }

        info!("Using Docker command");
        let mut cmd = Command::new(&config.command);

        cmd.arg("run")
            .arg("-i")
            .arg("--name")
            .arg(tool_id.as_str())
            .arg("-a")
            .arg("stdout")
            .arg("-a")
            .arg("stderr")
            .arg("-a")
            .arg("stdin")
            .arg("--rm");

        if let Some(args) = &config.args {
            info!("Args: {:?}", args);
            for arg in args {
                cmd.arg(arg);
            }
        }

        Self::setup_process(cmd, tool_id, env_vars).await
    }

    async fn setup_process(
        mut cmd: Command,
        tool_id: &ToolId,
        env_vars: Option<&HashMap<String, String>>,
    ) -> Result<Self, String> {
        use std::process::Stdio;

        if let Some(env_map) = env_vars {
            info!(
                "Setting {} environment variables for tool {}",
                env_map.len(),
                tool_id
            );
            for (key, value) in env_map {
                info!(
                    "Setting environment variable for tool {}: {}={}",
                    tool_id, key, value
                );
                cmd.env(key, value);
            }
        } else {
            info!("No environment variables provided for tool {}", tool_id);
        }

        cmd.stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        info!(
            "Spawning process for tool {}: {:?} with args: {:?}",
            tool_id,
            cmd.as_std().get_program(),
            cmd.as_std().get_args().collect::<Vec<_>>()
        );

        let mut child = cmd
            .spawn()
            .map_err(|e| format!("Failed to spawn process: {}", e))?;

        // Capture stderr to a separate task that logs errors
        if let Some(stderr) = child.stderr.take() {
            let tool_id_clone = tool_id.clone();
            std::mem::drop(tokio::spawn(async move {
                let mut stderr_reader = tokio::io::BufReader::new(stderr);
                let mut line = String::new();
                while let Ok(bytes_read) = stderr_reader.read_line(&mut line).await {
                    if bytes_read == 0 {
                        break;
                    }
                    info!("[{} stderr]: {}", tool_id_clone, line.trim());
                    line.clear();
                }
            }));
        }

        let stdin = child.stdin.take().ok_or_else(|| {
            std::mem::drop(child.kill());
            "Failed to open stdin".to_string()
        })?;

        let stdout = child.stdout.take().ok_or_else(|| {
            std::mem::drop(child.kill());
            "Failed to open stdout".to_string()
        })?;

        info!("Process spawned successfully with stdin and stdout pipes");
        Ok(Self {
            stdin,
            stdout,
            child,
        })
    }

    pub async fn send_command(
        &mut self,
        method: &str,
        params: serde_json::Value,
    ) -> MCPResult<serde_json::Value> {
        let command = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": method,
            "params": params
        });

        let cmd_str = serde_json::to_string(&command)
            .map_err(|e| MCPError::SerializationError(e.to_string()))?
            + "\n";

        self.stdin
            .write_all(cmd_str.as_bytes())
            .await
            .map_err(|e| MCPError::StdinWriteError(e.to_string()))?;

        self.stdin
            .flush()
            .await
            .map_err(|e| MCPError::StdinFlushError(e.to_string()))?;

        let mut reader = tokio::io::BufReader::new(&mut self.stdout);
        let mut response_line = String::new();

        let read_result = tokio::time::timeout(
            Duration::from_secs(30),
            reader.read_line(&mut response_line),
        )
        .await;

        match read_result {
            Ok(Ok(0)) => Err(MCPError::ServerClosedConnection),
            Ok(Ok(_)) => {
                let response: serde_json::Value = serde_json::from_str(&response_line)
                    .map_err(|e| MCPError::JsonParseError(e.to_string()))?;

                if let Some(error) = response.get("error") {
                    Err(MCPError::ToolExecutionError(error.to_string()))
                } else {
                    response
                        .get("result")
                        .cloned()
                        .ok_or(MCPError::NoResultField)
                }
            }
            Ok(Err(e)) => Err(MCPError::StdoutReadError(e.to_string())),
            Err(_) => Err(MCPError::TimeoutError("Response timeout".to_string())),
        }
    }
}
