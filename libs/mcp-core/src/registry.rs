use std::{collections::HashMap, time::Duration};

use log::{error, info, warn};
use serde_json::{json, Value};
use tokio::process::Child;

use crate::{
    mcp_proxy::{discover_server_tools, execute_server_tool, kill_process, spawn_process},
    mcp_state::MCPState,
    models::models::Tool,
    DBManager, MCPError,
};

pub struct ToolRegistry {
    pub processes: HashMap<String, Option<Child>>,
    pub server_tools: HashMap<String, Vec<Value>>,
    pub process_ios: HashMap<String, (tokio::process::ChildStdin, tokio::process::ChildStdout)>,
    db: DBManager,
}

impl ToolRegistry {
    pub fn new() -> Result<Self, String> {
        let db = DBManager::new()?;
        Ok(Self {
            processes: HashMap::new(),
            server_tools: HashMap::new(),
            process_ios: HashMap::new(),
            db,
        })
    }

    /// Get a tool by ID
    pub fn get_tool(&self, tool_id: &str) -> Result<Tool, String> {
        self.db.get_tool(tool_id)
    }

    /// Get all tools
    pub fn get_all_tools(&self) -> Result<HashMap<String, Tool>, String> {
        self.db.get_all_tools()
    }

    /// Save or update a tool
    pub fn save_tool(&self, tool_id: &str, tool: &Tool) -> Result<(), String> {
        self.db.save_tool(tool_id, tool)
    }

    /// Delete a tool
    pub fn delete_tool(&self, tool_id: &str) -> Result<(), String> {
        self.db.delete_tool(tool_id)
    }

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

    /// Execute a tool on a server
    pub async fn execute_tool(
        &mut self,
        server_id: &str,
        tool_id: &str,
        parameters: Value,
    ) -> Result<Value, MCPError> {
        execute_server_tool(server_id, tool_id, parameters, self).await
    }

    /// Restart a tool by its ID
    pub async fn restart_tool(&mut self, tool_id: &str) -> Result<(), String> {
        info!("Attempting to restart tool: {}", tool_id);

        // Get tool from database
        let tool_data = self.get_tool(tool_id)?;

        // Check if tool_type is empty
        if tool_data.tool_type.is_empty() {
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
        if !tool_data.enabled {
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
            info!("Using configuration from tool data for {}", tool_id);
            json!({
                "command": configuration.command,
                "args": configuration.args
            })
        } else if !tool_data.entry_point.clone().unwrap_or_default().is_empty() {
            info!(
                "Creating simple configuration with entry_point for {}",
                tool_id
            );
            json!({
                "command": tool_data.entry_point
            })
        } else if let Some(config) = &tool_data.config {
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
        let spawn_result = spawn_process(
            &config_value,
            tool_id,
            &tool_data.tool_type,
            env_vars.as_ref(),
        )
        .await;

        match spawn_result {
            Ok((process, stdin, stdout)) => {
                info!("Successfully spawned process for tool: {}", tool_id);
                self.processes.insert(tool_id.to_string(), Some(process));
                self.process_ios
                    .insert(tool_id.to_string(), (stdin, stdout));

                // Wait a moment for the server to start
                {
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

    // Start a background task that periodically checks if processes are running
    pub fn start_process_monitor(mcp_state: MCPState) {
        info!("Starting process monitor task");
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30));

            loop {
                interval.tick().await;

                let registry_clone = mcp_state.tool_registry.clone();
                let mut registry = registry_clone.write().await;

                // Get all tools from database
                let tools = match registry.get_all_tools() {
                    Ok(tools) => tools,
                    Err(e) => {
                        error!("Failed to get tools from database: {}", e);
                        continue;
                    }
                };

                // Check all enabled tools
                for (id, tool) in tools {
                    if tool.enabled {
                        // Check if process is running
                        let process_running =
                            registry.processes.get(&id).is_some_and(|p| p.is_some());

                        if !process_running {
                            warn!("Process for tool {} is not running but should be. Will attempt restart.", id);
                            if let Err(e) = registry.restart_tool(&id).await {
                                error!("Failed to restart tool {}: {}", id, e);
                            } else {
                                info!("Successfully restarted tool: {}", id);
                            }
                        }
                    }
                }
            }
        });
    }
}

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
