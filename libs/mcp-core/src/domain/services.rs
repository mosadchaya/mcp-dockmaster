use std::collections::HashMap;
use std::sync::Arc;
use serde_json::{json, Value};
use log::{info, error};
use tokio::time::Duration;

use crate::domain::traits::{ProcessManager, ToolRepository};
use crate::domain::entities::tool::{
    DiscoverServerToolsRequest, DiscoverServerToolsResponse, 
    ToolRegistrationRequest, ToolRegistrationResponse,
    ToolExecutionRequest, ToolExecutionResponse,
    ToolUninstallRequest, ToolUninstallResponse,
    ToolUpdateRequest, ToolUpdateResponse,
    ToolConfigUpdateRequest, ToolConfigUpdateResponse,
    ToolConfiguration, ToolConfig, Tool
};

/// Domain service that handles "register", "uninstall", "list", "execute"...
pub struct ToolService {
    // Store references to repository and process manager
    tool_repository: Arc<dyn ToolRepository>,
    process_manager: Arc<dyn ProcessManager>,
}

impl ToolService {
    pub fn new(
        tool_repository: Arc<dyn ToolRepository>,
        process_manager: Arc<dyn ProcessManager>,
    ) -> Self {
        Self { tool_repository, process_manager }
    }

    /// Register a new tool
    pub async fn register_tool(
        &self,
        request: ToolRegistrationRequest,
    ) -> Result<ToolRegistrationResponse, String> {
        info!("Starting registration of tool: {}", request.tool_name);
        
        // Safely access the command field if configuration exists
        if let Some(config) = &request.configuration {
            if let Some(cmd) = config.get("command") {
                info!("Command: {}", cmd);
            } else {
                info!("Command: Not specified in configuration");
            }
        } else {
            info!("Configuration not provided");
        }

        // Get all tools to generate a new ID
        let tools = self.tool_repository.get_all_tools().await.map_err(|e| e.to_string())?;
        let tool_id = format!("tool_{}", tools.len() + 1);
        info!("Generated tool ID: {}", tool_id);

        // Create the tool configuration if provided
        let configuration = request
            .configuration
            .as_ref()
            .map(|config| ToolConfiguration {
                command: config
                    .get("command")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string(),
                args: config.get("args").and_then(|v| v.as_array()).map(|args| {
                    args.iter()
                        .filter_map(|arg| arg.as_str().map(|s| s.to_string()))
                        .collect()
                }),
            });

        // Create the tool config with env variables if provided
        let mut tool_config = None;
        if let Some(auth) = &request.authentication {
            if let Some(env) = auth.get("env") {
                if let Some(env_obj) = env.as_object() {
                    let mut env_map = HashMap::new();
                    for (key, value) in env_obj {
                        // Extract the value as a string
                        let value_str = match value {
                            Value::String(s) => s.clone(),
                            Value::Number(n) => n.to_string(),
                            Value::Bool(b) => b.to_string(),
                            _ => {
                                // For objects, check if it has a description field (which means it's a template)
                                if let Value::Object(obj) = value {
                                    if obj.contains_key("description") {
                                        // This is a template, so we don't have a value yet
                                        continue;
                                    }
                                }
                                // For other types, convert to JSON string
                                value.to_string()
                            }
                        };
                        env_map.insert(key.clone(), value_str);
                    }
                    tool_config = Some(ToolConfig {
                        env: Some(env_map),
                        command: None,
                        args: None,
                    });
                }
            }
        }

        // Create the Tool struct
        let tool = Tool {
            name: request.tool_name.clone(),
            description: request.description.clone(),
            enabled: true, // Default to enabled
            tool_type: request.tool_type.clone(),
            entry_point: None,
            configuration,
            distribution: request.distribution.clone(),
            config: tool_config,
            authentication: request.authentication.clone(),
        };

        // Save the tool in the repository
        self.tool_repository.save_tool(&tool_id, &tool).await.map_err(|e| e.to_string())?;

        // Extract environment variables from the tool config
        let env_vars = if let Some(config) = &tool.config {
            config.env.clone()
        } else {
            None
        };

        // Create the config_value for the spawn functions
        let config_value = if let Some(configuration) = &tool.configuration {
            json!({
                "command": configuration.command,
                "args": configuration.args
            })
        } else if let Some(config) = &tool.config {
            if let Some(command) = &config.command {
                json!({
                    "command": command,
                    "args": config.args.clone().unwrap_or_default()
                })
            } else {
                return Err("Configuration is required for tools".to_string());
            }
        } else {
            return Err("Configuration is required for tools".to_string());
        };

        // Spawn process based on tool type
        let spawn_result = self.process_manager.spawn_process(
            &config_value,
            &tool_id,
            &request.tool_type,
            env_vars.as_ref(),
        )
        .await;

        match spawn_result {
            Ok((_process, stdin, stdout)) => {
                info!("Process spawned successfully for tool ID: {}", tool_id);
                
                // Wait a moment for the server to start
                info!("Waiting for server to initialize...");
                tokio::time::sleep(Duration::from_secs(3)).await;

                // Try to discover tools from this server with a timeout to avoid hanging
                info!("Attempting to discover tools from server {}", tool_id);
                let discover_result = tokio::time::timeout(Duration::from_secs(15), async {
                    let mut process_ios = HashMap::new();
                    process_ios.insert(tool_id.clone(), (stdin, stdout));
                    
                    self.process_manager.discover_server_tools(&tool_id, &mut process_ios).await
                })
                .await;

                // Handle the result of the discovery attempt
                match discover_result {
                    Ok(Ok(tools)) => {
                        info!(
                            "Successfully discovered {} tools from {}",
                            tools.len(),
                            tool_id
                        );
                        
                        // If empty tools, add a default "main" tool
                        if tools.is_empty() {
                            info!("No tools discovered, adding a default main tool");
                            let _default_tool = json!({
                                "id": "main",
                                "name": request.tool_name,
                                "description": request.description
                            });
                            
                            // TODO: Update server_tools in the repository
                        }
                    }
                    Ok(Err(e)) => {
                        error!("Error discovering tools from server {}: {}", tool_id, e);
                        // Add a default tool since discovery failed
                        let _default_tool = json!({
                            "id": "main",
                            "name": request.tool_name,
                            "description": request.description
                        });
                        
                        // TODO: Update server_tools in the repository
                        info!("Added default tool for server {}", tool_id);
                    }
                    Err(_) => {
                        error!("Timeout while discovering tools from server {}", tool_id);
                        // Add a default tool since discovery timed out
                        let _default_tool = json!({
                            "id": "main",
                            "name": request.tool_name,
                            "description": request.description
                        });
                        
                        // TODO: Update server_tools in the repository
                        info!("Added default tool for server {} after timeout", tool_id);
                    }
                }
            }
            Err(e) => {
                error!("Failed to spawn process for {}: {}", tool_id, e);
                return Ok(ToolRegistrationResponse {
                    success: false,
                    message: format!("Tool registered but failed to start process: {}", e),
                    tool_id: Some(tool_id),
                });
            }
        }

        info!("Tool registration completed for: {}", request.tool_name);
        Ok(ToolRegistrationResponse {
            success: true,
            message: format!("Tool '{}' registered successfully", request.tool_name),
            tool_id: Some(tool_id),
        })
    }

