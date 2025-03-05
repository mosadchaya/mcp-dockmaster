use crate::database;
use crate::error::{MCPError, MCPResult};
use crate::models::{ToolId, ToolMetadata, ToolRegistrationRequest, ToolRegistrationResponse, ToolType, ToolConfiguration};
use crate::process::ProcessManager;
use log::{error, info};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::Duration;

#[derive(Default)]
pub struct ToolRegistry {
    pub tools: HashMap<ToolId, ToolMetadata>,
    pub processes: HashMap<ToolId, Option<ProcessManager>>,
    pub server_tools: HashMap<ToolId, Vec<Value>>,
}

#[derive(Clone, Default)]
pub struct MCPState {
    pub tool_registry: Arc<RwLock<ToolRegistry>>,
}

impl ToolRegistry {
    pub async fn register_tool(
        &mut self,
        request: ToolRegistrationRequest,
    ) -> MCPResult<ToolRegistrationResponse> {
        info!("Starting registration of tool: {}", request.tool_name);

        // Generate tool ID
        let tool_id = ToolId::new(format!("tool_{}", self.tools.len() + 1));

        // Create tool metadata
        let metadata = ToolMetadata {
            name: request.tool_name.clone(),
            description: request.description.clone(),
            tool_type: request.tool_type.clone(),
            enabled: true,
            configuration: request.configuration.clone(),
            process_running: false,
            tool_count: 0,
        };

        // Store the tool metadata
        self.tools.insert(tool_id.clone(), metadata);

        // Create empty tools list
        self.server_tools.insert(tool_id.clone(), Vec::new());

        // Extract environment variables if they exist
        let env_vars = if let Some(auth) = &request.authentication {
            if let Some(env) = auth.get("env") {
                let mut env_map = HashMap::new();
                if let Some(env_obj) = env.as_object() {
                    for (key, value) in env_obj {
                        if let Some(value_str) = value.as_str() {
                            env_map.insert(key.clone(), value_str.to_string());
                        }
                    }
                }
                Some(env_map)
            } else {
                None
            }
        } else {
            None
        };

        // Get the configuration
        let config = request
            .configuration
            .ok_or_else(|| MCPError::ConfigurationError("Configuration is required".to_string()))?;

        // Spawn the process
        let process_manager =
            ProcessManager::new(&tool_id, &request.tool_type, &config, env_vars.as_ref()).await?;

        // Store the process
        self.processes
            .insert(tool_id.clone(), Some(process_manager));

        // Wait for server to initialize
        tokio::time::sleep(Duration::from_secs(2)).await;

        // Try to discover tools
        match self.discover_tools(&tool_id).await {
            Ok(tools) => {
                self.server_tools.insert(tool_id.clone(), tools);
            }
            Err(e) => {
                error!("Error discovering tools: {}", e);
                // Add default tool
                self.server_tools.insert(
                    tool_id.clone(),
                    vec![json!({
                        "id": "main",
                        "name": request.tool_name,
                        "description": request.description
                    })],
                );
            }
        }

        Ok(ToolRegistrationResponse {
            success: true,
            message: format!("Tool registered successfully"),
            tool_id: Some(tool_id),
        })
    }
    
    /// Execute a tool on a server
    pub async fn execute_tool(
        &mut self,
        server_id: &ToolId,
        tool_id: &str,
        parameters: Value,
    ) -> MCPResult<Value> {
        let process = self
            .processes
            .get_mut(server_id)
            .ok_or_else(|| MCPError::ServerNotFound(server_id.to_string()))?
            .as_mut()
            .ok_or_else(|| MCPError::ServerNotFound(server_id.to_string()))?;

        process.send_command("tools/call", json!({
            "name": tool_id,
            "arguments": parameters
        })).await
    }

    pub async fn discover_tools(&mut self, tool_id: &ToolId) -> MCPResult<Vec<Value>> {
        let process = self
            .processes
            .get_mut(tool_id)
            .ok_or_else(|| MCPError::ServerNotFound(tool_id.to_string()))?
            .as_mut()
            .ok_or_else(|| MCPError::ServerNotFound(tool_id.to_string()))?;

        let result = process.send_command("tools/list", json!({})).await?;

        if let Some(tools) = result.as_array() {
            Ok(tools.clone())
        } else {
            Ok(Vec::new())
        }
    }

