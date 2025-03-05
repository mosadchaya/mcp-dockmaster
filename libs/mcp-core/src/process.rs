use crate::error::{MCPError, MCPResult};
use crate::models::{ToolConfiguration, ToolId, ToolType};
use log::{error, info};
use serde_json::json;
use std::collections::HashMap;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt},
    process::{Child, ChildStdin, ChildStdout, Command},
    time::Duration,
};

pub struct ProcessManager {
    pub stdin: ChildStdin,
    pub stdout: ChildStdout,
    pub child: Child,
}

impl ProcessManager {
    pub async fn new(
        tool_id: &ToolId,
        tool_type: &ToolType,
        config: &ToolConfiguration,
        env_vars: Option<&HashMap<String, String>>,
    ) -> MCPResult<Self> {
        match tool_type {
            ToolType::Node => Self::spawn_nodejs_process(tool_id, config, env_vars).await,
            ToolType::Python => Self::spawn_python_process(tool_id, config, env_vars).await,
            ToolType::Docker => Self::spawn_docker_process(tool_id, config, env_vars).await,
        }
    }

    pub async fn kill(&mut self) -> MCPResult<()> {
        self.child
            .kill()
            .await
            .map_err(|e| MCPError::ProcessError(e.to_string()))
    }

    async fn spawn_nodejs_process(
        tool_id: &ToolId,
        config: &ToolConfiguration,
        env_vars: Option<&HashMap<String, String>>,
    ) -> MCPResult<Self> {
        info!("Spawning Node.js process for tool ID: {}", tool_id);

        let mut cmd = Command::new(&config.command);

        if let Some(args) = &config.args {
            for arg in args {
                cmd.arg(arg);
            }
        }

        cmd.arg("--tool-id").arg(tool_id.as_str());

        Self::setup_process(cmd, tool_id, env_vars).await
    }

    async fn spawn_python_process(
        tool_id: &ToolId,
        config: &ToolConfiguration,
        env_vars: Option<&HashMap<String, String>>,
    ) -> MCPResult<Self> {
        info!("Spawning Python process for tool ID: {}", tool_id);

        let mut cmd = Command::new(&config.command);

        if let Some(args) = &config.args {
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
    ) -> MCPResult<Self> {
        info!("Spawning Docker process for tool ID: {}", tool_id);

        let mut cmd = Command::new("docker");
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
    ) -> MCPResult<Self> {
        use std::process::Stdio;

        if let Some(vars) = env_vars {
            for (key, value) in vars {
                info!("Setting environment variable: {}={}", key, value);
                cmd.env(key, value);
            }
        }

        cmd.stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        info!(
            "Spawning process: {:?} with args: {:?}",
            cmd.as_std().get_program(),
            cmd.as_std().get_args().collect::<Vec<_>>()
        );

        let mut child = cmd
            .spawn()
            .map_err(|e| MCPError::ProcessError(e.to_string()))?;

        // Handle stderr in a separate task
        if let Some(stderr) = child.stderr.take() {
            let tool_id = tool_id.clone();
            tokio::spawn(async move {
                let mut stderr_reader = tokio::io::BufReader::new(stderr);
                let mut line = String::new();
                while let Ok(bytes_read) = stderr_reader.read_line(&mut line).await {
                    if bytes_read == 0 {
                        break;
                    }
                    error!("[{} stderr]: {}", tool_id, line.trim());
                    line.clear();
                }
            });
        }

        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| MCPError::ProcessError("Failed to open stdin".to_string()))?;

        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| MCPError::ProcessError("Failed to open stdout".to_string()))?;

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