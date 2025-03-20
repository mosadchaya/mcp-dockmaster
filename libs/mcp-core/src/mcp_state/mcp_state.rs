use crate::models::types::ServerToolInfo;
use crate::registry::server_registry::ServerRegistry;
use crate::types::ServerStatus;
use crate::utils::command::CommandWrappedInShellBuilder;
use crate::MCPError;
use log::{error, info};
use mcp_sdk_client::transport::stdio::StdioTransport;
use mcp_sdk_client::transport::stdio::StdioTransportHandle;
use mcp_sdk_client::{
    ClientCapabilities, ClientInfo, McpClient, McpClientTrait, McpService, Transport,
};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

/// Type alias for a transport that uses StdioTransportHandle
pub type StdioTransportType = Arc<dyn Transport<Handle = StdioTransportHandle> + Send + Sync>;

/// Type alias for McpClient trait objects
pub type McpClientType = Arc<dyn McpClientTrait + Send + Sync>;

/// MCPState: the main service layer
///
/// This module coordinates database operations, process management, and discovered tools.
/// It serves as the central orchestration layer that connects the database (ServerRegistry),
/// process management (ProcessManager), and JSON-RPC operations (mcp_proxy).
#[derive(Clone)]
pub struct MCPState {
    pub tool_registry: Arc<RwLock<ServerRegistry>>,
    pub server_tools: Arc<RwLock<HashMap<String, Vec<ServerToolInfo>>>>,
    pub mcp_clients: Arc<RwLock<HashMap<String, MCPClient>>>,
    pub are_tools_hidden: Arc<RwLock<bool>>,
}

#[derive(Clone)]
pub struct MCPClient {
    pub client: McpClientType,
    pub transport: StdioTransportType,
    pub server_status: ServerStatus,
}

impl MCPState {
    pub fn new(
        tool_registry: Arc<RwLock<ServerRegistry>>,
        server_tools: Arc<RwLock<HashMap<String, Vec<ServerToolInfo>>>>,
        mcp_clients: Arc<RwLock<HashMap<String, MCPClient>>>,
    ) -> Self {
        // Initialize with default value
        let are_tools_hidden = Arc::new(RwLock::new(false));

        Self {
            tool_registry,
            server_tools,
            mcp_clients,
            are_tools_hidden,
        }
    }

    /// Initialize the state from the database
    pub async fn init_state(&self) -> Result<(), String> {
        // Load the tool visibility state from the database
        let registry = self.tool_registry.read().await;
        match registry.get_setting("tools_hidden") {
            Ok(value) => {
                let hidden = value == "true";
                let mut are_tools_hidden = self.are_tools_hidden.write().await;
                *are_tools_hidden = hidden;
                info!("Loaded tools visibility state from database: {}", hidden);
            }
            Err(_) => {
                // Setting doesn't exist yet, use the default value (false)
                info!("No tools visibility state found in database, using default (visible)");
            }
        }

        Ok(())
    }

    /// Kill all running processes, attempting to kill each process even if some fail
    pub async fn kill_all_processes(&self) -> Result<(), String> {
        let server_ids: Vec<String> = self.mcp_clients.read().await.keys().cloned().collect();
        let mut errors = Vec::new();

        for server_id in server_ids {
            if let Err(e) = self.kill_process(&server_id).await {
                errors.push(format!("Failed to kill process {}: {}", server_id, e));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.join("\n"))
        }
    }

    /// Kill a process by its ID
    pub async fn kill_process(&self, server_id: &str) -> Result<(), String> {
        // First check if the client exists
        let client_exists = {
            let mcp_clients = self.mcp_clients.read().await;
            mcp_clients.contains_key(server_id)
        };

        if !client_exists {
            return Err(format!("No client found for server: {}", server_id));
        }

        // Get a clone of the client for closing the transport
        let mcp_client_clone = {
            let mcp_clients = self.mcp_clients.read().await;
            mcp_clients.get(server_id).cloned()
        };

        if let Some(mcp_client) = mcp_client_clone {
            // Close the transport
            let _ = mcp_client.transport.close().await;

            // Update the server status to Stopped in the map and remove the client
            let mut mcp_clients = self.mcp_clients.write().await;
            mcp_clients.remove(server_id);
            info!("Removed client for {}", server_id);

            // Remove the server tools
            let _ = self.server_tools.write().await.remove(server_id);

            Ok(())
        } else {
            Err(format!("No client found for server: {}", server_id))
        }
    }

