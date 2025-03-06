use std::collections::HashMap;
use std::sync::Arc;
use serde_json::{json, Value};
use log::{info, error};
use tokio::time::Duration;
use tokio::sync::RwLock;

use crate::mcp_state::MCPState;
use crate::models::models::{
    DiscoverServerToolsRequest, DiscoverServerToolsResponse, 
    ToolRegistrationRequest, ToolRegistrationResponse,
    ToolExecutionRequest, ToolExecutionResponse,
    ToolUninstallRequest, ToolUninstallResponse,
    ToolUpdateRequest, ToolUpdateResponse,
    ToolConfigUpdateRequest, ToolConfigUpdateResponse,
};
use crate::models::models::{ToolConfiguration, ToolConfig, Tool};

use crate::mcp_proxy::{
    discover_server_tools, execute_server_tool, kill_process, spawn_process
};

/// Domain service that handles "register", "uninstall", "list", "execute"...
pub struct ToolService {
    // Store reference to the state
    mcp_state: Arc<RwLock<MCPState>>,
}

impl ToolService {
    pub fn new(mcp_state: Arc<RwLock<MCPState>>) -> Self {
        Self { mcp_state }
    }

    /// Register a new tool
    pub async fn register_tool(
        &self,
        request: ToolRegistrationRequest,
    ) -> Result<ToolRegistrationResponse, String> {
        // Copy logic from mcp_proxy::register_tool
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

        let mcp_state = self.mcp_state.read().await;
        let mut registry = mcp_state.tool_registry.write().await;

        // Generate a simple tool ID (in production, use UUIDs)
        let tool_id = format!("tool_{}", registry.get_all_tools()?.len() + 1);
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

        // Save the tool in the registry
        registry.save_tool(&tool_id, &tool)?;

        // Create a default empty tools list
        registry.server_tools.insert(tool_id.clone(), Vec::new());

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
        let spawn_result = spawn_process(
            &config_value,
            &tool_id,
            &request.tool_type,
            env_vars.as_ref(),
        )
        .await;

        match spawn_result {
            Ok((process, stdin, stdout)) => {
                info!("Process spawned successfully for tool ID: {}", tool_id);
                registry.processes.insert(tool_id.clone(), Some(process));
                registry
                    .process_ios
                    .insert(tool_id.clone(), (stdin, stdout));

                // Wait a moment for the server to start
                info!("Waiting for server to initialize...");
                drop(registry); // Release the lock during sleep
                drop(mcp_state); // Release the lock during sleep
                tokio::time::sleep(Duration::from_secs(3)).await;

                // Try to discover tools from this server with a timeout to avoid hanging
                info!("Attempting to discover tools from server {}", tool_id);
                let discover_result = tokio::time::timeout(Duration::from_secs(15), async {
                    let mcp_state = self.mcp_state.read().await;
                    let mut registry = mcp_state.tool_registry.write().await;
                    discover_server_tools(&tool_id, &mut registry).await
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
                        let mcp_state = self.mcp_state.read().await;
                        let mut registry = mcp_state.tool_registry.write().await;
                        // Clone tools before inserting to avoid the "moved value" error
                        let tools_clone = tools.clone();
                        registry.server_tools.insert(tool_id.clone(), tools);

                        // If empty tools, add a default "main" tool
                        if tools_clone.is_empty() {
                            info!("No tools discovered, adding a default main tool");
                            let default_tool = json!({
                                "id": "main",
                                "name": request.tool_name,
                                "description": request.description
                            });
                            registry
                                .server_tools
                                .insert(tool_id.clone(), vec![default_tool]);
                        }
                    }
                    Ok(Err(e)) => {
                        error!("Error discovering tools from server {}: {}", tool_id, e);
                        // Add a default tool since discovery failed
                        let mcp_state = self.mcp_state.read().await;
                        let mut registry = mcp_state.tool_registry.write().await;
                        let default_tool = json!({
                            "id": "main",
                            "name": request.tool_name,
                            "description": request.description
                        });
                        registry
                            .server_tools
                            .insert(tool_id.clone(), vec![default_tool]);
                        info!("Added default tool for server {}", tool_id);
                    }
                    Err(_) => {
                        error!("Timeout while discovering tools from server {}", tool_id);
                        // Add a default tool since discovery timed out
                        let mcp_state = self.mcp_state.read().await;
                        let mut registry = mcp_state.tool_registry.write().await;
                        let default_tool = json!({
                            "id": "main",
                            "name": request.tool_name,
                            "description": request.description
                        });
                        registry
                            .server_tools
                            .insert(tool_id.clone(), vec![default_tool]);
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
        // Copy logic from mcp_proxy::list_tools
        let mcp_state = self.mcp_state.read().await;
        let registry = mcp_state.tool_registry.read().await;
        let mut tools = Vec::new();

        // Get all tools from the registry
        let tool_map = registry.get_all_tools()?;

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

            // Add process status - check the processes map
            if let Some(obj) = tool_value.as_object_mut() {
                let process_running = registry.processes.contains_key(&id);
                obj.insert("process_running".to_string(), json!(process_running));

                // Add number of available tools from this server
                let server_tool_count = registry
                    .server_tools
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
        // Copy logic from mcp_proxy::list_all_server_tools
        let mcp_state = self.mcp_state.read().await;
        let registry = mcp_state.tool_registry.read().await;
        let mut all_tools = Vec::new();

        for (server_id, tools) in &registry.server_tools {
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
        // Copy logic from mcp_proxy::discover_tools
        // Check if the server exists and is running
        let server_running = {
            let mcp_state = self.mcp_state.read().await;
            let registry = mcp_state.tool_registry.read().await;
            registry
                .processes
                .get(&request.server_id)
                .is_some_and(|p| p.is_some())
        };

        if !server_running {
            return Ok(DiscoverServerToolsResponse {
                success: false,
                tools: None,
                error: Some(format!(
                    "Server with ID '{}' is not running",
                    request.server_id
                )),
            });
        }

        // Discover tools from the server
        let mcp_state = self.mcp_state.read().await;
        let mut registry = mcp_state.tool_registry.write().await;
        match discover_server_tools(&request.server_id, &mut registry).await {
            Ok(tools) => {
                // Store the discovered tools
                registry
                    .server_tools
                    .insert(request.server_id.clone(), tools.clone());

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
    pub async fn execute_proxy_tool(
        &self,
        request: ToolExecutionRequest,
    ) -> Result<ToolExecutionResponse, String> {
        // Copy logic from mcp_proxy::execute_proxy_tool
        // Extract server_id and tool_id from the proxy_id
        let parts: Vec<&str> = request.tool_id.split(':').collect();
        println!("parts: {:?}", parts);
        if parts.len() != 2 {
            return Err("Invalid tool_id format. Expected 'server_id:tool_id'".to_string());
        }

        let server_id = parts[0];
        println!("server_id: {}", server_id);
        let tool_id = parts[1];
        println!("tool_id: {}", tool_id);

        // Execute the tool on the server
        let mcp_state = self.mcp_state.read().await;
        let mut registry = mcp_state.tool_registry.write().await;
        match execute_server_tool(
            server_id,
            tool_id,
            request.parameters.clone(),
            &mut registry,
        )
        .await
        {
            Ok(result) => Ok(ToolExecutionResponse {
                success: true,
                result: Some(result),
                error: None,
            }),
            Err(e) => Ok(ToolExecutionResponse {
                success: false,
                result: None,
                error: Some(e.to_string()),
            }),
        }
    }

    /// Update a tool's status (enabled/disabled)
    pub async fn update_tool_status(
        &self,
        request: ToolUpdateRequest,
    ) -> Result<ToolUpdateResponse, String> {
        // Copy logic from mcp_proxy::update_tool_status
        // First, check if the tool exists and get the necessary information
        let tool_info = {
            let mcp_state = self.mcp_state.read().await;
            let registry = mcp_state.tool_registry.read().await;

            if let Ok(tool) = registry.get_tool(&request.tool_id) {
                // Extract and clone the necessary values
                let tool_type = tool.tool_type.clone();
                let entry_point = tool.entry_point.clone().unwrap_or_default();
                let process_running = registry
                    .processes
                    .get(&request.tool_id)
                    .is_some_and(|p| p.is_some());

                Some((tool_type, entry_point, process_running))
            } else {
                None
            }
        };

        // If the tool doesn't exist, return an error
        if tool_info.is_none() {
            return Ok(ToolUpdateResponse {
                success: false,
                message: format!("Tool with ID '{}' not found", request.tool_id),
            });
        }

        // Now handle the process based on the enabled status
        let result = {
            let mcp_state = self.mcp_state.read().await;
            let registry = mcp_state.tool_registry.write().await;

            // Get the current tool data
            let mut tool = registry.get_tool(&request.tool_id)?;

            // Update the enabled status
            tool.enabled = request.enabled;

            // Save the updated tool
            registry.save_tool(&request.tool_id, &tool)?;

            // Drop the write lock before trying to restart the tool
            drop(registry);

            if request.enabled {
                let mcp_state = self.mcp_state.read().await;
                let mut registry = mcp_state.tool_registry.write().await;
                registry.restart_tool(&request.tool_id).await
            } else {
                Ok(())
            }
        };

        // Handle any errors from the process management
        if let Err(e) = result {
            return Ok(ToolUpdateResponse {
                success: false,
                message: e,
            });
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
        // Copy logic from mcp_proxy::update_tool_config
        info!("Updating configuration for tool: {}", request.tool_id);

        // First, check if the tool exists
        let (tool_exists, is_enabled) = {
            let mcp_state = self.mcp_state.read().await;
            let registry = mcp_state.tool_registry.read().await;
            match registry.get_tool(&request.tool_id) {
                Ok(tool) => (true, tool.enabled),
                Err(_) => (false, false),
            }
        };

        // If the tool doesn't exist, return an error
        if !tool_exists {
            error!("Tool with ID '{}' not found", request.tool_id);
            return Ok(ToolConfigUpdateResponse {
                success: false,
                message: format!("Tool with ID '{}' not found", request.tool_id),
            });
        }

        info!("Tool '{}' found, enabled: {}", request.tool_id, is_enabled);

        // Update the tool configuration
        let mcp_state = self.mcp_state.read().await;
        let registry = mcp_state.tool_registry.write().await;

        // Get the current tool data
        let mut tool = registry.get_tool(&request.tool_id)?;

        // Create or update the config object
        if tool.config.is_none() {
            tool.config = Some(ToolConfig {
                env: Some(HashMap::new()),
                command: None,
                args: None,
            });
        }

        if let Some(config) = &mut tool.config {
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
        registry.save_tool(&request.tool_id, &tool)?;

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
        // Copy logic from mcp_proxy::uninstall_tool
        let mcp_state = self.mcp_state.read().await;
        let mut registry = mcp_state.tool_registry.write().await;

        // First check if the tool exists
        if registry.get_tool(&request.tool_id).is_err() {
            return Ok(ToolUninstallResponse {
                success: false,
                message: format!("Tool with ID '{}' not found", request.tool_id),
            });
        }

        // Kill the process if it's running
        if let Some(Some(process)) = registry.processes.get_mut(&request.tool_id) {
            if let Err(e) = kill_process(process).await {
                return Ok(ToolUninstallResponse {
                    success: false,
                    message: format!("Failed to kill process: {}", e),
                });
            }
        }

        // Remove the process and IOs from the registry
        registry.processes.remove(&request.tool_id);
        registry.process_ios.remove(&request.tool_id);
        registry.server_tools.remove(&request.tool_id);

        // Delete the tool using registry's delete_tool method
        if let Err(e) = registry.delete_tool(&request.tool_id) {
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
        // Copy logic from mcp_proxy::get_all_server_data
        // Acquire a single read lock for all operations
        let mcp_state = self.mcp_state.read().await;
        let registry = mcp_state.tool_registry.read().await;

        // 1. Get registered servers
        let mut servers = Vec::new();
        let tool_map = registry.get_all_tools()?;

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

            // Add process status - check the processes map
            if let Some(obj) = tool_value.as_object_mut() {
                let process_running = registry.processes.contains_key(&id);
                obj.insert("process_running".to_string(), json!(process_running));

                // Add number of available tools from this server
                let server_tool_count = registry
                    .server_tools
                    .get(&id)
                    .map_or_else(|| 0, |tools| tools.len());
                obj.insert("tool_count".to_string(), json!(server_tool_count));
            }

            servers.push(tool_value);
        }

        // 2. Get all server tools
        let mut all_tools = Vec::new();
        for (server_id, tools) in &registry.server_tools {
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
