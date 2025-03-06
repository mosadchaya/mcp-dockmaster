use std::collections::HashMap;
use async_trait::async_trait;
use serde_json::Value;
use tokio::process::{Child, ChildStdin, ChildStdout};

use crate::domain::entities::{Tool, ToolExecutionRequest, ToolExecutionResponse};
use crate::domain::errors::DomainError;

/// Repository for managing tools
#[async_trait]
pub trait ToolRepository: Send + Sync {
    /// Get a tool by ID
    async fn get_tool(&self, tool_id: &str) -> Result<Tool, DomainError>;
    
    /// Get all tools
    async fn get_all_tools(&self) -> Result<HashMap<String, Tool>, DomainError>;
    
    /// Save or update a tool
    async fn save_tool(&self, tool_id: &str, tool: &Tool) -> Result<(), DomainError>;
    
    /// Delete a tool
    async fn delete_tool(&self, tool_id: &str) -> Result<(), DomainError>;
    
    /// Restart a tool
    async fn restart_tool(&self, tool_id: &str) -> Result<(), DomainError>;
    
    /// Execute a tool
    async fn execute_tool(&self, request: ToolExecutionRequest) -> Result<ToolExecutionResponse, DomainError>;
    
    /// Get all server tools
    async fn get_server_tools(&self) -> Result<HashMap<String, Vec<Value>>, DomainError>;
}

/// Manager for tool processes
#[async_trait]
pub trait ProcessManager: Send + Sync {
    /// Spawn a new process
    async fn spawn_process(
        &self,
        configuration: &Value,
        tool_id: &str,
        tool_type: &str,
        env_vars: Option<&HashMap<String, String>>,
    ) -> Result<(Child, ChildStdin, ChildStdout), DomainError>;
    
    /// Kill a running process
    async fn kill_process(&self, process: &mut Child) -> Result<(), DomainError>;
    
    /// Discover tools from a server
    async fn discover_server_tools(
        &self,
        server_id: &str,
        process_ios: &mut HashMap<String, (ChildStdin, ChildStdout)>,
    ) -> Result<Vec<Value>, DomainError>;
    
    /// Execute a tool on a server
    async fn execute_server_tool(
        &self,
        server_id: &str,
        tool_id: &str,
        parameters: Value,
        process_ios: &mut HashMap<String, (ChildStdin, ChildStdout)>,
    ) -> Result<Value, DomainError>;
    
    /// Kill all processes managed by this process manager
    async fn kill_all_processes(&self) -> Result<(), DomainError> {
        // Default implementation does nothing
        Ok(())
    }
}
