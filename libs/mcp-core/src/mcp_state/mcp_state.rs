use crate::models::types::{ServerStatus, ServerToolInfo};
use crate::registry::server_registry::ServerRegistry;
use crate::MCPError;
use async_trait::async_trait;
use log::{error, info, warn};
use mcp_client::transport::stdio::StdioTransportHandle;
use mcp_client::{
    ClientCapabilities, ClientInfo, McpClient, McpClientTrait, McpService, StdioTransport,
    Transport,
};
use mcp_core::Tool;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

use super::process_manager::ProcessManager;

/// MCPState: the main service layer
///
/// This module coordinates database operations, process management, and discovered tools.
/// It serves as the central orchestration layer that connects the database (ServerRegistry),
/// process management (ProcessManager), and JSON-RPC operations (mcp_proxy).
#[derive(Clone)]
pub struct MCPState {
    pub tool_registry: Arc<RwLock<ServerRegistry>>,
    pub process_manager: Arc<RwLock<ProcessManager>>,
    pub clients: Arc<RwLock<HashMap<String, Arc<dyn McpClientTrait + Send + Sync>>>>,
    pub transport_manager: Arc<
        RwLock<HashMap<String, Arc<dyn Transport<Handle = StdioTransportHandle> + Send + Sync>>>,
    >,
    pub server_tools: Arc<RwLock<HashMap<String, Vec<ServerToolInfo>>>>,
}

impl MCPState {
    pub fn new(
        tool_registry: Arc<RwLock<ServerRegistry>>,
        process_manager: Arc<RwLock<ProcessManager>>,
        server_tools: Arc<RwLock<HashMap<String, Vec<ServerToolInfo>>>>,
        clients: Arc<RwLock<HashMap<String, Arc<dyn McpClientTrait + Send + Sync>>>>,
        transport_manager: Arc<
            RwLock<
                HashMap<String, Arc<dyn Transport<Handle = StdioTransportHandle> + Send + Sync>>,
            >,
        >,
    ) -> Self {
        Self {
            tool_registry,
            process_manager,
            server_tools,
            clients,
            transport_manager,
        }
    }

    /// Kill all running processes
    pub async fn kill_all_processes(&self) {
        for transport in self.transport_manager.read().await.values() {
            transport.close().await;
        }
    }

    /// Execute a tool on a server
    pub async fn execute_tool(
        &self,
        server_id: &str,
        tool_id: &str,
        parameters: Value,
    ) -> Result<Value, MCPError> {
        let client = self.clients.read().await.get(server_id).cloned();
        if let Some(client) = client {
            info!("Successfully got client for server: {}", server_id);
            let result = client
                .call_tool(tool_id, parameters)
                .await
                .map_err(|e| MCPError::ToolExecutionError(e.to_string()))?;

            // Convert the result to a Value
            let content_value = serde_json::to_value(result.content)
                .map_err(|e| MCPError::SerializationError(e.to_string()))?;

            Ok(content_value)
        } else {
            Err(MCPError::ServerNotFound(format!(
                "No client found for server: {}",
                server_id
            )))
        }
    }

    /// Restart a server by its ID
    pub async fn restart_server(&self, server_id: &str) -> Result<(), String> {
        info!("Attempting to restart server: {}", server_id);

        // Get tool from database
        let server_data = {
            let registry = self.tool_registry.read().await;
            registry.get_server(server_id)?
        };

        // Check if tools_type is empty
        if server_data.tools_type.is_empty() {
            error!("Missing tools_type for server {}", server_id);
            return Err(format!("Missing tools_type for server {}", server_id));
        }

        let client = self.clients.read().await.get(server_id).cloned();
        if let Some(client) = client {
            info!("Successfully got client for server: {}", server_id);
            return Ok(());
        } else {
            // Check if the tool is enabled
            if !server_data.enabled {
                info!("Server {} is disabled, not restarting", server_id);
                return Ok(());
            }

            // Extract environment variables from the tool configuration
            let env_vars = if let Some(configuration) = &server_data.configuration {
                if let Some(env_map) = &configuration.env {
                    info!(
                        "Extracted {} environment variables for server {}",
                        env_map.len(),
                        server_id
                    );
                    // Convert ToolEnvironment -> just the defaults
                    let simple_env_map: HashMap<String, String> = env_map
                        .iter()
                        .filter_map(|(k, tool_env)| {
                            tool_env.default.clone().map(|v| (k.clone(), v))
                        })
                        .collect();
                    Some(simple_env_map)
                } else {
                    info!("No environment variables found for server {}", server_id);
                    None
                }
            } else {
                info!("No configuration found for server {}", server_id);
                None
            };

            // Get the configuration from the tool data
            let config_value = if let Some(configuration) = &server_data.configuration {
                info!("Using configuration from server data for {}", server_id);
                json!({
                    "command": configuration.command,
                    "args": configuration.args
                })
            } else if !server_data
                .entry_point
                .clone()
                .unwrap_or_default()
                .is_empty()
            {
                info!(
                    "Creating simple configuration with entry_point for {}",
                    server_id
                );
                json!({
                    "command": server_data.entry_point
                })
            } else {
                error!("Missing configuration for server {}", server_id);
                return Err(format!("Missing configuration for server {}", server_id));
            };

            let transport = StdioTransport::new(
                config_value["command"].as_str().unwrap().to_string(),
                config_value["args"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_str().unwrap().to_string())
                    .collect(),
                env_vars.unwrap_or_default(),
            );

            let transport_handle = match transport.start().await {
                Ok(handle) => handle,
                Err(e) => return Err(format!("Failed to start transport: {}", e)),
            };

            // Create the service with a timeout of 300 seconds
            let service = McpService::with_timeout(transport_handle, Duration::from_secs(300));

            // Create and initialize the client
            let mut client = McpClient::new(service);

            let client_info = ClientInfo {
                name: "mcp-dockmaster".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            };

            if let Err(e) = client
                .initialize(client_info, ClientCapabilities::default())
                .await
            {
                return Err(format!("Failed to initialize client: {}", e));
            }

            self.clients
                .write()
                .await
                .insert(server_id.to_string(), Arc::new(client));

            self.transport_manager
                .write()
                .await
                .insert(server_id.to_string(), Arc::new(transport));

            let _ = self.discover_server_tools(server_id).await;

            info!("Successfully initialized client for server: {}", server_id);

            return Ok(());
        }
    }

