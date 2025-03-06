use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use log::{info, error};
use serde_json::Value;
use tokio::sync::RwLock;

use crate::domain::traits::ToolRepository;
use crate::domain::entities::tool::{Tool, ToolExecutionRequest, ToolExecutionResponse};
use crate::domain::errors::DomainError;
use crate::database::DBManager;
use crate::registry::ToolRegistry;
use crate::domain::traits::ProcessManager;

/// Repository implementation that uses SQLite for persistence and ToolRegistry for in-memory storage
pub struct SqliteToolRepository {
    db_manager: DBManager,
    registry: Arc<RwLock<ToolRegistry>>,
    process_manager: Arc<dyn ProcessManager>,
}

impl SqliteToolRepository {
    /// Create a new SqliteToolRepository with the given dependencies
    pub fn new(db_manager: DBManager, registry: Arc<RwLock<ToolRegistry>>, process_manager: Arc<dyn ProcessManager>) -> Self {
        Self { 
            db_manager,
            registry,
            process_manager,
        }
    }
    
    /// Create a new SqliteToolRepository without a DBManager (for testing or when DB access is not needed)
    pub fn new_without_db(registry: Arc<RwLock<ToolRegistry>>, process_manager: Arc<dyn ProcessManager>) -> Self {
        Self { 
            db_manager: DBManager::new().expect("Failed to create DBManager"),
            registry,
            process_manager,
        }
    }
}

#[async_trait]
impl ToolRepository for SqliteToolRepository {
    async fn get_tool(&self, tool_id: &str) -> Result<Tool, DomainError> {
        let registry = self.registry.read().await;
        let model_tool = registry.get_tool(tool_id)
            .map_err(|e| DomainError::ToolNotFound(format!("{}: {}", tool_id, e)))?;
        
        // Convert from models::models::Tool to domain::entities::tool::Tool
        Ok(Tool {
            name: model_tool.name,
            description: model_tool.description,
            enabled: model_tool.enabled,
            tool_type: model_tool.tool_type,
            entry_point: model_tool.entry_point,
            configuration: model_tool.configuration.map(|c| crate::domain::entities::tool::ToolConfiguration {
                command: c.command,
                args: c.args,
            }),
            distribution: model_tool.distribution,
            config: model_tool.config.map(|c| crate::domain::entities::tool::ToolConfig {
                env: c.env,
                command: c.command,
                args: c.args,
            }),
            authentication: model_tool.authentication,
        })
    }
    
    async fn get_all_tools(&self) -> Result<HashMap<String, Tool>, DomainError> {
        let registry = self.registry.read().await;
        let model_tools = registry.get_all_tools()
            .map_err(|e| DomainError::RepositoryError(e))?;
        
        // Convert from HashMap<String, models::models::Tool> to HashMap<String, domain::entities::tool::Tool>
        let mut domain_tools = HashMap::new();
        for (id, model_tool) in model_tools {
            domain_tools.insert(id, Tool {
                name: model_tool.name,
                description: model_tool.description,
                enabled: model_tool.enabled,
                tool_type: model_tool.tool_type,
                entry_point: model_tool.entry_point,
                configuration: model_tool.configuration.map(|c| crate::domain::entities::tool::ToolConfiguration {
                    command: c.command,
                    args: c.args,
                }),
                distribution: model_tool.distribution,
                config: model_tool.config.map(|c| crate::domain::entities::tool::ToolConfig {
                    env: c.env,
                    command: c.command,
                    args: c.args,
                }),
                authentication: model_tool.authentication,
            });
        }
        
        Ok(domain_tools)
    }
    
