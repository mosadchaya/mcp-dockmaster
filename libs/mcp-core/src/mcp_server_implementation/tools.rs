use std::{borrow::Cow, sync::Arc};

use log::{error, info};
use rmcp::{
    model::{CallToolResult, Content, Tool},
    Error as McpError,
};
use serde::Deserialize;
use serde_json::{json, Map, Value};

use crate::{
    config::ToolConfig,
    core::{mcp_core::MCPCore, mcp_core_proxy_ext::McpCoreProxyExt},
    mcp_server_implementation::registry_cache::fetch_tool_from_registry,
    registry::registry_search::{RegistrySearch, SearchError},
    types::{
        Distribution, ServerConfigUpdateRequest, ServerConfiguration, ServerRegistrationRequest,
        ToolUninstallRequest,
    },
};

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

#[derive(Deserialize, Debug)]
struct ServerSearchRequest {
    query: String,
}

/// Base tool names (without namespace)
const BASE_TOOL_REGISTER_SERVER: &str = "register_server";
const BASE_TOOL_SEARCH_SERVER: &str = "search_server";
const BASE_TOOL_CONFIGURE_SERVER: &str = "configure_server";
const BASE_TOOL_UNINSTALL_SERVER: &str = "uninstall_server";
const BASE_TOOL_LIST_INSTALLED_SERVERS: &str = "list_installed_servers";

/// Get namespaced tool names using configuration
pub fn get_tool_names() -> (String, String, String, String, String) {
    let config = ToolConfig::from_env();
    (
        config.tool_name(BASE_TOOL_REGISTER_SERVER),
        config.tool_name(BASE_TOOL_SEARCH_SERVER),
        config.tool_name(BASE_TOOL_CONFIGURE_SERVER),
        config.tool_name(BASE_TOOL_UNINSTALL_SERVER),
        config.tool_name(BASE_TOOL_LIST_INSTALLED_SERVERS),
    )
}

/// Get the list installed servers tool definition
pub fn get_list_installed_servers_tool() -> Tool {
    let config = ToolConfig::from_env();
    let tool_name = config.tool_name(BASE_TOOL_LIST_INSTALLED_SERVERS);
    Tool {
        name: Cow::Owned(tool_name),
        description: Some(Cow::Owned("List all installed MCP Dockmaster servers".to_string())),
        input_schema: Arc::new(serde_json::Map::from_iter([
            ("type".to_string(), json!("object")),
            ("properties".to_string(), json!({})),
            ("required".to_string(), json!([])),
        ])),
        annotations: None,
    }
}

/// Get the register_server tool definition
pub fn get_register_server_tool() -> Tool {
    let config = ToolConfig::from_env();
    let tool_name = config.tool_name(BASE_TOOL_REGISTER_SERVER);
    Tool {
        name: Cow::Owned(tool_name),
        description: Some(Cow::Owned(
            "Register a new MCP server with Dockmaster using its registry tool ID".to_string(),
        )),
        input_schema: Arc::new(serde_json::Map::from_iter([
            ("type".to_string(), json!("object")),
            (
                "properties".to_string(),
                json!({
                    "tool_id": {
                        "type": "string",
                        "description": "ID of the tool in the registry to install"
                    }
                }),
            ),
            ("required".to_string(), json!(["tool_id"])),
        ])),
        annotations: None,
    }
}

pub fn get_search_server_tool() -> Tool {
    let config = ToolConfig::from_env();
    let tool_name = config.tool_name(BASE_TOOL_SEARCH_SERVER);
    Tool {
        name: Cow::Owned(tool_name),
        description: Some(Cow::Owned(
            "Search for MCP Servers in the Dockmaster registry".to_string(),
        )),
        input_schema: Arc::new(serde_json::Map::from_iter([
            ("type".to_string(), json!("object")),
            (
                "properties".to_string(),
                json!({
                    "query": {
                        "type": "string",
                        "description": "Search query"
                    }
                }),
            ),
            ("required".to_string(), json!(["query"])),
        ])),
        annotations: None,
    }
}

