// use std::sync::Arc;

// use log::error;
// use serde_json::{json, Value};

// use crate::{
//     core::mcp_core::MCPCore,
//     core::mcp_core_proxy_ext::McpCoreProxyExt,
//     mcp_server::mcp_tools_service::MCPToolsService,
//     models::types::ToolUninstallRequest,
//     registry::registry_search::{RegistrySearch, SearchError},
// };

// use super::notifications::broadcast_tools_list_changed;

// #[derive(Clone)]
// pub struct MCPDockmasterRouter {
//     mcp_core: MCPCore,
//     server_name: String,
//     tools_service: Arc<MCPToolsService>,
// }

// impl MCPDockmasterRouter {
//     /// Create a new MCP router for the Dockmaster server
//     pub async fn new(mcp_core: MCPCore) -> Self {
//         let tools_service = MCPToolsService::initialize(mcp_core.clone()).await;
//         Self {
//             mcp_core,
//             server_name: "mcp-dockmaster-server".to_string(),
//             tools_service,
//         }
//     }

//     /// Update the tools cache and broadcast a notification
//     pub async fn update_tools_cache(&self, operation: &str) -> Result<(), String> {
//         // Update the tools cache
//         if let Err(e) = self.tools_service.update_cache().await {
//             error!("Failed to update tools cache after {}: {}", operation, e);
//             return Err(e);
//         }

//         // Spawn the broadcast notification as a separate task
//         tokio::spawn(async {
//             tokio::time::sleep(std::time::Duration::from_secs(5)).await;
//             broadcast_tools_list_changed().await;
//         });

//         Ok(())
//     }

//     /// Handle register_server tool
//     async fn handle_register_server(&self, args: Value) -> Result<Value, crate::MCPError> {
//         // Convert the args into the format expected by the HTTP handler
//         let registration_request =
//             if let Some(tool_id) = args.get("tool_id").and_then(|v| v.as_str()) {
//                 // If we receive a tool_id, treat it as a registry-based installation
//                 serde_json::json!({
//                     "tool_id": tool_id
//                 })
//             } else {
//                 // Otherwise, expect direct registration parameters
//                 args
//             };

//         // Use the HTTP handler's logic through MCPCore
//         match crate::http_server::handlers::handle_register_tool(
//             self.mcp_core.clone(),
//             registration_request,
//         )
//         .await
//         {
//             Ok(response) => {
//                 // Update the tools cache after successful registration
//                 if let Err(e) = self.update_tools_cache("registration").await {
//                     error!("Failed to update tools cache after registration: {}", e);
//                 }

//                 Ok(json!({
//                     "success": true,
//                     "message": response.message,
//                     "tool_id": response.tool_id
//                 }))
//             }
//             Err(error) => Err(crate::MCPError::ExecutionError(error.message)),
//         }
//     }

//     /// Handle uninstall_server tool
//     async fn handle_uninstall_server(&self, args: Value) -> Result<Value, crate::MCPError> {
//         // Convert the args into the format expected by the HTTP handler
//         let uninstall_request: ToolUninstallRequest =
//             if let Some(server_id) = args.get("server_id").and_then(|v| v.as_str()) {
//                 ToolUninstallRequest {
//                     server_id: server_id.to_string(),
//                 }
//             } else {
//                 // Otherwise, return error of missing server_id
//                 return Err(crate::MCPError::ExecutionError(
//                     "Missing server_id parameter".to_string(),
//                 ));
//             };
//         match self.mcp_core.uninstall_server(uninstall_request).await {
//             Ok(response) => {
//                 // Update the tools cache after successful uninstallation
//                 if let Err(e) = self.update_tools_cache("uninstallation").await {
//                     error!("Failed to update tools cache after uninstallation: {}", e);
//                 }
//                 Ok(json!({
//                     "success": true,
//                     "message": response.message
//                 }))
//             }
//             Err(error) => Err(crate::MCPError::ExecutionError(error)),
//         }
//     }

