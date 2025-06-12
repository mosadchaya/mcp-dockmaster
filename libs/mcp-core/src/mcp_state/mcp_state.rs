use crate::mcp_state::tokio_child_process_custom::TokioChildProcessCustom;
use crate::registry::server_registry::ServerRegistry;
use crate::types::ServerStatus;
use crate::types::ServerToolInfo;
use crate::utils::command::CommandWrappedInShellBuilder;
use log::{error, info};
use rmcp::model::{
    CallToolResult, ClientCapabilities, ClientInfo, Implementation, InitializeRequestParam,
};
use rmcp::service::RunningService;
use rmcp::{RoleClient, ServiceError, ServiceExt};
use serde_json::{json, Map, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

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
    pub client: Arc<RunningService<RoleClient, InitializeRequestParam>>,
    // pub transport: StdioTransportType,
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
                info!("Loaded tools visibility state from database: {hidden}");
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
                errors.push(format!("Failed to kill process {server_id}: {e}"));
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
            return Err(format!("No client found for server: {server_id}"));
        }

        let mut mcp_clients = self.mcp_clients.write().await;
        let mcp_client_option = mcp_clients.remove(server_id);
        info!("Removed client for {server_id}");
        if let Some(mcp_client_instance) = mcp_client_option {
            let owned_service = Arc::try_unwrap(mcp_client_instance.client)
                .map_err(|arc_still_shared| {
                    format!(
                        "Failed to obtain exclusive ownership of client for server {}. Cannot cancel. Strong count: {}",
                            server_id,
                        Arc::strong_count(&arc_still_shared)
                    )
                })?;

            // Call cancel on the now owned_service.
            // The error from cancel() is logged, but kill_process doesn't fail due to it.
            if let Err(e) = owned_service.cancel().await {
                error!("Cancellation error for client {server_id}: {e}");
            }
        }
        // Remove the server tools
        let _ = self.server_tools.write().await.remove(server_id);
        Ok(())
    }

    /// Execute a tool on a server
    pub async fn execute_tool(
        &self,
        server_id: &str,
        tool_id: &str,
        parameters: Map<String, Value>,
    ) -> Result<CallToolResult, ServiceError> {
        let mcp_client = self.mcp_clients.read().await.get(server_id).cloned();
        if let Some(mcp_client) = mcp_client {
            info!("[execute tool] Successfully got client for server: {server_id}");
            let result = mcp_client
                .client
                .call_tool(rmcp::model::CallToolRequestParam {
                    name: tool_id.to_string().into(),
                    arguments: Some(parameters),
                })
                .await?;

            Ok(result)
        } else {
            Err(ServiceError::TransportClosed)
        }
    }

    /// Restart a server by its ID
    pub async fn restart_server(&self, server_id: &str) -> Result<(), String> {
        info!("Attempting to restart server: {server_id}");

        // Get tool from database
        let server_data = {
            let registry = self.tool_registry.read().await;
            registry.get_server(server_id)?
        };

        // Check if tools_type is empty
        if server_data.tools_type.is_empty() {
            error!("Missing tools_type for server {server_id}");
            return Err(format!("Missing tools_type for server {server_id}"));
        }

        // Additional validation for custom servers
        if matches!(server_data.server_type, crate::models::types::ServerType::Local | crate::models::types::ServerType::Custom) {
            // Validate that custom servers have proper configuration
            if server_data.configuration.is_none() && server_data.executable_path.is_none() {
                error!("Custom/Local server {} requires either configuration or executable_path", server_id);
                return Err(format!("Custom/Local server {} requires either configuration or executable_path", server_id));
            }
            
            // For custom servers with working directory, validate it exists
            if let Some(working_dir) = &server_data.working_directory {
                match crate::validation::resolve_template_variables(working_dir) {
                    Ok(resolved_dir) => {
                        let path = std::path::Path::new(&resolved_dir);
                        if !path.exists() {
                            error!("Working directory does not exist for server {}: {}", server_id, resolved_dir);
                            return Err(format!("Working directory does not exist: {}", resolved_dir));
                        }
                        if !path.is_dir() {
                            error!("Working directory path is not a directory for server {}: {}", server_id, resolved_dir);
                            return Err(format!("Working directory path is not a directory: {}", resolved_dir));
                        }
                    },
                    Err(e) => {
                        error!("Failed to resolve working directory template for server {}: {}", server_id, e);
                        return Err(format!("Failed to resolve working directory template: {}", e));
                    }
                }
            }
        }

        // Check if the client already exists
        let client_exists = {
            let mcp_clients = self.mcp_clients.read().await;
            mcp_clients.contains_key(server_id)
        };

        if client_exists {
            // First kill the existing process
            if let Err(e) = self.kill_process(server_id).await {
                error!("Failed to kill existing process during restart: {e}");
                return Err(format!("Failed to kill existing process: {e}"));
            }
            info!("Successfully killed existing process for server: {server_id}");
        }

        // Check if the tool is enabled
        if !server_data.enabled {
            info!("Server {server_id} is disabled, not restarting");
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
                // Convert ToolEnvironment -> resolved values
                let mut simple_env_map: HashMap<String, String> = HashMap::new();
                for (k, tool_env) in env_map.iter() {
                    if let Some(value) = &tool_env.default {
                        // Resolve template variables in environment values
                        match crate::validation::resolve_template_variables(value) {
                            Ok(resolved_value) => {
                                if resolved_value != *value {
                                    info!("Resolved environment variable '{}' template '{}' to '{}'", k, value, resolved_value);
                                }
                                simple_env_map.insert(k.clone(), resolved_value);
                            },
                            Err(e) => {
                                error!("Failed to resolve environment variable '{}' template '{}': {}", k, value, e);
                                // Use original value as fallback
                                simple_env_map.insert(k.clone(), value.clone());
                            }
                        }
                    }
                }
                Some(simple_env_map)
            } else {
                info!("No environment variables found for server {server_id}");
                None
            }
        } else {
            info!("No configuration found for server {server_id}");
            None
        };

        // Get the configuration from the tool data, with support for custom servers with executable_path
        let config_value = if let Some(configuration) = &server_data.configuration {
            info!("Using configuration from server data for {server_id}");
            json!({
                "command": configuration.command,
                "args": configuration.args
            })
        } else if let Some(executable_path) = &server_data.executable_path {
            // Handle custom servers with executable_path
            info!("Creating configuration from executable_path for custom server {server_id}");
            
            // Resolve template variables in executable path
            let resolved_executable = match crate::validation::resolve_template_variables(executable_path) {
                Ok(resolved) => {
                    if resolved != *executable_path {
                        info!("Resolved executable path template '{}' to '{}'", executable_path, resolved);
                    }
                    resolved
                },
                Err(e) => {
                    error!("Failed to resolve executable path template '{}': {}", executable_path, e);
                    return Err(format!("Failed to resolve executable path template: {}", e));
                }
            };
            
            // Determine command based on server type and tools_type
            let command = match server_data.tools_type.as_str() {
                "node" => "node",
                "python" => "python3",
                _ => &resolved_executable, // For custom runtime, use the executable directly
            };
            
            // For node/python runtimes, executable_path becomes first argument
            let args = if matches!(server_data.tools_type.as_str(), "node" | "python") {
                vec![json!(resolved_executable)]
            } else {
                vec![]
            };
            
            json!({
                "command": command,
                "args": args
            })
        } else if !server_data
            .entry_point
            .clone()
            .unwrap_or_default()
            .is_empty()
        {
            info!("Creating simple configuration with entry_point for {server_id}");
            json!({
                "command": server_data.entry_point
            })
        } else {
            error!("Missing configuration and executable_path for server {server_id}");
            return Err(format!("Missing configuration and executable_path for server {server_id}"));
        };

        let mut envs = env_vars.unwrap_or_default();
        let mut substituted_args = Vec::new();
        for v in config_value["args"].as_array().unwrap_or(&vec![]) {
            let args_key = v.as_str().unwrap();
            
            // First try template variable resolution (handles $HOME, $USER, etc.)
            let args_value = match crate::validation::resolve_template_variables(args_key) {
                Ok(resolved) => {
                    // If resolved value is different, template was applied
                    if resolved != args_key {
                        info!("Resolved argument template '{}' to '{}'", args_key, resolved);
                        resolved
                    } else {
                        // Fall back to legacy environment variable substitution
                        let adapted_args_key = args_key.replace("$", "");
                        if args_key.starts_with("$") && envs.contains_key(&adapted_args_key) {
                            let args_value_from_env = envs.get(&adapted_args_key).unwrap().clone();
                            envs.remove(&adapted_args_key);
                            args_value_from_env
                        } else {
                            args_key.to_string()
                        }
                    }
                },
                Err(e) => {
                    error!("Failed to resolve argument template '{}': {}", args_key, e);
                    args_key.to_string()
                }
            };
            substituted_args.push(args_value);
        }

        let mut command_builder =
            CommandWrappedInShellBuilder::new(config_value["command"].as_str().unwrap());
        command_builder.args(substituted_args.iter().map(|s| s.as_str()));
        command_builder.envs(envs);
        
        // Set working directory for custom servers if specified
        let mut command = command_builder.build();
        if let Some(working_dir) = &server_data.working_directory {
            // Resolve template variables in working directory
            match crate::validation::resolve_template_variables(working_dir) {
                Ok(resolved_dir) => {
                    info!("Setting working directory for server {}: {}", server_id, resolved_dir);
                    command.current_dir(resolved_dir);
                },
                Err(e) => {
                    error!("Failed to resolve working directory template for server {}: {}", server_id, e);
                    return Err(format!("Failed to resolve working directory template: {}", e));
                }
            }
        }

        let client_info = ClientInfo {
            protocol_version: Default::default(),
            capabilities: ClientCapabilities::default(),
            client_info: Implementation {
                name: "dockmaster-mcp-client".into(),
                version: env!("CARGO_PKG_VERSION").into(),
            },
        };

        let tokio_child_process = TokioChildProcessCustom::new(command)
            .map_err(|e| {
                error!("Failed to create tokio child process for server {}: {}", server_id, e);
                // Enhanced error message for custom servers
                if matches!(server_data.server_type, crate::models::types::ServerType::Local | crate::models::types::ServerType::Custom) {
                    format!("Failed to start custom server '{}': {}. Please check the executable path and arguments.", server_id, e)
                } else {
                    format!("Failed to create tokio child process for server '{}': {}", server_id, e)
                }
            })?;
        let service = client_info
            .serve(tokio_child_process)
            .await
            .map_err(|e| {
                error!("Failed to serve tokio child process for server {}: {}", server_id, e);
                // Enhanced error message for custom servers
                if matches!(server_data.server_type, crate::models::types::ServerType::Local | crate::models::types::ServerType::Custom) {
                    format!("Failed to initialize custom server '{}': {}. The server may have crashed or failed to start properly.", server_id, e)
                } else {
                    format!("Failed to serve tokio child process for server '{}': {}", server_id, e)
                }
            })?;

        self.mcp_clients.write().await.insert(
            server_id.to_string(),
            MCPClient {
                client: Arc::new(service),
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
                error!("Failed to discover tools for server: {e}");
            }
        }

        info!("Successfully initialized client for server: {server_id}");

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

        // Update the tools service cache state
        // if let Some(tools_service) = MCPToolsService::get_instance().await {
        //     if let Err(e) = tools_service.set_tools_hidden(hidden).await {
        //         error!("Failed to update tools service visibility state: {}", e);
        //     }
        // }

        // Persist the state to the database
        let registry = self.tool_registry.read().await;
        registry.save_setting("tools_hidden", if hidden { "true" } else { "false" })
    }

    pub async fn discover_server_tools(
        &self,
        server_id: &str,
    ) -> Result<Vec<ServerToolInfo>, String> {
        info!("[discover_tools] Starting discovery for server: {server_id}");

        let mcp_client = self.mcp_clients.read().await.get(server_id).cloned();
        if let Some(mcp_client) = mcp_client {
            info!("[discover tools] Successfully got client for server: {server_id}");

            match mcp_client.server_status {
                ServerStatus::Running => {
                    info!("Server status is Running, about to call list_tools");

                    let list_tools = match mcp_client.client.list_tools(None).await {
                        Ok(result) => {
                            info!("mcp_client: list_tools call succeeded");
                            result
                        }
                        Err(e) => {
                            error!("mcp_client: list_tools call failed: {e}");
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

                    let mut server_tool_infos = Vec::new();
                    for tool in &tools {
                        info!("Saving tool info to database: {tool:?}");
                        let server_tool_info =
                            ServerToolInfo::from_tool(tool.clone(), server_id.to_string())
                                .inspect_err(|e| {
                                    error!("failed to create server tool info from tool: {e}")
                                })?;
                        registry
                            .save_server_tool(&server_tool_info)
                            .inspect_err(|e| {
                                error!("failed to save server tool to database: {e}")
                            })?;
                        server_tool_infos.push(server_tool_info);
                    }

                    // Save the tools to the server_tools map
                    let mut server_tools = self.server_tools.write().await;
                    server_tools.insert(server_id.to_string(), server_tool_infos.clone());

                    Ok(server_tool_infos)
                }
                _ => Err(format!("Server {server_id} is not running")),
            }
        } else {
            info!("No client found for server: {server_id}");
            Err(format!("No client found for server: {server_id}"))
        }
    }
}
