use std::{future::Future, pin::Pin, sync::Arc};

use mcp_sdk_core::{
    handler::{PromptError, ResourceError},
    prompt::Prompt,
    protocol::ServerCapabilities,
    Content, Resource, Tool, ToolError,
};
use mcp_sdk_server::router::CapabilitiesBuilder;
use serde_json::{json, Value};
use log::{info, error};

use crate::{
    core::mcp_core::MCPCore,
    core::mcp_core_proxy_ext::McpCoreProxyExt,
    models::types::{ToolExecutionRequest, ToolUninstallRequest},
    registry::registry_search::{RegistrySearch, SearchError},
    mcp_server::mcp_tools_service::MCPToolsService,
};

use super::tools::{
    TOOL_REGISTER_SERVER,
    TOOL_SEARCH_SERVER,
    TOOL_CONFIGURE_SERVER,
    TOOL_UNINSTALL_SERVER,
    TOOL_LIST_INSTALLED_SERVERS,
};

use super::notifications::broadcast_tools_list_changed;

/// MCP Router implementation for the Dockmaster server
/// This router handles all MCP protocol methods and integrates with the MCPCore
#[derive(Clone)]
pub struct MCPDockmasterRouter {
    mcp_core: MCPCore,
    server_name: String,
    tools_service: Arc<MCPToolsService>,
}

impl MCPDockmasterRouter {
    /// Create a new MCP router for the Dockmaster server
    pub async fn new(mcp_core: MCPCore) -> Self {
        let tools_service = MCPToolsService::initialize(mcp_core.clone()).await;
        Self {
            mcp_core,
            server_name: "mcp-dockmaster-server".to_string(),
            tools_service,
        }
    }

    /// Update the tools cache and broadcast a notification
    pub async fn update_tools_cache(&self, operation: &str) -> Result<(), String> {
        // Update the tools cache
        if let Err(e) = self.tools_service.update_cache().await {
            error!("Failed to update tools cache after {}: {}", operation, e);
            return Err(e);
        }

        // Spawn the broadcast notification as a separate task
        tokio::spawn(async {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            broadcast_tools_list_changed().await;
        });

        Ok(())
    }

    /// Handle register_server tool
    async fn handle_register_server(&self, args: Value) -> Result<Value, ToolError> {
        // Convert the args into the format expected by the HTTP handler
        let registration_request = if let Some(tool_id) = args.get("tool_id").and_then(|v| v.as_str()) {
            // If we receive a tool_id, treat it as a registry-based installation
            serde_json::json!({
                "tool_id": tool_id
            })
        } else {
            // Otherwise, expect direct registration parameters
            args
        };

        // Use the HTTP handler's logic through MCPCore
        match crate::http_server::handlers::handle_register_tool(
            self.mcp_core.clone(),
            registration_request
        ).await {
            Ok(response) => {
                // Update the tools cache after successful registration
                if let Err(e) = self.update_tools_cache("registration").await {
                    error!("Failed to update tools cache after registration: {}", e);
                }
                
                Ok(json!({
                    "success": true,
                    "message": response.message,
                    "tool_id": response.tool_id
                }))
            },
            Err(error) => Err(ToolError::ExecutionError(error.message)),
        }
    }

    /// Handle uninstall_server tool
    async fn handle_uninstall_server(&self, args: Value) -> Result<Value, ToolError> {
        // Convert the args into the format expected by the HTTP handler
        let uninstall_request: ToolUninstallRequest = if let Some(server_id) = args.get("server_id").and_then(|v| v.as_str()) {
            ToolUninstallRequest {
                server_id: server_id.to_string()
            }
        } else {
            // Otherwise, return error of missing server_id
            return Err(ToolError::ExecutionError("Missing server_id parameter".to_string()));
        };
        match self.mcp_core.uninstall_server(uninstall_request).await {
            Ok(response) => {
                // Update the tools cache after successful uninstallation
                if let Err(e) = self.update_tools_cache("uninstallation").await {
                    error!("Failed to update tools cache after uninstallation: {}", e);
                }
                Ok(json!({
                    "success": true,
                    "message": response.message
                }))
            },
            Err(error) => Err(ToolError::ExecutionError(error)),
        }
    }

    /// Handle configure_server tool
    async fn handle_configure_server(&self, args: Value) -> Result<Value, ToolError> {
        // Convert the args into the format expected by the HTTP handler
        let configure_request = if let Some(server_id) = args.get("server_id").and_then(|v| v.as_str()) {
            serde_json::json!({
                "tool_id": server_id,
                "config": args.get("config").unwrap_or(&Value::Null)
            })
        } else {
            // Otherwise, expect direct registration parameters
            args
        };

        // Use the HTTP handler's logic through MCPCore
        match crate::http_server::handlers::handle_get_server_config(
            self.mcp_core.clone(),
            configure_request
        ).await {
            Ok(response) => {
                // Update the tools cache after successful configuration
                if let Err(e) = self.update_tools_cache("configuration").await {
                    error!("Failed to update tools cache after configuration: {}", e);
                }
                Ok(response)
            }
            Err(error) => Err(ToolError::ExecutionError(error.get("message").and_then(|m| m.as_str()).unwrap_or("Unknown error").to_string())),
        }
    }