    /// Discover tools from a server
    pub async fn discover_server_tools(&self, server_id: &str) -> Result<Vec<Tool>, String> {
        let client = self.clients.read().await.get(server_id).cloned();
        if let Some(client) = client {
            info!("Successfully got client for server: {}", server_id);
            let list_tools = client.list_tools(None).await.map_err(|e| e.to_string())?;
            let tools = list_tools.tools;
            info!(
                "Successfully discovered {} tools for {}",
                tools.len(),
                server_id
            );

            // Save the tools to the database
            let registry = self.tool_registry.read().await;
            for tool in &tools {
                let tool_info = ServerToolInfo {
                    id: tool.name.clone(),
                    name: tool.name.clone(),
                    description: tool.description.clone(),
                    input_schema: if tool.input_schema.is_object() {
                        serde_json::from_value(tool.input_schema.clone()).ok()
                    } else {
                        None
                    },
                    proxy_id: Some(server_id.to_string()),
                    server_id: server_id.to_string(),
                    is_active: true,
                };
                info!("Saving tool info to database: {:?}", tool_info);
                if let Err(e) = registry.save_server_tool(&tool_info) {
                    error!("Failed to save server tool to database: {}", e);
                }
            }

            return Ok(tools);
        } else {
            info!("No client found for server: {}", server_id);
            return Err(format!("No client found for server: {}", server_id));
        }
    }
}

#[async_trait]
pub trait McpStateProcessMonitor {
    async fn start_process_monitor(self);
}

#[async_trait]
impl McpStateProcessMonitor for Arc<RwLock<MCPState>> {
    // Start a background task that periodically checks if processes are running
    async fn start_process_monitor(self) {
        info!("Starting process monitor task");

        let self_clone_read_guard = self.read().await;
        let tool_registry = self_clone_read_guard.tool_registry.clone();
        let process_manager = self_clone_read_guard.process_manager.clone();
        drop(self_clone_read_guard);

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30));

            loop {
                interval.tick().await;

                // Get all tools from database
                let tools_result = {
                    let registry = tool_registry.read().await;
                    registry.get_all_servers()
                };

                let tools = match tools_result {
                    Ok(tools) => tools,
                    Err(e) => {
                        error!("Failed to get tools from database: {}", e);
                        continue;
                    }
                };

                // Check all enabled tools
                for (id_str, tool) in tools {
                    if tool.enabled {
                        // Check if process is running
                        let process_running = {
                            let process_manager = process_manager.read().await;
                            process_manager
                                .processes
                                .get(&id_str)
                                .is_some_and(|(p, status)| {
                                    matches!(status, ServerStatus::Running) && p.is_some()
                                })
                        };

                        if !process_running {
                            warn!("Process for tool {} is not running but should be. Will attempt restart.", id_str);
                            let self_clone_write_guard = self.write().await;
                            if let Err(e) = self_clone_write_guard.restart_server(&id_str).await {
                                error!("Failed to restart tool {}: {}", id_str, e);
                            } else {
                                info!("Successfully restarted tool: {}", id_str);
                            }
                        }
                    }
                }
            }
        });
    }
}
