use log::{error, info};
use serde_json::{json, Value};
use crate::mcp_state::MCPState;
use crate::models::types::{
    DiscoverServerToolsRequest, DiscoverServerToolsResponse, ToolExecutionRequest,
    ToolExecutionResponse,
};
use crate::MCPError;
use super::rpc_io::rpc_call;

/// Discover tools available from an MCP server
pub async fn discover_server_tools(
    server_id: &str,
    stdin: &mut tokio::process::ChildStdin,
    stdout: &mut tokio::process::ChildStdout,
) -> Result<Vec<Value>, String> {
    info!("Discovering tools from server {}", server_id);

    // According to MCP specification, the correct method is "tools/list"
    let params = json!({});
    
    match rpc_call(server_id, "tools/list", params, stdin, stdout, 10).await {
        Ok(result) => {
            // Handle different response formats
            if let Some(tools_array) = result.as_array() {
                info!("Found {} tools in result array", tools_array.len());
                return Ok(tools_array.clone());
            }
            
            // Some implementations might nest it under a tools field
            if let Some(tools) = result.get("tools") {
                if let Some(tools_array) = tools.as_array() {
                    info!("Found {} tools in result.tools array", tools_array.len());
                    return Ok(tools_array.clone());
                }
            }
            
            // If there's a result but we couldn't find tools array, try to use the entire result
            info!("No tools array found, using entire result as fallback");
            return Ok(vec![result]);
        },
        Err(e) => {
            return Err(format!("Failed to discover tools: {}", e));
        }
    }
}

/// Execute a tool on an MCP server
pub async fn execute_server_tool(
    server_id: &str,
    tool_name: &str,
    parameters: Value,
    stdin: &mut tokio::process::ChildStdin,
    stdout: &mut tokio::process::ChildStdout,
) -> Result<Value, MCPError> {
    let params = json!({ 
        "name": tool_name, 
        "arguments": parameters 
    });
    
    rpc_call(server_id, "tools/call", params, stdin, stdout, 30).await
}

/// Discover tools from a specific MCP server
pub async fn discover_tools(
    mcp_state: &MCPState,
    request: DiscoverServerToolsRequest,
) -> Result<DiscoverServerToolsResponse, String> {
    // Check if the server exists and is running
    let server_running = {
        let process_manager = mcp_state.process_manager.read().await;
        process_manager
            .processes
            .get(&request.server_id)
            .is_some_and(|p| p.is_some())
    };

    if !server_running {
        return Ok(DiscoverServerToolsResponse {
            success: false,
            tools: None,
            error: Some(format!(
                "Server with ID '{}' is not running",
                request.server_id
            )),
        });
    }

    // Discover tools from the server
    let mut process_manager = mcp_state.process_manager.write().await;
    let result =
        if let Some((stdin, stdout)) = process_manager.process_ios.get_mut(&request.server_id) {
            discover_server_tools(&request.server_id, stdin, stdout).await
        } else {
            Err(format!(
                "Server {} not found or not running",
                request.server_id
            ))
        };

    // Release the process_manager lock before accessing server_tools
    drop(process_manager);

    // Get a write lock on server_tools to update
    let mut server_tools = mcp_state.server_tools.write().await;
    match result {
        Ok(tools) => {
            // Store the discovered tools
            server_tools.insert(request.server_id.clone(), tools.clone());

            Ok(DiscoverServerToolsResponse {
                success: true,
                tools: Some(tools),
                error: None,
            })
        }
        Err(e) => Ok(DiscoverServerToolsResponse {
            success: false,
            tools: None,
            error: Some(format!("Failed to discover tools: {}", e)),
        }),
    }
}

/// Execute a tool from an MCP server
pub async fn execute_proxy_tool(
    mcp_state: &MCPState,
    request: ToolExecutionRequest,
) -> Result<ToolExecutionResponse, String> {
    // Extract server_id and tool_id from the proxy_id
    let parts: Vec<&str> = request.tool_id.split(':').collect();
    println!("parts: {:?}", parts);
    if parts.len() != 2 {
        return Err("Invalid tool_id format. Expected 'server_id:tool_id'".to_string());
    }

    let server_id = parts[0];
    println!("server_id: {}", server_id);
    let tool_id = parts[1];
    println!("tool_id: {}", tool_id);

    // Execute the tool on the server
    let mut process_manager = mcp_state.process_manager.write().await;

    // Check if the server exists
    let result = if !process_manager.process_ios.contains_key(server_id) {
        Err(MCPError::ServerNotFound(server_id.to_string()))
    } else {
        // Get stdin/stdout for the server
        let (stdin, stdout) = process_manager.process_ios.get_mut(server_id).unwrap();

        // Execute the tool
        execute_server_tool(
            server_id,
            tool_id,
            request.parameters.clone(),
            stdin,
            stdout,
        )
        .await
    };

    // Release the lock
    drop(process_manager);

    match result {
        Ok(result) => Ok(ToolExecutionResponse {
            success: true,
            result: Some(result),
            error: None,
        }),
        Err(e) => Ok(ToolExecutionResponse {
            success: false,
            result: None,
            error: Some(e.to_string()),
        }),
    }
}

/// List all available tools from all running MCP servers
pub async fn list_all_server_tools(mcp_state: &MCPState) -> Result<Vec<Value>, String> {
    let server_tools = mcp_state.server_tools.read().await;
    let mut all_tools = Vec::new();

    for (server_id, tools) in &*server_tools {
        for tool in tools {
            // Extract the basic tool information
            let mut tool_info = serde_json::Map::new();

            // Copy the original tool properties
            if let Some(obj) = tool.as_object() {
                for (key, value) in obj {
                    tool_info.insert(key.clone(), value.clone());
                }
            }

            // Add server_id
            tool_info.insert("server_id".to_string(), json!(server_id));

            // Create a proxy ID
            let tool_id = tool.get("id").and_then(|v| v.as_str()).unwrap_or("");
            let proxy_id = format!("{}:{}", server_id, tool_id);
            tool_info.insert("proxy_id".to_string(), json!(proxy_id));

            all_tools.push(json!(tool_info));
        }
    }

    Ok(all_tools)
}