    /// Execute a tool on a server
    pub async fn execute_tool(
        &self,
        server_id: &str,
        tool_id: &str,
        parameters: Value,
    ) -> Result<Value, MCPError> {
        let mcp_client = self.mcp_clients.read().await.get(server_id).cloned();
        if let Some(mcp_client) = mcp_client {
            info!(
                "[execute tool] Successfully got client for server: {}",
                server_id
            );
            let result = mcp_client
                .client
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

        // Check if the client already exists
        let client_exists = {
            let mcp_clients = self.mcp_clients.read().await;
            mcp_clients.contains_key(server_id)
        };

        if client_exists {
            // First kill the existing process
            if let Err(e) = self.kill_process(server_id).await {
                error!("Failed to kill existing process during restart: {}", e);
                return Err(format!("Failed to kill existing process: {}", e));
            }
            info!(
                "Successfully killed existing process for server: {}",
                server_id
            );
        }

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
                    .filter_map(|(k, tool_env)| tool_env.default.clone().map(|v| (k.clone(), v)))
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

        let mut envs = env_vars.unwrap_or_default();
        let mut sustituted_args = Vec::new();
        for v in config_value["args"].as_array().unwrap_or(&vec![]) {
            let args_key = v.as_str().unwrap();
            let adapted_args_key = args_key.replace("$", "");

            let args_value = if args_key.starts_with("$") && envs.contains_key(&adapted_args_key) {
                let args_value_from_env = envs.get(&adapted_args_key).unwrap().clone();
                envs.remove(&adapted_args_key);
                args_value_from_env
            } else {
                args_key.to_string()
            };
            sustituted_args.push(args_value);
        }
        let (adapted_program, adapted_args, adapted_envs) =
            CommandWrappedInShellBuilder::wrap_in_shell_as_values(
                config_value["command"].as_str().unwrap(),
                Some(sustituted_args.iter().map(|s| s.as_str())),
                Some(envs),
            );

        let transport = StdioTransport::new(adapted_program, adapted_args, adapted_envs);

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

        self.mcp_clients.write().await.insert(
            server_id.to_string(),
            MCPClient {
                client: Arc::new(client) as McpClientType,
                transport: Arc::new(transport) as StdioTransportType,
                server_status: ServerStatus::Running,
            },
        );

        match self.discover_server_tools(server_id).await {
            Ok(tools) => {
                info!(
                    "Successfully discovered {} tools for {}",
                    tools.len(),
                    server_id
                );
            }
            Err(e) => {
                error!("Failed to discover tools for server: {}", e);
            }
        }

        info!("Successfully initialized client for server: {}", server_id);

        Ok(())
    }

    /// Get the current tool visibility state
    pub async fn are_tools_hidden(&self) -> bool {
        let are_tools_hidden = self.are_tools_hidden.read().await;
        *are_tools_hidden
    }

    /// Set the tool visibility state
    pub async fn set_tools_hidden(&self, hidden: bool) -> Result<(), String> {
        // Update the in-memory state
        let mut are_tools_hidden = self.are_tools_hidden.write().await;
        *are_tools_hidden = hidden;
        info!(
            "Tools visibility set to: {}",
            if hidden { "hidden" } else { "visible" }
        );

        // Persist the state to the database
        let registry = self.tool_registry.read().await;
        registry.save_setting("tools_hidden", if hidden { "true" } else { "false" })
    }

    pub async fn discover_server_tools(
        &self,
        server_id: &str,
    ) -> Result<Vec<ServerToolInfo>, String> {
        info!(
            "[discover_tools] Starting discovery for server: {}",
            server_id
        );

        let mcp_client = self.mcp_clients.read().await.get(server_id).cloned();
        if let Some(mcp_client) = mcp_client {
            info!(
                "[discover tools] Successfully got client for server: {}",
                server_id
            );

            match mcp_client.server_status {
                ServerStatus::Running => {
                    info!("Server status is Running, about to call list_tools");

                    let list_tools = match mcp_client.client.list_tools(None).await {
                        Ok(result) => {
                            info!("mcp_client: list_tools call succeeded");
                            result
                        }
                        Err(e) => {
                            error!("mcp_client: list_tools call failed: {}", e);
                            return Err(e.to_string());
                        }
                    };

                    let tools = list_tools.tools;
                    info!(
                        "Successfully discovered {} tools for {}",
                        tools.len(),
                        server_id
                    );

                    // Save the tools to the database
                    let registry = self.tool_registry.read().await;
                    let mut tools_info = Vec::new();

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
                        tools_info.push(tool_info);
                    }

                    // Save the tools to the server_tools map
                    let mut server_tools = self.server_tools.write().await;
                    server_tools.insert(server_id.to_string(), tools_info.clone());

                    Ok(tools_info)
                }
                _ => Err(format!("Server {} is not running", server_id)),
            }
        } else {
            info!("No client found for server: {}", server_id);
            Err(format!("No client found for server: {}", server_id))
        }
    }
}