    pub async fn restart_all_tools(&mut self) -> MCPResult<Vec<MCPResult<()>>> {
        // Collect tool IDs first
        let tool_ids: Vec<_> = self.tools.keys().cloned().collect();

        // Process each tool sequentially to avoid borrow checker issues
        let mut results = Vec::new();
        for id in tool_ids {
            let result = tokio::time::timeout(Duration::from_secs(30), self.restart_tool(&id))
                .await
                .unwrap_or_else(|_| Err(MCPError::TimeoutError(id.to_string())));
            results.push(result);
        }

        Ok(results)
    }

    pub async fn restart_tool(&mut self, tool_id: &ToolId) -> MCPResult<()> {
        // Kill existing process if running
        if let Some(Some(process)) = self.processes.get_mut(tool_id) {
            process.kill().await?;
        }

        // Get tool metadata
        let metadata = self
            .tools
            .get(tool_id)
            .ok_or_else(|| MCPError::ToolError(format!("Tool not found: {}", tool_id)))?;

        // Skip if disabled
        if !metadata.enabled {
            return Ok(());
        }

        // Get configuration
        let config = metadata
            .configuration
            .as_ref()
            .ok_or_else(|| MCPError::ConfigurationError("Configuration missing".to_string()))?;

        // Spawn new process
        let process_manager =
            ProcessManager::new(tool_id, &metadata.tool_type, config, config.env.as_ref()).await?;

        // Store new process
        self.processes
            .insert(tool_id.clone(), Some(process_manager));

        // Wait for initialization
        tokio::time::sleep(Duration::from_secs(2)).await;

        // Rediscover tools
        match self.discover_tools(tool_id).await {
            Ok(tools) => {
                self.server_tools.insert(tool_id.clone(), tools);
            }
            Err(e) => {
                error!("Error rediscovering tools: {}", e);
            }
        }

        Ok(())
    }

    pub async fn kill_all_processes(&mut self) -> MCPResult<()> {
        for (tool_id, process_opt) in self.processes.iter_mut() {
            if let Some(process) = process_opt {
                if let Err(e) = process.kill().await {
                    error!("Failed to kill process for tool {}: {}", tool_id, e);
                }
            }
        }
        Ok(())
    }

    /// Initialize the MCP state from the database
    pub async fn init_mcp_state() -> MCPState {
        let mcp_state = MCPState::default();

        let db_manager = match database::DatabaseManager::new() {
            Ok(manager) => manager,
            Err(e) => {
                error!("Failed to initialize database: {}", e);
                return mcp_state;
            }
        };

        // Load the tool registry
        match db_manager.load_tool_registry() {
            Ok(registry_data) => {
                // Explicitly close the database connection
                let _ = db_manager.close();

                // Update the mcp_state.tool_registry with the loaded data
                let mut registry = mcp_state.tool_registry.write().await;
                
                // Convert the loaded data to the new format
                for (tool_id_str, tool_data) in registry_data.tools {
                    let tool_id = ToolId::new(tool_id_str.to_string());
                    
                    // Extract data from tool_data JSON
                    let name = tool_data.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let description = tool_data.get("description").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let tool_type_str = tool_data.get("tool_type").and_then(|v| v.as_str()).unwrap_or("node").to_string();
                    let tool_type = match tool_type_str.as_str() {
                        "python" => ToolType::Python,
                        "docker" => ToolType::Docker,
                        _ => ToolType::Node,
                    };
                    let enabled = tool_data.get("enabled").and_then(|v| v.as_bool()).unwrap_or(false);
                    
                    // Create configuration
                    let mut configuration = None;
                    if let Some(config) = tool_data.get("configuration") {
                        if let Some(command) = config.get("command").and_then(|v| v.as_str()) {
                            let mut args = None;
                            if let Some(args_array) = config.get("args").and_then(|v| v.as_array()) {
                                let args_vec: Vec<String> = args_array.iter()
                                    .filter_map(|a| a.as_str().map(|s| s.to_string()))
                                    .collect();
                                if !args_vec.is_empty() {
                                    args = Some(args_vec);
                                }
                            }
                            
                            let mut env = None;
                            if let Some(env_obj) = config.get("env").and_then(|v| v.as_object()) {
                                let mut env_map = std::collections::HashMap::new();
                                for (key, value) in env_obj {
                                    if let Some(value_str) = value.as_str() {
                                        env_map.insert(key.clone(), value_str.to_string());
                                    }
                                }
                                if !env_map.is_empty() {
                                    env = Some(env_map);
                                }
                            }
                            
                            configuration = Some(ToolConfiguration {
                                command: command.to_string(),
                                args,
                                env,
                            });
                        }
                    }
                    
                    // Create metadata
                    let metadata = ToolMetadata {
                        name,
                        description,
                        tool_type,
                        enabled,
                        configuration,
                        process_running: false,
                        tool_count: 0,
                    };
                    
                    // Add to registry
                    registry.tools.insert(tool_id.clone(), metadata);
                }
                
                // Add server tools
                for (server_id_str, tools) in registry_data.server_tools {
                    let server_id = ToolId::new(server_id_str.to_string());
                    registry.server_tools.insert(server_id, tools);
                }
                
                info!("Successfully loaded MCP state from database");

                // Restart enabled tools
                let tools_to_restart: Vec<ToolId> = registry.tools.iter()
                    .filter_map(|(tool_id, metadata)| {
                        if metadata.enabled {
                            Some(tool_id.clone())
                        } else {
                            None
                        }
                    })
                    .collect();
                
                drop(registry); // Release the lock before restarting tools
                
                for tool_id in tools_to_restart {
                    let mut registry = mcp_state.tool_registry.write().await;
                    let _ = registry.restart_tool(&tool_id).await;
                }
            }
            Err(e) => {
                // Explicitly close the database connection
                let _ = db_manager.close();

                error!("Failed to load MCP state from database: {}", e);
            }
        }

        mcp_state
    }

