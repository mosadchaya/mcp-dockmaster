use std::{collections::HashMap, time::Duration};

use log::{error, info, warn};
use serde_json::{json, Value};
use tokio::process::Child;

use crate::{
    mcp_proxy::{discover_server_tools, execute_server_tool, kill_process, spawn_process},
    mcp_state::MCPState,
    models::models::Tool,
    DatabaseManager, MCPError,
};

#[derive(Default)]
pub struct ToolRegistry {
    pub tools: HashMap<String, Tool>,
    pub processes: HashMap<String, Option<Child>>,
    pub server_tools: HashMap<String, Vec<Value>>,
    pub process_ios: HashMap<String, (tokio::process::ChildStdin, tokio::process::ChildStdout)>,
}

impl ToolRegistry {
    /// Kill all running processes
    pub async fn kill_all_processes(&mut self) {
        for (tool_id, process_opt) in self.processes.iter_mut() {
            if let Some(process) = process_opt {
                if let Err(e) = process.kill().await {
                    error!("Failed to kill process for tool {}: {}", tool_id, e);
                } else {
                    info!("Killed process for tool {}", tool_id);
                }
            }
        }
    }

    // Start a background task that periodically checks if processes are running
    pub fn start_process_monitor(mcp_state: MCPState) {
        info!("Starting process monitor task");
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30)); // Check every 30 seconds

            loop {
                interval.tick().await;

                // Check all processes
                let registry_clone = mcp_state.tool_registry.clone();
                let tools_to_restart = {
                    let registry = registry_clone.read().await;
                    let mut to_restart = Vec::new();

                    for (id, tool) in &registry.tools {
                        if tool.enabled {
                            // Check if process is running
                            let process_running =
                                registry.processes.get(id).is_some_and(|p| p.is_some());

                            if !process_running {
                                warn!("Process for tool {} is not running but should be. Will attempt restart.", id);
                                to_restart.push(id.clone());
                            }
                        }
                    }

                    to_restart
                };

                // Restart any processes that should be running but aren't
                for tool_id in tools_to_restart {
                    info!("Attempting to restart tool: {}", tool_id);
                    let mut registry = mcp_state.tool_registry.write().await;
                    if let Err(e) = registry.restart_tool(&tool_id).await {
                        error!("Failed to restart tool {}: {}", tool_id, e);
                    } else {
                        info!("Successfully restarted tool: {}", tool_id);
                    }
                }
            }
        });
    }

    /// Execute a tool on a server
    pub async fn execute_tool(
        &mut self,
        server_id: &str,
        tool_id: &str,
        parameters: Value,
    ) -> Result<Value, MCPError> {
        execute_server_tool(server_id, tool_id, parameters, self).await
    }

    /// Initialize the MCP state from the database
    pub async fn init_mcp_state() -> MCPState {
        let mcp_state = MCPState::default();

        let db_manager = match DatabaseManager::new() {
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
                *registry = registry_data;
                info!("Successfully loaded MCP state from database");
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

            // Create clones of the data we need to save
            let tools_clone = registry.tools.clone();
            let server_tools_clone = registry.server_tools.clone();

            // Create a temporary registry with just the data we need
            ToolRegistry {
                tools: tools_clone,
                server_tools: server_tools_clone,
                processes: HashMap::new(),
                process_ios: HashMap::new(),
            }
        };

        // Now save the cloned data to the database
        match DatabaseManager::new() {
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

    /// Restart a tool by its ID
    pub async fn restart_tool(&mut self, tool_id: &str) -> Result<(), String> {
        info!("Attempting to restart tool: {}", tool_id);

        // Check if the tool exists
        let tool_info = if let Some(tool) = self.tools.get(tool_id) {
            // Extract necessary information
            let tool_type = tool.tool_type.clone();
            let entry_point = tool.entry_point.clone().unwrap_or_default();

            info!(
                "Found tool {}: type={}, entry_point={}",
                tool_id, tool_type, entry_point
            );
            Some((tool_type, entry_point, tool.clone()))
        } else {
            error!("Tool with ID '{}' not found in registry", tool_id);
            None
        };

        if tool_info.is_none() {
            return Err(format!("Tool with ID '{}' not found", tool_id));
        }

        let (tool_type, entry_point, tool_data) = tool_info.unwrap();

        // Check if tool_type is empty
        if tool_type.is_empty() {
            error!("Missing tool_type for tool {}", tool_id);
            return Err(format!("Missing tool_type for tool {}", tool_id));
        }

        // Check if the process is already running
        let process_running = self.processes.get(tool_id).is_some_and(|p| p.is_some());

        if process_running {
            info!(
                "Tool {} is already running, killing process before restart",
                tool_id
            );

            // Get the process and kill it
            if let Some(Some(process)) = self.processes.get_mut(tool_id) {
                if let Err(e) = kill_process(process).await {
                    error!("Failed to kill process for tool {}: {}", tool_id, e);
                    return Err(format!("Failed to kill process: {}", e));
                }
                info!("Successfully killed process for tool {}", tool_id);
            }

            // Remove the process from the registry
            self.processes.insert(tool_id.to_string(), None);

            // Remove the process IOs
            self.process_ios.remove(tool_id);
        }

        // Check if the tool is enabled
        let is_enabled = tool_data.enabled;

        if !is_enabled {
            info!("Tool {} is disabled, not restarting", tool_id);
            return Ok(());
        }

        info!(
            "Tool {} is enabled and not running, starting process",
            tool_id
        );

        // Extract environment variables from the tool configuration
        let env_vars = if let Some(config) = &tool_data.config {
            if let Some(env_map) = &config.env {
                info!(
                    "Extracted {} environment variables for tool {}",
                    env_map.len(),
                    tool_id
                );
                for (key, value) in env_map {
                    info!(
                        "Setting environment variable for tool {}: {}={}",
                        tool_id, key, value
                    );
                }
                Some(env_map.clone())
            } else {
                info!("No environment variables found for tool {}", tool_id);
                None
            }
        } else {
            info!("No configuration found for tool {}", tool_id);
            None
        };

        // Get the configuration from the tool data
        let config_value = if let Some(configuration) = &tool_data.configuration {
            // Use the configuration directly
            info!("Using configuration from tool data for {}", tool_id);
            json!({
                "command": configuration.command,
                "args": configuration.args
            })
        } else if !entry_point.is_empty() {
            // If no configuration but entry_point exists, create a simple config
            info!(
                "Creating simple configuration with entry_point for {}",
                tool_id
            );
            json!({
                "command": entry_point
            })
        } else if let Some(config) = &tool_data.config {
            // Try to use config if it exists
            if let Some(command) = &config.command {
                info!("Using command from config for {}: {}", tool_id, command);
                json!({
                    "command": command,
                    "args": config.args.clone().unwrap_or_default()
                })
            } else {
                error!("Missing command in configuration for tool {}", tool_id);
                return Err(format!(
                    "Missing command in configuration for tool {}",
                    tool_id
                ));
            }
        } else {
            error!("Missing configuration for tool {}", tool_id);
            return Err(format!("Missing configuration for tool {}", tool_id));
        };

        // Spawn process based on tool type
        let spawn_result =
            spawn_process(&config_value, tool_id, &tool_type, env_vars.as_ref()).await;

        match spawn_result {
            Ok((process, stdin, stdout)) => {
                info!("Successfully spawned process for tool: {}", tool_id);
                self.processes.insert(tool_id.to_string(), Some(process));
                self.process_ios
                    .insert(tool_id.to_string(), (stdin, stdout));

                // Wait a moment for the server to start
                // We need to use a separate scope to avoid moving self
                {
                    // Release the lock during sleep
                    info!("Waiting for server to start for tool: {}", tool_id);
                    let sleep_future = tokio::time::sleep(Duration::from_secs(2));
                    sleep_future.await;
                }

                // Try to discover tools from this server
                match discover_server_tools(tool_id, self).await {
                    Ok(tools) => {
                        self.server_tools.insert(tool_id.to_string(), tools.clone());
                        info!(
                            "Successfully discovered {} tools for {}",
                            tools.len(),
                            tool_id
                        );
                    }
                    Err(e) => {
                        error!("Failed to discover tools from server {}: {}", tool_id, e);
                        // Continue even if discovery fails
                    }
                }

                Ok(())
            }
            Err(e) => {
                error!("Failed to spawn process for tool {}: {}", tool_id, e);
                Err(format!("Failed to spawn process: {}", e))
            }
        }
    }

    pub async fn init_mcp_server(mcp_state: MCPState) {
        info!("Starting background initialization of MCP services");

        // Use the existing initialize_mcp_state function which handles loading from DB and restarting tools
        let initialized_state = ToolRegistry::init_mcp_state().await;

        // Copy the initialized data to our existing state
        {
            let initialized_registry = initialized_state.tool_registry.read().await;
            let mut current_registry = mcp_state.tool_registry.write().await;

            // Copy the tools data
            current_registry.tools = initialized_registry.tools.clone();

            // For processes, we need special handling
            // Since we can't directly copy the processes, we'll use the restart_tool method on the registry
            // to respawn processes for tools that were running
            let tool_ids_to_restart: Vec<String> = initialized_registry
                .tools
                .iter()
                .filter_map(|(tool_id, metadata)| {
                    if metadata.enabled {
                        info!("Found enabled tool: {}", tool_id);
                        Some(tool_id.clone())
                    } else {
                        None
                    }
                })
                .collect();

            // Release the read lock before proceeding
            drop(initialized_registry);

            // Now restart each tool
            for tool_id in tool_ids_to_restart {
                info!("Respawning process for tool: {}", tool_id);
                match current_registry.restart_tool(&tool_id).await {
                    Ok(()) => {
                        info!("Successfully spawned process for tool: {}", tool_id);
                    }
                    Err(e) => {
                        error!("Failed to spawn process for tool {}: {}", tool_id, e);
                    }
                }
            }
        }
    }
}

// #[derive(Default)]
// pub struct ToolRegistry {
//     pub tools: HashMap<ToolId, ToolMetadata>,
//     pub processes: HashMap<ToolId, Option<ProcessManager>>,
//     pub server_tools: HashMap<ToolId, Vec<Value>>,
// }

// impl ToolRegistry {
//     pub async fn register_tool(
//         &mut self,
//         request: ToolRegistrationRequest,
//     ) -> MCPResult<ToolRegistrationResponse> {
//         info!("Starting registration of tool: {}", request.tool_name);

//         // Generate tool ID
//         let tool_id = ToolId::new(format!("tool_{}", self.tools.len() + 1));

//         // Create tool metadata
//         let metadata = ToolMetadata {
//             name: request.tool_name.clone(),
//             description: request.description.clone(),
//             tool_type: request.tool_type.clone(),
//             enabled: true,
//             configuration: request.configuration.clone(),
//             process_running: false,
//             tool_count: 0,
//         };

//         // Store the tool metadata
//         self.tools.insert(tool_id.clone(), metadata);

//         // Create empty tools list
//         self.server_tools.insert(tool_id.clone(), Vec::new());

//         // Extract environment variables if they exist
//         let env_vars = if let Some(auth) = &request.authentication {
//             if let Some(env) = auth.get("env") {
//                 let mut env_map = HashMap::new();
//                 if let Some(env_obj) = env.as_object() {
//                     for (key, value) in env_obj {
//                         if let Some(value_str) = value.as_str() {
//                             env_map.insert(key.clone(), value_str.to_string());
//                         }
//                     }
//                 }
//                 Some(env_map)
//             } else {
//                 None
//             }
//         } else {
//             None
//         };

//         // Get the configuration
//         let config = request
//             .configuration
//             .ok_or_else(|| MCPError::ConfigurationError("Configuration is required".to_string()))?;

//         // Spawn the process
//         let process_manager =
//             ProcessManager::new(&tool_id, &request.tool_type, &config, env_vars.as_ref()).await?;

//         // Store the process
//         self.processes
//             .insert(tool_id.clone(), Some(process_manager));

//         // Wait for server to initialize
//         tokio::time::sleep(Duration::from_secs(2)).await;

//         // Try to discover tools
//         match self.discover_tools(&tool_id).await {
//             Ok(tools) => {
//                 self.server_tools.insert(tool_id.clone(), tools);
//             }
//             Err(e) => {
//                 error!("Error discovering tools: {}", e);
//                 // Add default tool
//                 self.server_tools.insert(
//                     tool_id.clone(),
//                     vec![json!({
//                         "id": "main",
//                         "name": request.tool_name,
//                         "description": request.description
//                     })],
//                 );
//             }
//         }

//         Ok(ToolRegistrationResponse {
//             success: true,
//             message: "Tool registered successfully".to_string(),
//             tool_id: Some(tool_id),
//         })
//     }

//     pub async fn discover_tools(&mut self, tool_id: &ToolId) -> MCPResult<Vec<Value>> {
//         let process = self
//             .processes
//             .get_mut(tool_id)
//             .ok_or_else(|| MCPError::ServerNotFound(tool_id.to_string()))?
//             .as_mut()
//             .ok_or_else(|| MCPError::ServerNotFound(tool_id.to_string()))?;

//         let result = process.send_command("tools/list", json!({})).await?;

//         if let Some(tools) = result.as_array() {
//             Ok(tools.clone())
//         } else {
//             Ok(Vec::new())
//         }
//     }

//     pub async fn restart_all_tools(&mut self) -> MCPResult<Vec<MCPResult<()>>> {
//         // Collect tool IDs first
//         let tool_ids: Vec<_> = self.tools.keys().cloned().collect();

//         // Process each tool sequentially to avoid borrow checker issues
//         let mut results = Vec::new();
//         for id in tool_ids {
//             let result = tokio::time::timeout(Duration::from_secs(30), self.restart_tool(&id))
//                 .await
//                 .unwrap_or_else(|_| Err(MCPError::TimeoutError(id.to_string())));
//             results.push(result);
//         }

//         Ok(results)
//     }

//     pub async fn restart_tool(&mut self, tool_id: &ToolId) -> MCPResult<()> {
//         // Kill existing process if running
//         if let Some(Some(process)) = self.processes.get_mut(tool_id) {
//             process.kill().await?;
//         }

//         // Get tool metadata
//         let metadata = self
//             .tools
//             .get(tool_id)
//             .ok_or_else(|| MCPError::ToolError(format!("Tool not found: {}", tool_id)))?;

//         // Skip if disabled
//         if !metadata.enabled {
//             return Ok(());
//         }

//         // Get configuration
//         let config = metadata
//             .configuration
//             .as_ref()
//             .ok_or_else(|| MCPError::ConfigurationError("Configuration missing".to_string()))?;

//         // Spawn new process
//         let process_manager =
//             ProcessManager::new(tool_id, &metadata.tool_type, config, config.env.as_ref()).await?;

//         // Store new process
//         self.processes
//             .insert(tool_id.clone(), Some(process_manager));

//         // Wait for initialization
//         tokio::time::sleep(Duration::from_secs(2)).await;

//         // Rediscover tools
//         match self.discover_tools(tool_id).await {
//             Ok(tools) => {
//                 self.server_tools.insert(tool_id.clone(), tools);
//             }
//             Err(e) => {
//                 error!("Error rediscovering tools: {}", e);
//             }
//         }

//         Ok(())
//     }

//     pub async fn kill_all_processes(&mut self) -> MCPResult<()> {
//         for (tool_id, process_opt) in self.processes.iter_mut() {
//             if let Some(process) = process_opt {
//                 if let Err(e) = process.kill().await {
//                     error!("Failed to kill process for tool {}: {}", tool_id, e);
//                 }
//             }
//         }
//         Ok(())
//     }
// }

// #[cfg(test)]
// mod tests {
//     use crate::models::models::{ToolConfiguration, ToolType};

//     use super::*;

//     #[tokio::test]
//     async fn test_tool_registration() {
//         let mut registry = ToolRegistry::default();
//         let request = ToolRegistrationRequest {
//             tool_name: "test_tool".to_string(),
//             description: "Test tool".to_string(),
//             tool_type: ToolType::Node,
//             authentication: None,
//             configuration: Some(ToolConfiguration {
//                 command: "node".to_string(),
//                 args: Some(vec!["test.js".to_string()]),
//                 env: None,
//             }),
//             distribution: None,
//         };

//         let result = registry.register_tool(request).await;
//         assert!(result.is_ok());
//     }

//     // Add more tests...
// }