    async fn save_tool(&self, tool_id: &str, tool: &Tool) -> Result<(), DomainError> {
        let registry = self.registry.read().await;
        
        // Convert from domain::entities::tool::Tool to models::models::Tool
        let model_tool = crate::models::models::Tool {
            name: tool.name.clone(),
            description: tool.description.clone(),
            enabled: tool.enabled,
            tool_type: tool.tool_type.clone(),
            entry_point: tool.entry_point.clone(),
            configuration: tool.configuration.as_ref().map(|c| crate::models::models::ToolConfiguration {
                command: c.command.clone(),
                args: c.args.clone(),
            }),
            distribution: tool.distribution.clone(),
            config: tool.config.as_ref().map(|c| crate::models::models::ToolConfig {
                env: c.env.clone(),
                command: c.command.clone(),
                args: c.args.clone(),
            }),
            authentication: tool.authentication.clone(),
        };
        
        registry.save_tool(tool_id, &model_tool)
            .map_err(|e| DomainError::RepositoryError(e))
    }
    
    async fn delete_tool(&self, tool_id: &str) -> Result<(), DomainError> {
        let registry = self.registry.read().await;
        registry.delete_tool(tool_id)
            .map_err(|e| DomainError::RepositoryError(e))
    }
    
    async fn restart_tool(&self, tool_id: &str) -> Result<(), DomainError> {
        let mut registry = self.registry.write().await;
        registry.restart_tool(tool_id).await
            .map_err(|e| DomainError::RepositoryError(e))
    }
    
    async fn execute_tool(&self, request: ToolExecutionRequest) -> Result<ToolExecutionResponse, DomainError> {
        info!("Executing tool with ID: {}", request.tool_id);
        
        // Extract server_id and tool_id from the proxy_id if it contains a colon
        let parts: Vec<&str> = request.tool_id.split(':').collect();
        
        // Create a new HashMap for process_ios since we can't clone ChildStdin/ChildStdout
        let mut process_ios = HashMap::new();
        
        if parts.len() == 2 {
            // This is a server:tool format
            let server_id = parts[0];
            let tool_id = parts[1];
            
            info!("Executing server tool: {} from server: {}", tool_id, server_id);
            
            // Execute the tool on the server
            let _registry = self.registry.write().await;
            
            match self.process_manager.execute_server_tool(
                server_id,
                tool_id,
                request.parameters.clone(),
                &mut process_ios,
            ).await {
                Ok(result) => Ok(ToolExecutionResponse {
                    success: true,
                    result: Some(result),
                    error: None,
                }),
                Err(e) => {
                    error!("Error executing server tool: {}", e);
                    Ok(ToolExecutionResponse {
                        success: false,
                        result: None,
                        error: Some(format!("Error executing server tool: {}", e)),
                    })
                }
            }
        } else {
            // This is a regular tool execution
            // Extract the tool name from parameters if available
            let tool_name = request.parameters.get("tool_name").and_then(|v| v.as_str());
            
            if let Some(tool_name) = tool_name {
                info!("Executing tool: {} with ID: {}", tool_name, request.tool_id);
                
                // Execute the tool on the server
                let _registry = self.registry.write().await;
                
                match self.process_manager.execute_server_tool(
                    &request.tool_id,
                    tool_name,
                    request.parameters.clone(),
                    &mut process_ios,
                ).await {
                    Ok(result) => Ok(ToolExecutionResponse {
                        success: true,
                        result: Some(result),
                        error: None,
                    }),
                    Err(e) => {
                        error!("Error executing tool: {}", e);
                        Ok(ToolExecutionResponse {
                            success: false,
                            result: None,
                            error: Some(format!("Error executing tool: {}", e)),
                        })
                    }
                }
            } else {
                // No tool_name provided, return an error
                error!("No tool_name provided in parameters");
                Ok(ToolExecutionResponse {
                    success: false,
                    result: None,
                    error: Some("No tool_name provided in parameters".to_string()),
                })
            }
        }
    }
    
    async fn get_server_tools(&self) -> Result<HashMap<String, Vec<Value>>, DomainError> {
        let registry = self.registry.read().await;
        Ok(registry.server_tools.clone())
    }
}