//     /// Handle configure_server tool
//     async fn handle_configure_server(&self, args: Value) -> Result<Value, crate::MCPError> {
//         // Convert the args into the format expected by the HTTP handler
//         let configure_request =
//             if let Some(server_id) = args.get("server_id").and_then(|v| v.as_str()) {
//                 serde_json::json!({
//                     "tool_id": server_id,
//                     "config": args.get("config").unwrap_or(&Value::Null)
//                 })
//             } else {
//                 // Otherwise, expect direct registration parameters
//                 args
//             };

//         // Use the HTTP handler's logic through MCPCore
//         match crate::http_server::handlers::handle_get_server_config(
//             self.mcp_core.clone(),
//             configure_request,
//         )
//         .await
//         {
//             Ok(response) => {
//                 // Update the tools cache after successful configuration
//                 if let Err(e) = self.update_tools_cache("configuration").await {
//                     error!("Failed to update tools cache after configuration: {}", e);
//                 }
//                 Ok(response)
//             }
//             Err(error) => Err(ToolError::ExecutionError(
//                 error
//                     .get("message")
//                     .and_then(|m| m.as_str())
//                     .unwrap_or("Unknown error")
//                     .to_string(),
//             )),
//         }
//     }

//     /// Handle search_server tool
//     async fn handle_search_server(&self, args: Value) -> Result<Value, crate::MCPError> {
//         // Extract the query parameter from the args
//         let query = match args.get("query").and_then(|q| q.as_str()) {
//             Some(q) => q,
//             None => {
//                 return Err(ToolError::ExecutionError(
//                     "Missing or invalid 'query' parameter".to_string(),
//                 ))
//             }
//         };

//         // Create a new RegistrySearch instance
//         let mut registry_search = match RegistrySearch::new().await {
//             Ok(search) => search,
//             Err(e) => match e {
//                 SearchError::CacheError(msg) => {
//                     error!("Cache error during registry search: {}", msg);
//                     return Err(crate::MCPError::ExecutionError(format!(
//                         "Registry cache error: {}",
//                         msg
//                     )));
//                 }
//                 SearchError::IndexError(msg) => {
//                     error!("Index error during registry search: {}", msg);
//                     return Err(crate::MCPError::ExecutionError(format!(
//                         "Registry index error: {}",
//                         msg
//                     )));
//                 }
//                 SearchError::QueryError(msg) => {
//                     error!("Query error during registry search: {}", msg);
//                     return Err(crate::MCPError::ExecutionError(format!(
//                         "Query error: {}",
//                         msg
//                     )));
//                 }
//             },
//         };

//         // Execute the search
//         let search_results = match registry_search.search(query) {
//             Ok(results) => results,
//             Err(e) => match e {
//                 SearchError::QueryError(msg) => {
//                     return Err(crate::MCPError::ExecutionError(format!(
//                         "Invalid query: {}",
//                         msg
//                     )));
//                 }
//                 _ => {
//                     return Err(crate::MCPError::ExecutionError(format!(
//                         "Search execution error: {:?}",
//                         e
//                     )));
//                 }
//             },
//         };

//         // Limit results to top 10 for better UI display
//         let top_results = search_results.into_iter().take(10).collect::<Vec<_>>();

//         // Transform results into a format suitable for JSON response
//         let formatted_results = top_results
//             .into_iter()
//             .map(|(tool, score)| {
//                 json!({
//                     "id": tool.id,
//                     "name": tool.name,
//                     "description": tool.description,
//                     "short_description": tool.short_description,
//                     "publisher": tool.publisher,
//                     "is_official": tool.is_official,
//                     "source_url": tool.source_url,
//                     "distribution": tool.distribution,
//                     "license": tool.license,
//                     "runtime": tool.runtime,
//                     "categories": tool.categories,
//                     "tags": tool.tags,
//                     "score": score
//                 })
//             })
//             .collect::<Vec<_>>();

//         // Return the results as JSON
//         Ok(json!({
//             "results": formatted_results,
//             "count": formatted_results.len(),
//             "query": query
//         }))
//     }

//     /// Handle list_installed_servers tool
//     async fn handle_list_installed_servers(&self, _args: Value) -> Result<Value, crate::MCPError> {
//         // Get the installed servers from MCPCore
//         let result = self.mcp_core.list_servers().await;

//         // Return the installed servers as JSON
//         Ok(json!({
//             "servers": result
//         }))
//     }
// }
