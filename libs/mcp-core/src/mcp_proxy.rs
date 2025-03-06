use crate::models::error::MCPError;
use log::info;
use serde_json::Value;

// This file is kept for backward compatibility
// All process management logic has been moved to infrastructure/process_management/tokio_process_manager.rs
// All domain logic has been moved to domain/services.rs and application/services/tool_service.rs

// Re-export functions from infrastructure/process_management/tokio_process_manager.rs
// for backward compatibility
pub use crate::infrastructure::process_management::TokioProcessManager;
pub use crate::domain::traits::ProcessManager;

// Deprecated: Use ProcessManager::discover_server_tools instead
pub async fn discover_server_tools(
    server_id: &str,
    registry: &mut crate::registry::ToolRegistry,
) -> Result<Vec<Value>, String> {
    info!("discover_server_tools is deprecated, use ProcessManager::discover_server_tools instead");
    
    // Create a process manager
    let process_manager = TokioProcessManager::new();
    
    // Call the process manager's discover_server_tools method
    process_manager.discover_server_tools(server_id, &mut registry.process_ios)
        .await
        .map_err(|e| e.to_string())
}

// Deprecated: Use ProcessManager::execute_server_tool instead
pub async fn execute_server_tool(
    server_id: &str,
    tool_name: &str,
    parameters: Value,
    registry: &mut crate::registry::ToolRegistry,
) -> Result<Value, MCPError> {
    info!("execute_server_tool is deprecated, use ProcessManager::execute_server_tool instead");
    
    // Create a process manager
    let process_manager = TokioProcessManager::new();
    
    // Call the process manager's execute_server_tool method
    process_manager.execute_server_tool(server_id, tool_name, parameters, &mut registry.process_ios)
        .await
        .map_err(|e| match e {
            crate::domain::errors::DomainError::ServerNotFound(s) => MCPError::ServerNotFound(s),
            crate::domain::errors::DomainError::SerializationError(s) => MCPError::SerializationError(s),
            crate::domain::errors::DomainError::StdinWriteError(s) => MCPError::StdinWriteError(s),
            crate::domain::errors::DomainError::StdinFlushError(s) => MCPError::StdinFlushError(s),
            crate::domain::errors::DomainError::StdoutReadError(s) => MCPError::StdoutReadError(s),
            crate::domain::errors::DomainError::TimeoutError(s) => MCPError::TimeoutError(s),
            crate::domain::errors::DomainError::ServerClosedConnection => MCPError::ServerClosedConnection,
            crate::domain::errors::DomainError::NoResponse => MCPError::NoResponse,
            crate::domain::errors::DomainError::JsonParseError(s) => MCPError::JsonParseError(s),
            crate::domain::errors::DomainError::ToolExecutionError(s) => MCPError::ToolExecutionError(s),
            crate::domain::errors::DomainError::NoResultField => MCPError::NoResultField,
            _ => MCPError::UnknownError(format!("{:?}", e)),
        })
}

// Deprecated: Use ProcessManager::spawn_process instead
pub async fn spawn_process(
    configuration: &Value,
    tool_id: &str,
    tool_type: &str,
    env_vars: Option<&std::collections::HashMap<String, String>>,
) -> Result<
    (
        tokio::process::Child,
        tokio::process::ChildStdin,
        tokio::process::ChildStdout,
    ),
    String,
> {
    info!("spawn_process is deprecated, use ProcessManager::spawn_process instead");
    
    // Create a process manager
    let process_manager = TokioProcessManager::new();
    
    // Call the process manager's spawn_process method
    process_manager.spawn_process(configuration, tool_id, tool_type, env_vars)
        .await
        .map_err(|e| e.to_string())
}

// Deprecated: Use ProcessManager::kill_process instead
pub async fn kill_process(process: &mut tokio::process::Child) -> Result<(), String> {
    info!("kill_process is deprecated, use ProcessManager::kill_process instead");
    
    // Create a process manager
    let process_manager = TokioProcessManager::new();
    
    // Call the process manager's kill_process method
    process_manager.kill_process(process)
        .await
        .map_err(|e| e.to_string())
}