pub fn get_configure_server_tool() -> Tool {
    let config = ToolConfig::from_env();
    let tool_name = config.tool_name(BASE_TOOL_CONFIGURE_SERVER);
    Tool {
        name: Cow::Owned(tool_name),
        description: Some(Cow::Owned(
            "Configure a Dockmaster server and its environment variables".to_string(),
        )),
        input_schema: Arc::new(serde_json::Map::from_iter([
            ("type".to_string(), json!("object")),
            (
                "properties".to_string(),
                json!({
                    "server_id": {
                        "type": "string",
                    "description": "ID of the server to configure"
                },
                "config": {
                    "type": "object",
                    "description": "Configuration for the server, it's not neccesary to nest the environment variables inside an env object, just pass the key and value"
                }
                }),
            ),
            ("required".to_string(), json!(["server_id", "config"])),
        ])),
        annotations: None,
    }
}

/// Get the uninstall_server tool definition
pub fn get_uninstall_server_tool() -> Tool {
    let config = ToolConfig::from_env();
    let tool_name = config.tool_name(BASE_TOOL_UNINSTALL_SERVER);
    Tool {
        name: Cow::Owned(tool_name),
        description: Some(Cow::Owned(
            "Uninstall a server from Dockmaster using its registry tool ID".to_string(),
        )),
        input_schema: Arc::new(serde_json::Map::from_iter([
            ("type".to_string(), json!("object")),
            (
                "properties".to_string(),
                json!({
                "server_id": {
                    "type": "string",
                    "description": "ID of the server to uninstall"
                    }
                }),
            ),
            ("required".to_string(), json!(["server_id"])),
        ])),
        annotations: None,
    }
}

pub async fn handle_register_server(
    mcp_core: Arc<MCPCore>,
    params: Map<String, Value>,
) -> Result<CallToolResult, McpError> {
    let params: ToolRegistrationRequest =
        serde_json::from_value(serde_json::Value::Object(params)).unwrap();

    info!("[INSTALLATION] handle_register_tool: params {params:?}");
    match params {
        ToolRegistrationRequest::ByName(request) => {
            info!("[INSTALLATION] handle_register_tool: request (BY NAME) {request:?}");
            let tool_id = request.id;
            let tool_name = request.name;
            let description = request.description;
            let tools_type = request.r#type;
            let configuration = request.configuration;
            let distribution = request.distribution;

            let tool = ServerRegistrationRequest {
                server_id: tool_id.clone(),
                server_name: tool_name,
                description,
                tools_type,
                configuration,
                distribution,
                server_type: None, // Default for registry tools
                working_directory: None,
                executable_path: None,
            };

            info!("[POST] handle_register_tool: tool {tool:?}");
            let r = mcp_core.register_server(tool).await.map_err(|e| {
                McpError::internal_error(format!("Error registering server: {e}"), None)
            })?;
            info!("[INSTALLATION] handle_register_tool: r {r:?}");
            Ok(CallToolResult {
                content: vec![Content::text(r.message)],
                is_error: Some(false),
            })
        }
        ToolRegistrationRequest::ById(request) => {
            info!("[INSTALLATION] handle_register_tool: request (BY ID) {request:?}");
            let tool_id = request.tool_id;

            let registry = fetch_tool_from_registry().await?;

            let tool = registry
                .tools
                .iter()
                .find(|tool| tool.id.as_str() == tool_id);
            if tool.is_none() {
                return Err(McpError::invalid_params(
                    format!("Tool {tool_id} not found"),
                    Some(serde_json::Value::String(tool_id)),
                ));
            }
            let tool = tool.unwrap();
            info!("Building tool from registry: {tool:?}");
            let r = mcp_core
                .register_server(ServerRegistrationRequest {
                    server_id: tool_id.clone(),
                    server_name: tool.name.clone(),
                    description: tool.description.clone(),
                    tools_type: tool.runtime.clone(),
                    configuration: Some(tool.config.clone()),
                    distribution: Some(tool.distribution.clone()),
                    server_type: None, // Default for registry tools
                    working_directory: None,
                    executable_path: None,
                })
                .await
                .map_err(|e| {
                    McpError::internal_error(format!("Error registering server: {e}"), None)
                })?;
            info!("[INSTALLATION] handle_register_tool: r {r:?}");
            Ok(CallToolResult {
                content: vec![Content::text(r.message)],
                is_error: Some(false),
            })
        }
    }
}

