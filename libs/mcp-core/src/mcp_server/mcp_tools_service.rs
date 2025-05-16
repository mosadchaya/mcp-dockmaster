use crate::core::mcp_core_proxy_ext::McpCoreProxyExt;
use crate::types::{
    Distribution, RegistryToolsResponse, ServerConfiguration, ServerRegistrationRequest,
    ServerRegistrationResponse, ToolUninstallRequest,
};
use crate::{core::mcp_core::MCPCore, types::ToolExecutionRequest};
use log::{error, info};
use rmcp::model::Tool;
use rmcp::{Error as McpError, ServiceError};
use serde::Deserialize;
use serde_json::{json, Value};
use tokio::sync::{Mutex, RwLock};
use tokio::time::Instant;

use super::{
    tools::{
        get_configure_server_tool, get_list_installed_servers_tool, get_register_server_tool,
        get_search_server_tool, get_uninstall_server_tool, TOOL_LIST_INSTALLED_SERVERS,
        TOOL_UNINSTALL_SERVER,
    },
    TOOL_CONFIGURE_SERVER, TOOL_REGISTER_SERVER, TOOL_SEARCH_SERVER,
};

use lazy_static::lazy_static;
use std::sync::Arc;
use std::time::Duration;

#[derive(Deserialize, Debug)]

struct ToolRegistrationRequestByName {
    id: String,
    name: String,
    description: String,
    r#type: String,
    configuration: Option<ServerConfiguration>,
    distribution: Option<Distribution>,
}
#[derive(Deserialize, Debug)]

struct ToolRegistrationRequestById {
    tool_id: String,
}

#[derive(Deserialize, Debug)]
#[allow(clippy::large_enum_variant)]
#[serde(untagged)]
enum ToolRegistrationRequest {
    ByName(ToolRegistrationRequestByName),
    ById(ToolRegistrationRequestById),
}

// Cache structure to store registry data and timestamp
struct RegistryCache {
    data: Option<Value>,
    timestamp: Option<Instant>,
}

lazy_static! {
    static ref REGISTRY_CACHE: Mutex<RegistryCache> = Mutex::new(RegistryCache {
        data: None,
        timestamp: None,
    });
}

lazy_static! {
    static ref INSTANCE: RwLock<Option<Arc<MCPToolsService>>> = RwLock::new(None);
}

// Cache duration constant (1 minutes)
const CACHE_DURATION: Duration = Duration::from_secs(60);

pub struct MCPToolsService {
    mcp_core: MCPCore,
    tools_cache: Arc<RwLock<Vec<Tool>>>,
    are_tools_hidden: Arc<RwLock<bool>>,
}

impl MCPToolsService {
    pub fn new(mcp_core: MCPCore) -> Self {
        Self {
            mcp_core,
            tools_cache: Arc::new(RwLock::new(Vec::new())),
            are_tools_hidden: Arc::new(RwLock::new(false)),
        }
    }

    pub async fn initialize(mcp_core: MCPCore) -> Arc<Self> {
        let mut instance = INSTANCE.write().await;
        if instance.is_none() {
            *instance = Some(Arc::new(Self::new(mcp_core)));
        }
        instance.as_ref().unwrap().clone()
    }

    pub async fn get_instance() -> Option<Arc<Self>> {
        let instance = INSTANCE.read().await;
        instance.clone()
    }

    /// Set the tools visibility state
    pub async fn set_tools_hidden(&self, hidden: bool) -> Result<(), String> {
        let mut are_tools_hidden = self.are_tools_hidden.write().await;
        *are_tools_hidden = hidden;
        info!(
            "Tools visibility set to: {}",
            if hidden { "hidden" } else { "visible" }
        );
        Ok(())
    }