    /// Handle search_server tool
    async fn handle_search_server(&self, args: Value) -> Result<Value, ToolError> {
        // Extract the query parameter from the args
        let query = match args.get("query").and_then(|q| q.as_str()) {
            Some(q) => q,
            None => return Err(ToolError::ExecutionError("Missing or invalid 'query' parameter".to_string())),
        };

        // Create a new RegistrySearch instance
        let mut registry_search = match RegistrySearch::new().await {
            Ok(search) => search,
            Err(e) => match e {
                SearchError::CacheError(msg) => {
                    error!("Cache error during registry search: {}", msg);
                    return Err(ToolError::ExecutionError(format!("Registry cache error: {}", msg)));
                },
                SearchError::IndexError(msg) => {
                    error!("Index error during registry search: {}", msg);
                    return Err(ToolError::ExecutionError(format!("Registry index error: {}", msg)));
                },
                SearchError::QueryError(msg) => {
                    error!("Query error during registry search: {}", msg);
                    return Err(ToolError::ExecutionError(format!("Query error: {}", msg)));
                },
            },
        };

        // Execute the search
        let search_results = match registry_search.search(query) {
            Ok(results) => results,
            Err(e) => match e {
                SearchError::QueryError(msg) => {
                    return Err(ToolError::ExecutionError(format!("Invalid query: {}", msg)));
                },
                _ => {
                    return Err(ToolError::ExecutionError(format!("Search execution error: {:?}", e)));
                },
            },
        };

        // Limit results to top 10 for better UI display
        let top_results = search_results.into_iter().take(10).collect::<Vec<_>>();
        
        // Transform results into a format suitable for JSON response
        let formatted_results = top_results.into_iter().map(|(tool, score)| {
            json!({
                "id": tool.id,
                "name": tool.name,
                "description": tool.description,
                "short_description": tool.short_description,
                "publisher": tool.publisher,
                "is_official": tool.is_official,
                "source_url": tool.source_url,
                "distribution": tool.distribution,
                "license": tool.license,
                "runtime": tool.runtime,
                "categories": tool.categories,
                "tags": tool.tags,
                "score": score
            })
        }).collect::<Vec<_>>();

        // Return the results as JSON
        Ok(json!({
            "results": formatted_results,
            "count": formatted_results.len(),
            "query": query
        }))
    }

    /// Handle list_installed_servers tool
    async fn handle_list_installed_servers(&self, _args: Value) -> Result<Value, ToolError> {
        // Get the installed servers from MCPCore
        let result = self.mcp_core.list_servers().await;
        
        // Return the installed servers as JSON
        Ok(json!({
            "servers": result
        }))
    }

    /// Execute a tool by finding the appropriate server and forwarding the call
    async fn execute_tool(&self, tool_name: &str, args: Value) -> Result<Value, ToolError> {
        match tool_name {
            TOOL_REGISTER_SERVER => self.handle_register_server(args).await,
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
                                    Err(ToolError::ExecutionError(response.error.unwrap_or_else(|| "Unknown error".to_string())))
                                }
                            },
                            Err(e) => Err(ToolError::ExecutionError(format!("Failed to execute tool: {}", e))),
                        }
                    },
                    None => Err(ToolError::NotFound(format!("Tool '{}' not found", tool_name))),
                }
            }
        }
    }
}

impl mcp_sdk_server::Router for MCPDockmasterRouter {
    fn name(&self) -> String {
        self.server_name.clone()
    }

    fn instructions(&self) -> String {
        "This server provides tools for managing Docker containers, images, and networks. You can use it to manage containers, build images, and interact with Docker registries. It also allows you to register new MCP servers.".to_string()
    }

    fn capabilities(&self) -> ServerCapabilities {
        // Build capabilities with tools support
        CapabilitiesBuilder::new()
            .with_tools(false)
            .with_resources(false, false)
            .with_prompts(false)
            .build()
    }

    fn list_tools(&self) -> Vec<Tool> {
        self.tools_service.list_tools()
    }

    fn call_tool(
        &self,
        tool_name: &str,
        arguments: Value,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Content>, ToolError>> + Send + 'static>> {
        let this = self.clone();
        let tool_name = tool_name.to_string();
        info!("Calling tool: {}", tool_name);
        Box::pin(async move {
            match this.execute_tool(&tool_name, arguments).await {
                Ok(result) => {
                    let result_str = serde_json::to_string_pretty(&result).unwrap_or_default();
                    Ok(vec![Content::text(result_str)])
                },
                Err(e) => Err(e),
            }
        })
    }

    fn list_resources(&self) -> Vec<Resource> {
        // No resources for now
        vec![]
    }

    fn read_resource(
        &self,
        uri: &str,
    ) -> Pin<Box<dyn Future<Output = Result<String, ResourceError>> + Send + 'static>> {
        let uri = uri.to_string();
        Box::pin(async move {
            Err(ResourceError::NotFound(format!("Resource not found: {}", uri)))
        })
    }

    fn list_prompts(&self) -> Vec<Prompt> {
        // No prompts for now
        vec![]
    }

    fn get_prompt(
        &self,
        prompt_name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<String, PromptError>> + Send + 'static>> {
        let prompt_name = prompt_name.to_string();
        Box::pin(async move {
            Err(PromptError::NotFound(format!("Prompt not found: {}", prompt_name)))
        })
    }
}
