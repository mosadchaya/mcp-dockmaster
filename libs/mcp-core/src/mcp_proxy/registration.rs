use log::{error, info};
use serde_json::{json, Value};
use std::collections::HashMap;
use tokio::time::Duration;
use crate::mcp_state::MCPState;
use crate::models::types::{
    Tool, ToolConfig, ToolConfiguration, ToolConfigUpdateRequest, ToolConfigUpdateResponse,
    ToolRegistrationRequest, ToolRegistrationResponse, ToolUninstallRequest,
    ToolUninstallResponse, ToolUpdateRequest, ToolUpdateResponse,
};
use super::process_lifecycle::{kill_process, spawn_process};
use super::tool_orchestration::discover_server_tools;

/// Register a new tool with the MCP server
pub async fn register_tool(
    mcp_state: &MCPState,
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

    let registry = mcp_state.tool_registry.write().await;

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
    let mut server_tools = mcp_state.server_tools.write().await;
    server_tools.insert(tool_id.clone(), Vec::new());
    drop(server_tools);

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
            let mut process_manager = mcp_state.process_manager.write().await;
            process_manager
                .processes
                .insert(tool_id.clone(), Some(process));
            process_manager
                .process_ios
                .insert(tool_id.clone(), (stdin, stdout));
            drop(process_manager);

            // Wait a moment for the server to start
            info!("Waiting for server to initialize...");
            tokio::time::sleep(Duration::from_secs(3)).await;

            // Try to discover tools from this server with a timeout to avoid hanging
            info!("Attempting to discover tools from server {}", tool_id);
            let discover_result = tokio::time::timeout(Duration::from_secs(15), async {
                let mut process_manager = mcp_state.process_manager.write().await;
                if let Some((stdin, stdout)) = process_manager.process_ios.get_mut(&tool_id) {
                    discover_server_tools(&tool_id, stdin, stdout).await
                } else {
                    Err(format!("Server {} not found or not running", tool_id))
                }
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
                    // Clone tools before inserting to avoid the "moved value" error
                    let tools_clone = tools.clone();
                    let mut server_tools = mcp_state.server_tools.write().await;
                    server_tools.insert(tool_id.clone(), tools);

                    // If empty tools, add a default "main" tool
                    if tools_clone.is_empty() {
                        info!("No tools discovered, adding a default main tool");
                        let default_tool = json!({
                            "id": "main",
                            "name": request.tool_name,
                            "description": request.description
                        });
                        server_tools.insert(tool_id.clone(), vec![default_tool]);
                    }
                }
                Ok(Err(e)) => {
                    error!("Error discovering tools from server {}: {}", tool_id, e);
                    // Add a default tool since discovery failed
                    let mut server_tools = mcp_state.server_tools.write().await;
                    let default_tool = json!({
                        "id": "main",
                        "name": request.tool_name,
                        "description": request.description
                    });
                    server_tools.insert(tool_id.clone(), vec![default_tool]);
                    info!("Added default tool for server {}", tool_id);
                }
                Err(_) => {
                    error!("Timeout while discovering tools from server {}", tool_id);
                    // Add a default tool since discovery timed out
                    let mut server_tools = mcp_state.server_tools.write().await;
                    let default_tool = json!({
                        "id": "main",
                        "name": request.tool_name,
                        "description": request.description
                    });
                    server_tools.insert(tool_id.clone(), vec![default_tool]);
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
pub async fn list_tools(mcp_state: &MCPState) -> Result<Vec<Value>, String> {
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
            let process_running = {
                let process_manager = mcp_state.process_manager.read().await;
                process_manager
                    .processes
                    .get(&id)
                    .is_some_and(|p| p.is_some())
            };
            obj.insert("process_running".to_string(), json!(process_running));

            // Add server tool count if the process is running
            if process_running {
                let server_tool_count = {
                    let server_tools = mcp_state.server_tools.read().await;
                    server_tools
                        .get(&id)
                        .map(|tools| tools.len())
                        .unwrap_or(0)
                };
                obj.insert("server_tool_count".to_string(), json!(server_tool_count));
            }
        }

        tools.push(tool_value);
    }

    Ok(tools)
}

/// Update a tool's status (enabled/disabled)
pub async fn update_tool_status(
    mcp_state: &MCPState,
    request: ToolUpdateRequest,
) -> Result<ToolUpdateResponse, String> {
    // Get the tool info
    let tool_info = {
        let registry = mcp_state.tool_registry.read().await;
        let tool = registry.get_tool(&request.tool_id)?;

        // Check if the process is running
        let process_running = {
            let process_manager = mcp_state.process_manager.read().await;
            process_manager
                .processes
                .get(&request.tool_id)
                .is_some_and(|p| p.is_some())
        };

        (tool, process_running)
    };

    // Update the tool status
    let result = {
        let registry = mcp_state.tool_registry.write().await;
        let mut tool = tool_info.0.clone();
        tool.enabled = request.enabled;
        registry.save_tool(&request.tool_id, &tool)
    };

    match result {
        Ok(_) => {
            // If enabling and not running, start the process
            if request.enabled && !tool_info.1 {
                match mcp_state.restart_tool(&request.tool_id).await {
                    Ok(_) => {
                        info!("Successfully started process for tool: {}", request.tool_id);
                    }
                    Err(e) => {
                        error!(
                            "Failed to start process for tool {}: {}",
                            request.tool_id, e
                        );
                        return Ok(ToolUpdateResponse {
                            success: false,
                            message: format!("Failed to start process: {}", e),
                        });
                    }
                }
            }
            // If disabling and running, kill the process
            else if !request.enabled && tool_info.1 {
                let mut process_manager = mcp_state.process_manager.write().await;
                if let Some(Some(process)) = process_manager.processes.get_mut(&request.tool_id) {
                    if let Err(e) = kill_process(process).await {
                        error!(
                            "Failed to kill process for tool {}: {}",
                            request.tool_id, e
                        );
                        return Ok(ToolUpdateResponse {
                            success: false,
                            message: format!("Failed to kill process: {}", e),
                        });
                    }
                }
                // Remove the process from the registry
                process_manager
                    .processes
                    .insert(request.tool_id.clone(), None);
                // Remove the process IOs
                process_manager.process_ios.remove(&request.tool_id);
            }

            Ok(ToolUpdateResponse {
                success: true,
                message: format!(
                    "Tool '{}' {} successfully",
                    request.tool_id,
                    if request.enabled {
                        "enabled"
                    } else {
                        "disabled"
                    }
                ),
            })
        }
        Err(e) => Ok(ToolUpdateResponse {
            success: false,
            message: format!("Failed to update tool status: {}", e),
        }),
    }
}

/// Update a tool's configuration (environment variables)
pub async fn update_tool_config(
    mcp_state: &MCPState,
    request: ToolConfigUpdateRequest,
) -> Result<ToolConfigUpdateResponse, String> {
    // Get the tool from the registry
    let tool = {
        let registry = mcp_state.tool_registry.read().await;
        registry.get_tool(&request.tool_id)?
    };

    // Create a new tool with updated config
    let mut updated_tool = tool.clone();

    // Update the tool config
    if let Some(config) = &mut updated_tool.config {
        // Update environment variables
        if let Some(env) = &mut config.env {
            if let Some(request_env) = &request.config.env {
                for (key, value) in request_env {
                    env.insert(key.clone(), value.clone());
                }
            }
        } else {
            config.env = request.config.env.clone();
        }
    } else {
        // Create a new config if none exists
        updated_tool.config = Some(ToolConfig {
            env: request.config.env.clone(),
            command: request.config.command.clone(),
            args: request.config.args.clone(),
        });
    }

    // Save the updated tool
    {
        let registry = mcp_state.tool_registry.write().await;
        registry.save_tool(&request.tool_id, &updated_tool)?;
    }

    // Check if the process is running
    let process_running = {
        let process_manager = mcp_state.process_manager.read().await;
        process_manager
            .processes
            .get(&request.tool_id)
            .is_some_and(|p| p.is_some())
    };

    // If the process is running, restart it (always restart when config changes)
    if process_running {
        // Kill the process
        {
            let mut process_manager = mcp_state.process_manager.write().await;
            if let Some(Some(process)) = process_manager.processes.get_mut(&request.tool_id) {
                if let Err(e) = kill_process(process).await {
                    error!(
                        "Failed to kill process for tool {}: {}",
                        request.tool_id, e
                    );
                    return Ok(ToolConfigUpdateResponse {
                        success: false,
                        message: format!("Failed to kill process: {}", e),
                    });
                }
            }
            // Remove the process from the registry
            process_manager
                .processes
                .insert(request.tool_id.clone(), None);
            // Remove the process IOs
            process_manager.process_ios.remove(&request.tool_id);
        }

        // Restart the process
        if let Err(e) = mcp_state.restart_tool(&request.tool_id).await {
            error!("Failed to restart tool {}: {}", request.tool_id, e);
            return Ok(ToolConfigUpdateResponse {
                success: false,
                message: format!("Failed to restart tool: {}", e),
            });
        }
    }

    Ok(ToolConfigUpdateResponse {
        success: true,
        message: format!("Tool '{}' configuration updated successfully", request.tool_id),
    })
}

/// Uninstall a registered tool
pub async fn uninstall_tool(
    mcp_state: &MCPState,
    request: ToolUninstallRequest,
) -> Result<ToolUninstallResponse, String> {
    // Check if the tool exists
    let tool_exists = {
        let registry = mcp_state.tool_registry.read().await;
        registry.get_tool(&request.tool_id).is_ok()
    };

    if !tool_exists {
        return Ok(ToolUninstallResponse {
            success: false,
            message: format!("Tool with ID '{}' not found", request.tool_id),
        });
    }

    // Check if the process is running
    let process_running = {
        let process_manager = mcp_state.process_manager.read().await;
        process_manager
            .processes
            .get(&request.tool_id)
            .is_some_and(|p| p.is_some())
    };

    // If the process is running, kill it
    if process_running {
        let mut process_manager = mcp_state.process_manager.write().await;
        if let Some(Some(process)) = process_manager.processes.get_mut(&request.tool_id) {
            if let Err(e) = kill_process(process).await {
                error!("Failed to kill process for tool {}: {}", request.tool_id, e);
                return Ok(ToolUninstallResponse {
                    success: false,
                    message: format!("Failed to kill process: {}", e),
                });
            }
        }
        // Remove the process from the registry
        process_manager.processes.remove(&request.tool_id);
        // Remove the process IOs
        process_manager.process_ios.remove(&request.tool_id);
    }

    // Remove the tool from the registry
    {
        let registry = mcp_state.tool_registry.write().await;
        registry.delete_tool(&request.tool_id)?;
    }

    // Remove the tool from the server_tools
    {
        let mut server_tools = mcp_state.server_tools.write().await;
        server_tools.remove(&request.tool_id);
    }

    Ok(ToolUninstallResponse {
        success: true,
        message: format!("Tool '{}' uninstalled successfully", request.tool_id),
    })
}

/// Check if the database exists and has data
pub async fn check_database_exists_command() -> Result<bool, String> {
    let db_manager = crate::database::DBManager::new()?;
    db_manager.check_exists()
}

/// Clear all data from the database
pub async fn clear_database_command() -> Result<String, String> {
    let mut db_manager = crate::database::DBManager::new()?;
    match db_manager.clear_database() {
        Ok(_) => Ok("Database cleared successfully".to_string()),
        Err(e) => Err(format!("Failed to clear database: {}", e)),
    }
}

/// Restart a tool by its ID
pub async fn restart_tool_command(
    mcp_state: &MCPState,
    tool_id: String,
) -> Result<ToolUpdateResponse, String> {
    info!("Received request to restart tool: {}", tool_id);

    // Check if the tool exists
    let tool_exists = {
        let registry = mcp_state.tool_registry.read().await;
        registry.get_tool(&tool_id).is_ok()
    };

    if !tool_exists {
        error!("Tool with ID '{}' not found for restart", tool_id);
        return Ok(ToolUpdateResponse {
            success: false,
            message: format!("Tool with ID '{}' not found", tool_id),
        });
    }

    info!("Tool '{}' found, attempting to restart", tool_id);

    // Restart the tool using MCPState
    let restart_result = mcp_state.restart_tool(&tool_id).await;

    match restart_result {
        Ok(_) => {
            info!("Successfully restarted tool: {}", tool_id);
            Ok(ToolUpdateResponse {
                success: true,
                message: format!("Tool '{}' restarted successfully", tool_id),
            })
        }
        Err(e) => {
            error!("Failed to restart tool {}: {}", tool_id, e);
            Ok(ToolUpdateResponse {
                success: false,
                message: format!("Failed to restart tool: {}", e),
            })
        }
    }
}

/// Initialize the MCP server and start background services
pub async fn init_mcp_server(mcp_state: MCPState) {
    info!("Starting background initialization of MCP services");

    // Initialize the registry with database connection
    let registry = match crate::registry::ToolRegistry::new() {
        Ok(registry) => registry,
        Err(e) => {
            error!("Failed to initialize registry: {}", e);
            return;
        }
    };

    // Get all tools from database
    let tools = match registry.get_all_tools() {
        Ok(tools) => tools,
        Err(e) => {
            error!("Failed to get tools from database: {}", e);
            return;
        }
    };

    // Update the state with the new registry
    {
        let mut current_registry = mcp_state.tool_registry.write().await;
        *current_registry = registry;

        // Drop the write lock before trying to restart tools
        drop(current_registry);

        // Restart enabled tools
        for (tool_id_str, metadata) in tools {
            if metadata.enabled {
                info!("Found enabled tool: {}", tool_id_str);
                match mcp_state.restart_tool(&tool_id_str).await {
                    Ok(()) => {
                        info!("Successfully spawned process for tool: {}", tool_id_str);
                    }
                    Err(e) => {
                        error!("Failed to spawn process for tool {}: {}", tool_id_str, e);
                    }
                }
            }
        }
    }

    // Start the process monitor
    let mcp_state_arc = std::sync::Arc::new(mcp_state);
    mcp_state_arc.clone().start_process_monitor();
}

/// Get all server data in a single function to avoid multiple locks
pub async fn get_all_server_data(mcp_state: &MCPState) -> Result<Value, String> {
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
            let process_running = {
                let process_manager = mcp_state.process_manager.read().await;
                process_manager
                    .processes
                    .get(&id)
                    .is_some_and(|p| p.is_some())
            };
            obj.insert("process_running".to_string(), json!(process_running));

            // Add server tool count if the process is running
            if process_running {
                let server_tool_count = {
                    let server_tools = mcp_state.server_tools.read().await;
                    server_tools
                        .get(&id)
                        .map(|tools| tools.len())
                        .unwrap_or(0)
                };
                obj.insert("server_tool_count".to_string(), json!(server_tool_count));
            }
        }

        tools.push(tool_value);
    }

    // Get all server tools
    let mut server_tools_map = HashMap::new();
    {
        let server_tools = mcp_state.server_tools.read().await;
        for (server_id, tools) in &*server_tools {
            server_tools_map.insert(server_id.clone(), tools.clone());
        }
    }

    Ok(json!({
        "tools": tools,
        "server_tools": server_tools_map
    }))
}
