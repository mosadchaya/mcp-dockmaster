use std::collections::HashMap;
use async_trait::async_trait;
use log::info;
use serde_json::{json, Value};
use tokio::process::{Child, ChildStdin, ChildStdout};

use crate::domain::traits::ProcessManager;
use crate::domain::errors::DomainError;

/// A mock process manager for testing
pub struct MockProcessManager;

impl MockProcessManager {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl ProcessManager for MockProcessManager {
    /// Mock implementation that doesn't actually spawn a process
    async fn spawn_process(
        &self,
        _configuration: &Value,
        _tool_id: &str,
        _tool_type: &str,
        _env_vars: Option<&HashMap<String, String>>,
    ) -> Result<(Child, ChildStdin, ChildStdout), DomainError> {
        // In a real test, we would return a mock process
        // For now, we'll just return an error since we don't need this for our tests
        Err(DomainError::ProcessError("Mock process manager doesn't actually spawn processes".to_string()))
    }
    
    /// Mock implementation that doesn't actually kill a process
    async fn kill_process(&self, _process: &mut Child) -> Result<(), DomainError> {
        // In a real test, we would kill the mock process
        // For now, we'll just return Ok since we don't need this for our tests
        Ok(())
    }
    
    /// Mock implementation that returns predefined tools
    async fn discover_server_tools(
        &self,
        server_id: &str,
        _process_ios: &mut HashMap<String, (ChildStdin, ChildStdout)>,
    ) -> Result<Vec<Value>, DomainError> {
        info!("Mock discover_server_tools called for server: {}", server_id);
        
        // Return a predefined list of tools
        let tools = vec![
            json!({
                "id": "hello_world",
                "name": "hello_world",
                "description": "A simple hello world tool"
            }),
            json!({
                "id": "hello_world_with_input",
                "name": "hello_world_with_input",
                "description": "A hello world tool that takes input"
            }),
            json!({
                "id": "hello_world_with_config",
                "name": "hello_world_with_config",
                "description": "A hello world tool that uses configuration"
            })
        ];
        
        Ok(tools)
    }
    
    /// Mock implementation that returns predefined results
    async fn execute_server_tool(
        &self,
        _server_id: &str,
        tool_id: &str,
        parameters: Value,
        _process_ios: &mut HashMap<String, (ChildStdin, ChildStdout)>,
    ) -> Result<Value, DomainError> {
        info!("Mock execute_server_tool called for tool: {}", tool_id);
        
        // Return different results based on the tool ID
        match tool_id {
            "hello_world" => {
                Ok(json!({
                    "content": [
                        {
                            "text": "hello world"
                        }
                    ]
                }))
            },
            "hello_world_with_input" => {
                // Extract the message parameter
                let message = parameters.get("message")
                    .and_then(|m| m.as_str())
                    .unwrap_or("default message");
                
                Ok(json!({
                    "content": [
                        {
                            "text": format!("hello world {}", message)
                        }
                    ]
                }))
            },
            "hello_world_with_config" => {
                // Extract the config parameter
                let config = parameters.get("config")
                    .and_then(|c| c.as_str())
                    .unwrap_or("default config");
                
                Ok(json!({
                    "content": [
                        {
                            "text": format!("hello configuration {}", config)
                        }
                    ]
                }))
            },
            _ => {
                Err(DomainError::ToolNotFound(format!("Unknown tool: {}", tool_id)))
            }
        }
    }
    
    /// Mock implementation that doesn't actually kill processes
    async fn kill_all_processes(&self) -> Result<(), DomainError> {
        // In a real test, we would kill all mock processes
        // For now, we'll just return Ok since we don't need this for our tests
        Ok(())
    }
}