    /// Save the current MCP state to the database
    pub async fn save_mcp_state(mcp_state: &MCPState) -> Result<(), String> {
        // First, get a clone of the registry data to avoid holding the lock for too long
        let registry_data = {
            let registry = mcp_state.tool_registry.read().await;

            // Create a temporary registry with the data in the old format
            let mut old_registry = crate::mcp_proxy::LegacyToolRegistry::default();
            
            // Convert tools to the old format
            for (tool_id, metadata) in &registry.tools {
                let mut tool_data = json!({
                    "name": metadata.name,
                    "description": metadata.description,
                    "enabled": metadata.enabled,
                    "tool_type": match metadata.tool_type {
                        ToolType::Node => "node",
                        ToolType::Python => "python",
                        ToolType::Docker => "docker",
                    },
                });
                
                // Add configuration if available
                if let Some(config) = &metadata.configuration {
                    let mut config_json = json!({
                        "command": config.command,
                    });
                    
                    if let Some(args) = &config.args {
                        config_json["args"] = json!(args);
                    }
                    
                    if let Some(env) = &config.env {
                        config_json["env"] = json!(env);
                    }
                    
                    tool_data["configuration"] = config_json;
                }
                
                old_registry.tools.insert(tool_id.to_string(), tool_data);
            }
            
            // Copy server tools
            for (server_id, tools) in &registry.server_tools {
                old_registry.server_tools.insert(server_id.to_string(), tools.clone());
            }
            
            old_registry
        };

        // Now save the cloned data to the database
        match database::DatabaseManager::new() {
            Ok(mut db_manager) => {
                let result = db_manager.save_tool_registry(&registry_data);

                // Explicitly close the database connection
                let _ = db_manager.close();

                match result {
                    Ok(_) => {
                        info!("Successfully saved MCP state to database");
                        Ok(())
                    }
                    Err(e) => {
                        error!("Failed to save tool registry: {}", e);
                        Err(e)
                    }
                }
            }
            Err(e) => {
                error!("Failed to initialize database for saving: {}", e);
                Err(e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ToolConfiguration, ToolType};

    use super::*;

    #[tokio::test]
    async fn test_tool_registration() {
        let mut registry = ToolRegistry::default();
        let request = ToolRegistrationRequest {
            tool_name: "test_tool".to_string(),
            description: "Test tool".to_string(),
            tool_type: ToolType::Node,
            authentication: None,
            configuration: Some(ToolConfiguration {
                command: "node".to_string(),
                args: Some(vec!["test.js".to_string()]),
                env: None,
            }),
            distribution: None,
        };

        let result = registry.register_tool(request).await;
        assert!(result.is_ok());
    }

    // Add more tests...
}