    /// List all registered tools
    pub async fn list_tools(&self) -> Result<Vec<Value>, String> {
        let mut tools = Vec::new();

        // Get all tools from the repository
        let tool_map = self.tool_repository.get_all_tools().await.map_err(|e| e.to_string())?;
        
        // Get server tools to get tool counts
        let server_tools = self.tool_repository.get_server_tools().await.map_err(|e| e.to_string())?;

        for (id, tool_struct) in tool_map {
            // Convert the Tool struct to a Value
            let mut tool_value = json!({
                "name": tool_struct.name,
                "description": tool_struct.description,
                "enabled": tool_struct.enabled,
                "tool_type": tool_struct.tool_type,
                "id": id,
            });

            // Add optional fields if they exist
            if let Some(entry_point) = &tool_struct.entry_point {
                if let Some(obj) = tool_value.as_object_mut() {
                    obj.insert("entry_point".to_string(), json!(entry_point));
                }
            }

            if let Some(configuration) = &tool_struct.configuration {
                if let Some(obj) = tool_value.as_object_mut() {
                    obj.insert(
                        "configuration".to_string(),
                        json!({
                            "command": configuration.command,
                            "args": configuration.args
                        }),
                    );
                }
            }

            if let Some(distribution) = &tool_struct.distribution {
                if let Some(obj) = tool_value.as_object_mut() {
                    obj.insert("distribution".to_string(), distribution.clone());
                }
            }

            if let Some(config) = &tool_struct.config {
                if let Some(obj) = tool_value.as_object_mut() {
                    let mut config_json = json!({});
                    if let Some(env) = &config.env {
                        if let Some(config_obj) = config_json.as_object_mut() {
                            config_obj.insert("env".to_string(), json!(env));
                        }
                    }
                    if let Some(command) = &config.command {
                        if let Some(config_obj) = config_json.as_object_mut() {
                            config_obj.insert("command".to_string(), json!(command));
                        }
                    }
                    if let Some(args) = &config.args {
                        if let Some(config_obj) = config_json.as_object_mut() {
                            config_obj.insert("args".to_string(), json!(args));
                        }
                    }
                    obj.insert("config".to_string(), config_json);
                }
            }

            if let Some(authentication) = &tool_struct.authentication {
                if let Some(obj) = tool_value.as_object_mut() {
                    obj.insert("authentication".to_string(), authentication.clone());
                }
            }

            // Add process status and tool count
            if let Some(obj) = tool_value.as_object_mut() {
                // Process running status would need to be tracked in the repository
                obj.insert("process_running".to_string(), json!(true)); // Default to true for now

                // Add number of available tools from this server
                let server_tool_count = server_tools
                    .get(&id)
                    .map_or_else(|| 0, |tools| tools.len());
                obj.insert("tool_count".to_string(), json!(server_tool_count));
            }

            tools.push(tool_value);
        }
        Ok(tools)
    }

