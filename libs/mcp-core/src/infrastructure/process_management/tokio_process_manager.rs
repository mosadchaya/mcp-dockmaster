use std::collections::HashMap;
use async_trait::async_trait;
use log::{info, error};
use serde_json::{json, Value};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    process::{Child, ChildStdin, ChildStdout, Command},
    time::{Duration, timeout},
};

use crate::domain::traits::ProcessManager;
use crate::domain::errors::DomainError;
use crate::dm_process::DMProcess;

pub struct TokioProcessManager;

impl TokioProcessManager {
    pub fn new() -> Self {
        Self {}
    }
    
    /// Helper method to write a command to a process stdin
    async fn write_command(
        &self,
        stdin: &mut ChildStdin,
        command: &str,
    ) -> Result<(), DomainError> {
        stdin
            .write_all(format!("{}\n", command).as_bytes())
            .await
            .map_err(|e| DomainError::StdinWriteError(e.to_string()))?;
        
        stdin
            .flush()
            .await
            .map_err(|e| DomainError::StdinFlushError(e.to_string()))?;
        
        Ok(())
    }
    
    /// Helper method to read a response from a process stdout
    async fn read_response(
        &self,
        stdout: &mut BufReader<&mut ChildStdout>,
        server_id: &str,
    ) -> Result<String, DomainError> {
        let mut response = String::new();
        
        match timeout(Duration::from_secs(10), stdout.read_line(&mut response)).await {
            Ok(result) => {
                result.map_err(|e| DomainError::StdoutReadError(e.to_string()))?;
                
                if response.is_empty() {
                    return Err(DomainError::ServerClosedConnection);
                }
                
                Ok(response)
            },
            Err(_) => Err(DomainError::TimeoutError(server_id.to_string())),
        }
    }
}

#[async_trait]
impl ProcessManager for TokioProcessManager {
    /// Spawn a new process
    async fn spawn_process(
        &self,
        configuration: &Value,
        tool_id: &str,
        _tool_type: &str,
        env_vars: Option<&HashMap<String, String>>,
    ) -> Result<(Child, ChildStdin, ChildStdout), DomainError> {
        info!("Spawning process for tool: {}", tool_id);
        
        let command = configuration
            .get("command")
            .and_then(|c| c.as_str())
            .ok_or_else(|| DomainError::InvalidToolConfiguration("Missing command".to_string()))?;
        
        let args = configuration
            .get("args")
            .and_then(|a| a.as_array())
            .map(|args| {
                args.iter()
                    .filter_map(|a| a.as_str())
                    .collect::<Vec<&str>>()
            })
            .unwrap_or_default();
        
        let mut cmd = Command::new(command);
        cmd.args(args);
        
        // Add environment variables if provided
        if let Some(env) = env_vars {
            for (key, value) in env {
                cmd.env(key, value);
            }
        }
        
        // Configure stdin and stdout
        cmd.stdin(std::process::Stdio::piped());
        cmd.stdout(std::process::Stdio::piped());
        
        // Spawn the process
        let mut child = cmd
            .spawn()
            .map_err(|e| DomainError::ProcessError(format!("Failed to spawn process: {}", e)))?;
        
        // Get stdin and stdout handles
        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| DomainError::ProcessError("Failed to open stdin".to_string()))?;
        
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| DomainError::ProcessError("Failed to open stdout".to_string()))?;
        
        Ok((child, stdin, stdout))
    }
    
    /// Kill a running process
    async fn kill_process(&self, process: &mut Child) -> Result<(), DomainError> {
        process
            .kill()
            .await
            .map_err(|e| DomainError::ProcessError(format!("Failed to kill process: {}", e)))
    }
    
    /// Discover tools from a server
    async fn discover_server_tools(
        &self,
        server_id: &str,
        process_ios: &mut HashMap<String, (ChildStdin, ChildStdout)>,
    ) -> Result<Vec<Value>, DomainError> {
        // Get the process I/O for the server
        let io_pair = process_ios
            .get_mut(server_id)
            .ok_or_else(|| DomainError::ServerNotFound(server_id.to_string()))?;
        
        let (stdin, stdout) = &mut *io_pair;
        
        // Create a buffered reader for stdout
        let mut stdout_reader = BufReader::new(&mut *stdout);
        
        // Send the discover command
        self.write_command(stdin, "discover").await?;
        
        // Read the response
        let response = self.read_response(&mut stdout_reader, server_id).await?;
        
        // Parse the response as JSON
        let json_response: Value = serde_json::from_str(&response)
            .map_err(|e| DomainError::JsonParseError(e.to_string()))?;
        
        // Extract the tools array
        let tools = json_response
            .get("tools")
            .and_then(|t| t.as_array())
            .map(|a| a.clone())
            .unwrap_or_default();
        
        Ok(tools)
    }
    
    /// Execute a tool on a server
    async fn execute_server_tool(
        &self,
        server_id: &str,
        tool_id: &str,
        parameters: Value,
        process_ios: &mut HashMap<String, (ChildStdin, ChildStdout)>,
    ) -> Result<Value, DomainError> {
        // Get the process I/O for the server
        let io_pair = process_ios
            .get_mut(server_id)
            .ok_or_else(|| DomainError::ServerNotFound(server_id.to_string()))?;
        
        let (stdin, stdout) = &mut *io_pair;
        
        // Create a buffered reader for stdout
        let mut stdout_reader = BufReader::new(&mut *stdout);
        
        // Create the execute command
        let execute_command = json!({
            "command": "execute",
            "tool_id": tool_id,
            "parameters": parameters
        });
        
        // Send the execute command
        self.write_command(stdin, &execute_command.to_string()).await?;
        
        // Read the response
        let response = self.read_response(&mut stdout_reader, server_id).await?;
        
        // Parse the response as JSON
        let json_response: Value = serde_json::from_str(&response)
            .map_err(|e| DomainError::JsonParseError(e.to_string()))?;
        
        // Extract the result
        let result = json_response
            .get("result")
            .ok_or_else(|| DomainError::NoResultField)?
            .clone();
        
        Ok(result)
    }
    
    /// Kill all processes managed by this process manager
    async fn kill_all_processes(&self) -> Result<(), DomainError> {
        // In a real implementation, we would track all spawned processes
        // and kill them here. For now, we'll just return Ok.
        Ok(())
    }
}