    /// Get the list of tools synchronously from cache
    pub fn list_tools(&self) -> Vec<Tool> {
        // Check if tools should be hidden using the cached state
        let are_tools_hidden = if let Ok(hidden) = self.are_tools_hidden.try_read() {
            *hidden
        } else {
            // If we can't read the state, assume tools are visible
            false
        };

        // If tools are hidden, return empty list
        if are_tools_hidden {
            info!("Tools are hidden, returning empty list");
            return Vec::new();
        }

        // Start with an empty list of tools
        let mut tools = Vec::new();

        // Get cached tools if available
        let cache_handle = self.tools_cache.clone();

        // Try to read from the existing cache (non-blocking)
        let cache_found = if let Ok(cache) = cache_handle.try_read() {
            if !cache.is_empty() {
                info!("Found {} tools in cache", cache.len());
                // Add all cached tools
                tools = cache.clone();
                true
            } else {
                false
            }
        } else {
            false
        };

        // If we didn't find any cached tools, add the built-in tools
        if !cache_found {
            info!("No tools in cache, adding built-in tools");
            tools.push(get_register_server_tool());
            tools.push(get_search_server_tool());
            tools.push(get_configure_server_tool());
            tools.push(get_uninstall_server_tool());
            tools.push(get_list_installed_servers_tool());
        }

        // Log what we're returning
        info!("Returning {} tools from list_tools", tools.len());

        // Trigger an async task to update the cache for future calls
        let mcp_core = self.mcp_core.clone();
        let cache_clone = self.tools_cache.clone();

        // Spawn a task to update the cache for future requests
        tokio::spawn(async move {
            if let Err(e) = update_cache_internal(mcp_core, cache_clone).await {
                error!("Failed to update tools cache: {}", e);
            }
        });

        tools
    }

    /// Update the tools cache with the latest tools from all servers
    pub async fn update_cache(&self) -> Result<(), String> {
        // Update the visibility state from MCPCore
        let mcp_state = self.mcp_core.mcp_state.read().await;
        let are_tools_hidden = mcp_state.are_tools_hidden.read().await;
        let mut cached_hidden = self.are_tools_hidden.write().await;
        *cached_hidden = *are_tools_hidden;

        // Only update the tools cache if tools are visible
        if !*are_tools_hidden {
            update_cache_internal(self.mcp_core.clone(), self.tools_cache.clone()).await
        } else {
            // If tools are hidden, clear the cache
            let mut cache = self.tools_cache.write().await;
            cache.clear();
            Ok(())
        }
    }

    /// Execute a tool by finding the appropriate server and forwarding the call
    pub async fn execute_tool(
        &self,
        tool_name: &str,
        args: Value,
    ) -> Result<Value, crate::MCPError> {
        match tool_name {
            TOOL_REGISTER_SERVER => handle_register_server(args).await,
            TOOL_SEARCH_SERVER => self.handle_search_server(args).await,
            TOOL_CONFIGURE_SERVER => self.handle_configure_server(args).await,
            TOOL_UNINSTALL_SERVER => self.handle_uninstall_server(args).await,
            TOOL_LIST_INSTALLED_SERVERS => self.handle_list_installed_servers(args).await,
            _ => {
                // For non-built-in tools, find the appropriate server that has this tool
                let mcp_state = self.mcp_core.mcp_state.read().await;
                let server_tools = mcp_state.server_tools.read().await;

                // Find which server has the requested tool
                let mut server_id = None;

                for (sid, tools) in &*server_tools {
                    for tool in tools {
                        if tool.id == tool_name {
                            server_id = Some(sid.clone());
                            break;
                        }

                        // Also check by name if id doesn't match
                        if tool.name == tool_name {
                            server_id = Some(sid.clone());
                            break;
                        }
                    }

                    if server_id.is_some() {
                        break;
                    }
                }

                // Drop the locks before proceeding
                drop(server_tools);
                drop(mcp_state);

                match server_id {
                    Some(server_id) => {
                        let request = ToolExecutionRequest {
                            tool_id: format!("{}:{}", server_id, tool_name),
                            parameters: args,
                        };

                        match self.mcp_core.execute_proxy_tool(request).await {
                            Ok(response) => {
                                if response.success {
                                    Ok(response.result.unwrap_or(json!(null)))
                                } else {
                                    Err(crate::MCPError::ExecutionError(
                                        response
                                            .error
                                            .unwrap_or_else(|| "Unknown error".to_string()),
                                    ))
                                }
                            }
                            Err(e) => Err(crate::MCPError::ExecutionError(format!(
                                "Failed to execute tool: {}",
                                e
                            ))),
                        }
                    }
                    None => Err(crate::MCPError::NotFound(format!(
                        "Tool '{}' not found",
                        tool_name
                    ))),
                }
            }
        }
    }
}