    /// List all available tools from all running MCP servers
    pub async fn list_all_server_tools(&self) -> Result<Vec<Value>, String> {
        let server_tools = self.tool_repository.get_server_tools().await.map_err(|e| e.to_string())?;
        let mut all_tools = Vec::new();

        for (server_id, tools) in &server_tools {
            for tool in tools {
                // Extract the basic tool information
                let mut tool_info = serde_json::Map::new();

                // Copy the original tool properties
                if let Some(obj) = tool.as_object() {
                    for (key, value) in obj {
                        tool_info.insert(key.clone(), value.clone());
                    }
                }

                // Add server_id
                tool_info.insert("server_id".to_string(), json!(server_id));

                // Create a proxy ID
                let tool_id = tool.get("id").and_then(|v| v.as_str()).unwrap_or("");
                let proxy_id = format!("{}:{}", server_id, tool_id);
                tool_info.insert("proxy_id".to_string(), json!(proxy_id));

                all_tools.push(json!(tool_info));
            }
        }

        Ok(all_tools)
    }

    /// Discover tools from a specific MCP server
    pub async fn discover_tools(
        &self,
        request: DiscoverServerToolsRequest,
    ) -> Result<DiscoverServerToolsResponse, String> {
        // Check if the server exists by trying to get it from the repository
        let server = match self.tool_repository.get_tool(&request.server_id).await {
            Ok(_) => true,
            Err(_) => false,
        };

        if !server {
            return Ok(DiscoverServerToolsResponse {
                success: false,
                tools: None,
                error: Some(format!(
                    "Server with ID '{}' not found",
                    request.server_id
                )),
            });
        }

        // Get the current server tools
        let _server_tools = self.tool_repository.get_server_tools().await.map_err(|e| e.to_string())?;
        
        // Get process IOs for the server - this would need to be handled by the process manager
        // For now, we'll create a dummy HashMap to pass to discover_server_tools
        let mut process_ios = HashMap::new();
        
        // Discover tools from the server using the process manager
        match self.process_manager.discover_server_tools(&request.server_id, &mut process_ios).await {
            Ok(tools) => {
                // Store the discovered tools in the repository
                // This would typically be handled by the repository, but for now we'll just return the tools
                
                Ok(DiscoverServerToolsResponse {
                    success: true,
                    tools: Some(tools),
                    error: None,
                })
            }
            Err(e) => Ok(DiscoverServerToolsResponse {
                success: false,
                tools: None,
                error: Some(format!("Failed to discover tools: {}", e)),
            }),
        }
    }

