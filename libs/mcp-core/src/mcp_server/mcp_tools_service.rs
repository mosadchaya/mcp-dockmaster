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


pub struct MCPToolsService {
    mcp_core: MCPCore,
    tools_cache: Arc<RwLock<Vec<Tool>>>,
    are_tools_hidden: Arc<RwLock<bool>>,
}

impl MCPToolsService for McpServer {
    /// Get the list of tools synchronously from cache
    pub fn list_tools(&self) -> Vec<Tool> {
        let tools = self.mcp_core.list_tools().await;
        tools
    }

    async fn handle_list_installed_servers(&self, _args: Value) -> Result<Value, crate::MCPError> {
        // Get the installed servers from MCPCore
        let result = self.mcp_core.list_servers().await;

        // Return the installed servers as JSON
        Ok(json!({
            "servers": result
        }))
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

    async fn handle_configure_server(&self, args: Value) -> Result<Value, crate::MCPError> {
        // Convert the args into the format expected by the HTTP handler
        let configure_request =
            if let Some(server_id) = args.get("server_id").and_then(|v| v.as_str()) {
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
            configure_request,
        )
        .await
        {
            Ok(response) => {
                // Update the tools cache after successful configuration
                if let Err(e) = self.update_tools_cache("configuration").await {
                    error!("Failed to update tools cache after configuration: {}", e);
                }
                Ok(response)
            }
            Err(error) => Err(ToolError::ExecutionError(
                error
                    .get("message")
                    .and_then(|m| m.as_str())
                    .unwrap_or("Unknown error")
                    .to_string(),
            )),
        }
    }

    /// Handle search_server tool
    async fn handle_search_server(&self, args: Value) -> Result<Value, crate::MCPError> {
        // Extract the query parameter from the args
        let query = match args.get("query").and_then(|q| q.as_str()) {
            Some(q) => q,
            None => {
                return Err(ToolError::ExecutionError(
                    "Missing or invalid 'query' parameter".to_string(),
                ))
            }
        };

        // Create a new RegistrySearch instance
        let mut registry_search = match RegistrySearch::new().await {
            Ok(search) => search,
            Err(e) => match e {
                SearchError::CacheError(msg) => {
                    error!("Cache error during registry search: {}", msg);
                    return Err(crate::MCPError::ExecutionError(format!(
                        "Registry cache error: {}",
                        msg
                    )));
                }
                SearchError::IndexError(msg) => {
                    error!("Index error during registry search: {}", msg);
                    return Err(crate::MCPError::ExecutionError(format!(
                        "Registry index error: {}",
                        msg
                    )));
                }
                SearchError::QueryError(msg) => {
                    error!("Query error during registry search: {}", msg);
                    return Err(crate::MCPError::ExecutionError(format!(
                        "Query error: {}",
                        msg
                    )));
                }
            },
        };

        // Execute the search
        let search_results = match registry_search.search(query) {
            Ok(results) => results,
            Err(e) => match e {
                SearchError::QueryError(msg) => {
                    return Err(crate::MCPError::ExecutionError(format!(
                        "Invalid query: {}",
                        msg
                    )));
                }
                _ => {
                    return Err(crate::MCPError::ExecutionError(format!(
                        "Search execution error: {:?}",
                        e
                    )));
                }
            },
        };

        // Limit results to top 10 for better UI display
        let top_results = search_results.into_iter().take(10).collect::<Vec<_>>();

        // Transform results into a format suitable for JSON response
        let formatted_results = top_results
            .into_iter()
            .map(|(tool, score)| {
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
            })
            .collect::<Vec<_>>();

        // Return the results as JSON
        Ok(json!({
            "results": formatted_results,
            "count": formatted_results.len(),
            "query": query
        }))
    }
}