/// Handle uninstall_server tool
pub async fn handle_uninstall_server(
    mcp_core: Arc<MCPCore>,
    args: Map<String, Value>,
) -> Result<CallToolResult, McpError> {
    // Convert the args into the format expected by the HTTP handler
    let uninstall_request: ToolUninstallRequest =
        if let Some(server_id) = args.get("server_id").and_then(|v| v.as_str()) {
            ToolUninstallRequest {
                server_id: server_id.to_string(),
            }
        } else {
            // Otherwise, return error of missing server_id
            return Err(McpError::invalid_params(
                "Missing server_id parameter".to_string(),
                Some(serde_json::Value::String("server_id".to_string())),
            ));
        };
    match mcp_core.uninstall_server(uninstall_request).await {
        Ok(response) => Ok(CallToolResult {
            content: vec![Content::text(response.message)],
            is_error: Some(false),
        }),
        Err(error) => Err(McpError::internal_error(
            format!("Error uninstalling server: {error}"),
            None,
        )),
    }
}

/// Handle configure_server tool
pub async fn handle_configure_server(
    mcp_core: Arc<MCPCore>,
    args: Map<String, Value>,
) -> Result<CallToolResult, McpError> {
    let configure_request: ServerConfigUpdateRequest =
        serde_json::from_value(serde_json::Value::Object(args)).unwrap();

    let r = mcp_core
        .update_server_config(configure_request)
        .await
        .map_err(|e| McpError::internal_error(format!("Error configuring server: {e}"), None))?;

    Ok(CallToolResult {
        content: vec![Content::text(r.message)],
        is_error: Some(false),
    })
}

/// Handle search_server tool
pub async fn handle_search_server(args: Map<String, Value>) -> Result<CallToolResult, McpError> {
    let params: ServerSearchRequest =
        serde_json::from_value(serde_json::Value::Object(args)).unwrap();

    // Create a new RegistrySearch instance
    let mut registry_search = match RegistrySearch::new().await {
        Ok(search) => search,
        Err(e) => match e {
            SearchError::CacheError(msg) => {
                error!("Cache error during registry search: {msg}");
                return Err(McpError::internal_error(
                    format!("Cache error during registry search: {msg}"),
                    None,
                ));
            }
            SearchError::IndexError(msg) => {
                error!("Index error during registry search: {msg}");
                return Err(McpError::internal_error(
                    format!("Registry index error: {msg}"),
                    None,
                ));
            }
            SearchError::QueryError(msg) => {
                error!("Query error during registry search: {msg}");
                return Err(McpError::internal_error(
                    format!("Query error: {msg}"),
                    None,
                ));
            }
        },
    };

    // Execute the search
    let search_results = match registry_search.search(params.query.as_str()) {
        Ok(results) => results,
        Err(e) => match e {
            SearchError::QueryError(msg) => {
                return Err(McpError::invalid_params(
                    format!("Invalid query: {msg}"),
                    None,
                ));
            }
            _ => {
                return Err(McpError::invalid_params(
                    format!("Search execution error: {e}"),
                    None,
                ));
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
    Ok(CallToolResult {
        content: vec![Content::text(
            serde_json::to_string(&formatted_results).unwrap(),
        )],
        is_error: Some(false),
    })
}

/// Handle list_installed_servers tool
pub async fn handle_list_installed_servers(
    mcp_core: Arc<MCPCore>,
) -> Result<CallToolResult, McpError> {
    // Get the installed servers from MCPCore
    let result = mcp_core.list_servers().await;

    // Return the installed servers as JSON
    Ok(CallToolResult {
        content: vec![Content::text(serde_json::to_string(&result).unwrap())],
        is_error: Some(false),
    })
}