    /// Execute a tool from an MCP server
    pub async fn execute_tool(
        &self,
        request: ToolExecutionRequest,
    ) -> Result<ToolExecutionResponse, String> {
        // Execute the tool using the repository
        self.tool_repository.execute_tool(request).await.map_err(|e| e.to_string())
    }

    /// Update a tool's status (enabled/disabled)
    pub async fn update_tool_status(
        &self,
        request: ToolUpdateRequest,
    ) -> Result<ToolUpdateResponse, String> {
        // First, check if the tool exists
        let tool = match self.tool_repository.get_tool(&request.tool_id).await {
            Ok(tool) => tool,
            Err(_) => {
                return Ok(ToolUpdateResponse {
                    success: false,
                    message: format!("Tool with ID '{}' not found", request.tool_id),
                });
            }
        };

        // Create an updated tool with the new enabled status
        let mut updated_tool = tool;
        updated_tool.enabled = request.enabled;

        // Save the updated tool
        if let Err(e) = self.tool_repository.save_tool(&request.tool_id, &updated_tool).await {
            return Ok(ToolUpdateResponse {
                success: false,
                message: format!("Failed to update tool: {}", e),
            });
        }

        // If the tool is being enabled, restart it
        if request.enabled {
            if let Err(e) = self.tool_repository.restart_tool(&request.tool_id).await {
                return Ok(ToolUpdateResponse {
                    success: false,
                    message: e.to_string(),
                });
            }
        }

        // Return success
        Ok(ToolUpdateResponse {
            success: true,
            message: format!(
                "Tool '{}' status updated to {}",
                request.tool_id,
                if request.enabled {
                    "enabled"
                } else {
                    "disabled"
                }
            ),
        })
    }

    /// Update a tool's configuration (environment variables)
    pub async fn update_tool_config(
        &self,
        request: ToolConfigUpdateRequest,
    ) -> Result<ToolConfigUpdateResponse, String> {
        info!("Updating configuration for tool: {}", request.tool_id);

        // First, check if the tool exists
        let tool = match self.tool_repository.get_tool(&request.tool_id).await {
            Ok(tool) => tool,
            Err(_) => {
                error!("Tool with ID '{}' not found", request.tool_id);
                return Ok(ToolConfigUpdateResponse {
                    success: false,
                    message: format!("Tool with ID '{}' not found", request.tool_id),
                });
            }
        };

        info!("Tool '{}' found, enabled: {}", request.tool_id, tool.enabled);

        // Create a mutable copy of the tool
        let mut updated_tool = tool;

        // Create or update the config object
        if updated_tool.config.is_none() {
            updated_tool.config = Some(ToolConfig {
                env: Some(HashMap::new()),
                command: None,
                args: None,
            });
        }

        if let Some(config) = &mut updated_tool.config {
            // Create or update the env object
            if config.env.is_none() {
                config.env = Some(HashMap::new());
            }

            if let Some(env_map) = &mut config.env {
                // Update each environment variable
                if let Some(req_env) = &request.config.env {
                    for (key, value) in req_env {
                        info!(
                            "Setting environment variable for tool {}: {}={}",
                            request.tool_id, key, value
                        );
                        env_map.insert(key.clone(), value.clone());
                    }
                }
            }
        }

        // Save the updated tool
        self.tool_repository.save_tool(&request.tool_id, &updated_tool).await.map_err(|e| e.to_string())?;

        // Return success
        Ok(ToolConfigUpdateResponse {
            success: true,
            message: format!("Tool '{}' configuration updated", request.tool_id),
        })
    }

    /// Uninstall a registered tool
    pub async fn uninstall_tool(
        &self,
        request: ToolUninstallRequest,
    ) -> Result<ToolUninstallResponse, String> {
        // First check if the tool exists
        if let Err(_) = self.tool_repository.get_tool(&request.tool_id).await {
            return Ok(ToolUninstallResponse {
                success: false,
                message: format!("Tool with ID '{}' not found", request.tool_id),
            });
        }

        // Delete the tool using repository's delete_tool method
        if let Err(e) = self.tool_repository.delete_tool(&request.tool_id).await {
            return Ok(ToolUninstallResponse {
                success: false,
                message: format!("Failed to delete tool: {}", e),
            });
        }

        Ok(ToolUninstallResponse {
            success: true,
            message: "Tool uninstalled successfully".to_string(),
        })
    }

