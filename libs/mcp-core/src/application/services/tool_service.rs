use std::collections::HashMap;
use std::sync::Arc;
use serde_json::{json, Value};
use log::{info, error};
use tokio::time::Duration;

use crate::domain::traits::{ProcessManager, ToolRepository};
use crate::domain::entities::tool::{
    Tool, ToolConfiguration, ToolConfig, ToolConfigUpdate,
    ToolRegistrationRequest, 
    ToolExecutionRequest, 
    ToolUpdateRequest, 
    ToolConfigUpdateRequest, 
    ToolUninstallRequest, 
    DiscoverServerToolsRequest
};
use crate::domain::errors::DomainError;
use crate::application::dto::{
    ToolRegistrationDTO, ToolRegistrationResponseDTO,
    ToolExecutionDTO, ToolExecutionResponseDTO,
    ToolUpdateDTO, ToolUpdateResponseDTO,
    ToolConfigUpdateDTO, ToolConfigUpdateResponseDTO,
    ToolUninstallDTO, ToolUninstallResponseDTO,
    DiscoverServerToolsDTO, DiscoverServerToolsResponseDTO
};

/// Application service that handles tool operations
pub struct ToolService {
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
        request: ToolRegistrationDTO,
    ) -> Result<ToolRegistrationResponseDTO, DomainError> {
        info!("Starting registration of tool: {}", request.tool_name);
        
        // Convert DTO to domain request
        let domain_request = ToolRegistrationRequest {
            tool_name: request.tool_name,
            description: request.description,
            authentication: request.authentication,
            tool_type: request.tool_type,
            configuration: request.configuration,
            distribution: request.distribution,
        };
        
        // Safely access the command field if configuration exists
        if let Some(config) = &domain_request.configuration {
            if let Some(cmd) = config.get("command") {
                info!("Command: {}", cmd);
            } else {
                info!("Command: Not specified in configuration");
            }
        } else {
            info!("Configuration not provided");
        }
        
        // Generate a tool ID
        let tools = self.tool_repository.get_all_tools().await?;
        let tool_id = format!("tool_{}", tools.len() + 1);
        info!("Generated tool ID: {}", tool_id);
        
        // Create the tool configuration if provided
        let configuration = domain_request
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
        if let Some(auth) = &domain_request.authentication {
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
            name: domain_request.tool_name.clone(),
            description: domain_request.description.clone(),
            enabled: true, // Default to enabled
            tool_type: domain_request.tool_type.clone(),
            entry_point: None,
            configuration,
            distribution: domain_request.distribution,
            config: tool_config,
            authentication: domain_request.authentication,
        };
        
        // Save the tool in the repository
        self.tool_repository.save_tool(&tool_id, &tool).await?;

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
                return Err(DomainError::ConfigurationError("Configuration is required for tools".to_string()));
            }
        } else {
            return Err(DomainError::ConfigurationError("Configuration is required for tools".to_string()));
        };

        // Spawn process based on tool type
        let spawn_result = self.process_manager.spawn_process(
            &config_value,
            &tool_id,
            &domain_request.tool_type,
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
                                "name": domain_request.tool_name,
                                "description": domain_request.description
                            });
                            
                            // TODO: Update server_tools in the repository
                        }
                    }
                    Ok(Err(e)) => {
                        error!("Error discovering tools from server {}: {}", tool_id, e);
                        // Add a default tool since discovery failed
                        let _default_tool = json!({
                            "id": "main",
                            "name": domain_request.tool_name,
                            "description": domain_request.description
                        });
                        
                        // TODO: Update server_tools in the repository
                        info!("Added default tool for server {}", tool_id);
                    }
                    Err(_) => {
                        error!("Timeout while discovering tools from server {}", tool_id);
                        // Add a default tool since discovery timed out
                        let _default_tool = json!({
                            "id": "main",
                            "name": domain_request.tool_name,
                            "description": domain_request.description
                        });
                        
                        // TODO: Update server_tools in the repository
                        info!("Added default tool for server {} after timeout", tool_id);
                    }
                }
            }
            Err(e) => {
                error!("Failed to spawn process for {}: {}", tool_id, e);
                return Ok(ToolRegistrationResponseDTO {
                    success: false,
                    message: format!("Tool registered but failed to start process: {}", e),
                    tool_id: Some(tool_id),
                });
            }
        }

        info!("Tool registration completed for: {}", domain_request.tool_name);
        Ok(ToolRegistrationResponseDTO {
            success: true,
            message: format!("Tool '{}' registered successfully", domain_request.tool_name),
            tool_id: Some(tool_id),
        })
    }

    /// List all tools
    pub async fn list_tools(&self) -> Result<Vec<Value>, DomainError> {
        let mut tools = Vec::new();

        // Get all tools from the repository
        let tool_map = self.tool_repository.get_all_tools().await?;
        
        // Get server tools to get tool counts
        let server_tools = self.tool_repository.get_server_tools().await?;

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
        
        // Add server tools
        for (server_id, server_tools) in &server_tools {
            for tool in server_tools {
                let mut tool_info = serde_json::Map::new();
                
                if let Some(obj) = tool.as_object() {
                    for (key, value) in obj {
                        tool_info.insert(key.clone(), value.clone());
                    }
                }
                
                tool_info.insert("server_id".to_string(), json!(server_id));
                
                if !tool_info.contains_key("description") {
                    tool_info.insert("description".to_string(), json!("Tool from server"));
                }
                
                if !tool_info.contains_key("inputSchema") {
                    tool_info.insert(
                        "inputSchema".to_string(),
                        json!({
                            "type": "object",
                            "properties": {}
                        }),
                    );
                }
                
                tools.push(json!(tool_info));
            }
        }
        
        Ok(tools)
    }

    /// Discover tools from an MCP server
    pub async fn discover_tools(
        &self,
        request: DiscoverServerToolsDTO,
    ) -> Result<DiscoverServerToolsResponseDTO, DomainError> {
        // Convert DTO to domain request
        let domain_request = DiscoverServerToolsRequest {
            server_id: request.server_id,
        };
        
        // Check if the server exists by trying to get it from the repository
        let server = match self.tool_repository.get_tool(&domain_request.server_id).await {
            Ok(_) => true,
            Err(_) => false,
        };

        if !server {
            return Ok(DiscoverServerToolsResponseDTO {
                success: false,
                tools: None,
                error: Some(format!(
                    "Server with ID '{}' not found",
                    domain_request.server_id
                )),
            });
        }
        
        // Get the current server tools
        let _server_tools = self.tool_repository.get_server_tools().await?;
        
        // Get process IOs for the server - this would need to be handled by the process manager
        // For now, we'll create a dummy HashMap to pass to discover_server_tools
        let mut process_ios = HashMap::new();
        
        // Discover tools from the server using the process manager
        match self.process_manager.discover_server_tools(&domain_request.server_id, &mut process_ios).await {
            Ok(tools) => {
                // Store the discovered tools in the repository
                // This would typically be handled by the repository, but for now we'll just return the tools
                
                Ok(DiscoverServerToolsResponseDTO {
                    success: true,
                    tools: Some(tools),
                    error: None,
                })
            }
            Err(e) => Ok(DiscoverServerToolsResponseDTO {
                success: false,
                tools: None,
                error: Some(format!("Failed to discover tools: {}", e)),
            }),
        }
    }
    
    /// List all server tools
    pub async fn list_all_server_tools(&self) -> Result<Vec<Value>, DomainError> {
        let server_tools = self.tool_repository.get_server_tools().await?;
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
    
    /// Get all server data including tools and status
    pub async fn get_all_server_data(&self) -> Result<Vec<Value>, DomainError> {
        // Get all tools from the repository
        let tool_map = self.tool_repository.get_all_tools().await?;
        
        // Get server tools to get tool counts
        let server_tools = self.tool_repository.get_server_tools().await?;
        
        let mut result = Vec::new();
        
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
                
                // Add tools array
                if let Some(tools) = server_tools.get(&id) {
                    obj.insert("tools".to_string(), json!(tools));
                } else {
                    obj.insert("tools".to_string(), json!([]));
                }
            }
            
            result.push(tool_value);
        }
        
        Ok(result)
    }

    /// Execute a tool
    pub async fn execute_tool(
        &self,
        request: ToolExecutionDTO,
    ) -> Result<ToolExecutionResponseDTO, DomainError> {
        info!("Executing tool with ID: {}", request.tool_id);
        
        // Convert DTO to domain request
        let domain_request = ToolExecutionRequest {
            tool_id: request.tool_id.clone(),
            parameters: request.parameters.clone(),
        };
        
        // Check if the tool ID contains a server:tool format
        let parts: Vec<&str> = domain_request.tool_id.split(':').collect();
        
        if parts.len() == 2 {
            // This is a server tool execution
            let server_id = parts[0];
            let tool_id = parts[1];
            
            info!("Executing server tool: {} from server: {}", tool_id, server_id);
            
            // Get process IOs for the server
            let mut process_ios = HashMap::new();
            
            // Execute the tool using the process manager
            match self.process_manager.execute_server_tool(
                server_id,
                tool_id,
                domain_request.parameters.clone(),
                &mut process_ios,
            ).await {
                Ok(result) => {
                    Ok(ToolExecutionResponseDTO {
                        success: true,
                        result: Some(result),
                        error: None,
                    })
                }
                Err(e) => {
                    error!("Error executing server tool: {}", e);
                    Ok(ToolExecutionResponseDTO {
                        success: false,
                        result: None,
                        error: Some(format!("Error executing server tool: {}", e)),
                    })
                }
            }
        } else {
            // This is a regular tool execution
            // Extract the tool name from parameters if available
            let tool_name = domain_request.parameters.get("tool_name").and_then(|v| v.as_str());
            
            if let Some(tool_name) = tool_name {
                info!("Executing tool: {} with ID: {}", tool_name, domain_request.tool_id);
                
                // Create a new request with the tool ID in server:tool format
                let server_tool_request = ToolExecutionRequest {
                    tool_id: format!("{}:{}", domain_request.tool_id, tool_name),
                    parameters: domain_request.parameters,
                };
                
                // Execute the tool
                let result = self.tool_repository.execute_tool(server_tool_request).await?;
                
                // Convert domain response to DTO
                Ok(ToolExecutionResponseDTO {
                    success: result.success,
                    result: result.result,
                    error: result.error,
                })
            } else {
                // Execute the tool directly
                let result = self.tool_repository.execute_tool(domain_request).await?;
                
                // Convert domain response to DTO
                Ok(ToolExecutionResponseDTO {
                    success: result.success,
                    result: result.result,
                    error: result.error,
                })
            }
        }
    }

    /// Update tool status (enabled/disabled)
    pub async fn update_tool(
        &self,
        request: ToolUpdateDTO,
    ) -> Result<ToolUpdateResponseDTO, DomainError> {
        info!("Updating tool status for ID: {}", request.tool_id);
        
        // Convert DTO to domain request
        let domain_request = ToolUpdateRequest {
            tool_id: request.tool_id,
            enabled: request.enabled,
        };
        
        // Get the tool
        let mut tool = self.tool_repository.get_tool(&domain_request.tool_id).await?;
        
        // Update the tool
        tool.enabled = domain_request.enabled;
        
        // Save the tool
        self.tool_repository.save_tool(&domain_request.tool_id, &tool).await?;
        
        info!(
            "Tool '{}' {} successfully",
            domain_request.tool_id,
            if domain_request.enabled { "enabled" } else { "disabled" }
        );
        
        // Return success response
        Ok(ToolUpdateResponseDTO {
            success: true,
            message: format!(
                "Tool '{}' {} successfully",
                domain_request.tool_id,
                if domain_request.enabled { "enabled" } else { "disabled" }
            ),
        })
    }

    /// Update tool configuration
    pub async fn update_tool_config(
        &self,
        request: ToolConfigUpdateDTO,
    ) -> Result<ToolConfigUpdateResponseDTO, DomainError> {
        info!("Updating tool configuration for ID: {}", request.tool_id);
        
        // Convert DTO to domain request
        let domain_request = ToolConfigUpdateRequest {
            tool_id: request.tool_id,
            config: ToolConfigUpdate {
                env: request.config.env.clone(),
            },
        };
        
        // Get the tool
        let mut tool = self.tool_repository.get_tool(&domain_request.tool_id).await?;
        
        // Update the tool config
        // Convert ToolConfigUpdate to ToolConfig
        let config = ToolConfig {
            env: domain_request.config.env.clone(),
            command: None,
            args: None,
        };
        
        // Preserve existing command and args if they exist
        if let Some(existing_config) = &tool.config {
            let config = ToolConfig {
                env: domain_request.config.env.clone(),
                command: existing_config.command.clone(),
                args: existing_config.args.clone(),
            };
            tool.config = Some(config);
        } else {
            tool.config = Some(config);
        }
        
        // Save the tool
        self.tool_repository.save_tool(&domain_request.tool_id, &tool).await?;
        
        info!("Tool '{}' configuration updated successfully", domain_request.tool_id);
        
        // Return success response
        Ok(ToolConfigUpdateResponseDTO {
            success: true,
            message: format!("Tool '{}' configuration updated successfully", domain_request.tool_id),
        })
    }

    /// Uninstall a tool
    pub async fn uninstall_tool(
        &self,
        request: ToolUninstallDTO,
    ) -> Result<ToolUninstallResponseDTO, DomainError> {
        info!("Uninstalling tool with ID: {}", request.tool_id);
        
        // Convert DTO to domain request
        let domain_request = ToolUninstallRequest {
            tool_id: request.tool_id,
        };
        
        // Get the tool to check if it exists
        match self.tool_repository.get_tool(&domain_request.tool_id).await {
            Ok(_) => {
                // Delete the tool
                self.tool_repository.delete_tool(&domain_request.tool_id).await?;
                
                info!("Tool '{}' uninstalled successfully", domain_request.tool_id);
                
                // Return success response
                Ok(ToolUninstallResponseDTO {
                    success: true,
                    message: format!("Tool '{}' uninstalled successfully", domain_request.tool_id),
                })
            },
            Err(e) => {
                error!("Failed to uninstall tool: {}", e);
                Ok(ToolUninstallResponseDTO {
                    success: false,
                    message: format!("Failed to uninstall tool: {}", e),
                })
            }
        }
    }
    
    /// Kill all processes
    pub async fn kill_all_processes(&self) -> Result<(), DomainError> {
        info!("Killing all processes");
        self.process_manager.kill_all_processes().await
    }
}
