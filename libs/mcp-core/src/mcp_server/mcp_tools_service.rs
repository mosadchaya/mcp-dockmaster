use crate::core::mcp_core::MCPCore;
use crate::core::mcp_core_proxy_ext::McpCoreProxyExt;
use mcp_sdk_core::Tool;
use serde_json::json;
use tokio::sync::RwLock;
use log::{info, error};

use super::tools::{
    get_register_server_tool,
    get_search_server_tool,
    get_configure_server_tool,
    get_uninstall_server_tool,
    get_list_installed_servers_tool,
};

use std::sync::Arc;
use lazy_static::lazy_static;

lazy_static! {
    static ref INSTANCE: RwLock<Option<Arc<MCPToolsService>>> = RwLock::new(None);
}

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
        info!("Tools visibility set to: {}", if hidden { "hidden" } else { "visible" });
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
}

/// Internal function to update the cache
async fn update_cache_internal(mcp_core: MCPCore, cache: Arc<RwLock<Vec<Tool>>>) -> Result<(), String> {
    // Get user-installed tools from MCPCore
    match mcp_core.list_all_server_tools().await {
        Ok(server_tools) => {
            // Add built-in tools
            let mut tools_vec = vec![
                get_register_server_tool(),
                get_search_server_tool(),
                get_configure_server_tool(),
                get_uninstall_server_tool(),
                get_list_installed_servers_tool(),
            ];

            // Add user-installed tools
            for tool_info in server_tools {
                // Convert ServerToolInfo to Tool
                if let Some(input_schema) = tool_info.input_schema {
                    // Convert InputSchema to serde_json::Value
                    let schema_value = json!({
                        "type": input_schema.r#type,
                        "properties": input_schema.properties,
                        "required": input_schema.required,
                    });
                    
                    let tool = Tool {
                        name: tool_info.name,
                        description: tool_info.description,
                        input_schema: schema_value,
                    };
                    
                    tools_vec.push(tool);
                } else {
                    // Create a tool with an empty schema
                    let tool = Tool {
                        name: tool_info.name,
                        description: tool_info.description,
                        input_schema: json!({
                            "type": "object",
                            "properties": {},
                            "required": []
                        }),
                    };
                    
                    tools_vec.push(tool);
                }
            }
            
            // Update the cache
            let mut cache = cache.write().await;
            *cache = tools_vec;
            info!("Tools cache updated with {} tools", cache.len());
            Ok(())
        },
        Err(e) => {
            error!("Failed to update tools cache: {}", e);
            // Clear the cache to force refresh on next request
            let mut cache = cache.write().await;
            cache.clear();
            Err(e)
        }
    }
}