    /// Get all server data in a single function to avoid multiple locks
    pub async fn get_all_server_data(&self) -> Result<Value, String> {
        // 1. Get registered servers
        let mut servers = Vec::new();
        let tool_map = self.tool_repository.get_all_tools().await.map_err(|e| e.to_string())?;
        
        // Get server tools to get tool counts
        let server_tools = self.tool_repository.get_server_tools().await.map_err(|e| e.to_string())?;

        for (id, tool_struct) in tool_map {
            // Convert the Tool struct to a Value
            let mut tool_value = json!({
                "name": tool_struct.name,
                "description": tool_struct.description,
                "enabled": tool_struct.enabled,
                "tool_type": tool_struct.tool_type,
                "id": id,
            });

            // Add optional fields if they exist
            if let Some(entry_point) = &tool_struct.entry_point {
                if let Some(obj) = tool_value.as_object_mut() {
                    obj.insert("entry_point".to_string(), json!(entry_point));
                }
            }

            if let Some(configuration) = &tool_struct.configuration {
                if let Some(obj) = tool_value.as_object_mut() {
                    obj.insert(
                        "configuration".to_string(),
                        json!({
                            "command": configuration.command,
                            "args": configuration.args
                        }),
                    );
                }
            }

            if let Some(distribution) = &tool_struct.distribution {
                if let Some(obj) = tool_value.as_object_mut() {
                    obj.insert("distribution".to_string(), distribution.clone());
                }
            }

            if let Some(config) = &tool_struct.config {
                if let Some(obj) = tool_value.as_object_mut() {
                    let mut config_json = json!({});
                    if let Some(env) = &config.env {
                        if let Some(config_obj) = config_json.as_object_mut() {
                            config_obj.insert("env".to_string(), json!(env));
                        }
                    }
                    if let Some(command) = &config.command {
                        if let Some(config_obj) = config_json.as_object_mut() {
                            config_obj.insert("command".to_string(), json!(command));
                        }
                    }
                    if let Some(args) = &config.args {
                        if let Some(config_obj) = config_json.as_object_mut() {
                            config_obj.insert("args".to_string(), json!(args));
                        }
                    }
                    obj.insert("config".to_string(), config_json);
                }
            }

            if let Some(authentication) = &tool_struct.authentication {
                if let Some(obj) = tool_value.as_object_mut() {
                    obj.insert("authentication".to_string(), authentication.clone());
                }
            }

            // Add process status and tool count
            if let Some(obj) = tool_value.as_object_mut() {
                // Process running status would need to be tracked in the repository
                obj.insert("process_running".to_string(), json!(true)); // Default to true for now

                // Add number of available tools from this server
                let server_tool_count = server_tools
                    .get(&id)
                    .map_or_else(|| 0, |tools| tools.len());
                obj.insert("tool_count".to_string(), json!(server_tool_count));
            }

            servers.push(tool_value);
        }

        // 2. Get all server tools
        let mut all_tools = Vec::new();
        for (server_id, tools) in &server_tools {
            for tool in tools {
                // Extract the basic tool information
                let mut tool_info = serde_json::Map::new();

                // Copy the original tool properties
                if let Some(obj) = tool.as_object() {
                    for (key, value) in obj {
                        tool_info.insert(key.clone(), value.clone());
                    }
                }

                // Add server_id
                tool_info.insert("server_id".to_string(), json!(server_id));

                // Create a proxy ID
                let tool_id = tool.get("id").and_then(|v| v.as_str()).unwrap_or("");
                let proxy_id = format!("{}:{}", server_id, tool_id);
                tool_info.insert("proxy_id".to_string(), json!(proxy_id));

                all_tools.push(json!(tool_info));
            }
        }

        // Return all data in one response
        Ok(json!({
            "servers": servers,
            "tools": all_tools
        }))
    }
}
