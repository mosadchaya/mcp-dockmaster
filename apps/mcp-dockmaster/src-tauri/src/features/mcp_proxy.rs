use mcp_core::{
    core::{
        mcp_core::MCPCore, mcp_core_database_ext::McpCoreDatabaseExt,
        mcp_core_proxy_ext::McpCoreProxyExt,
    },
    models::types::{
        DiscoverServerToolsRequest, ServerRegistrationRequest, ServerToolInfo,
        ToolConfigUpdateRequest, ToolConfigUpdateResponse, ToolExecutionRequest,
        ToolExecutionResponse, ToolRegistrationResponse, ToolUninstallRequest,
        ToolUninstallResponse, ToolUpdateRequest, ToolUpdateResponse,
    },
    types::RuntimeServer,
};

use tauri::State;

/// Register a new tool with the MCP server
#[tauri::command]
pub async fn register_tool(
    mcp_core: State<'_, MCPCore>,
    request: ServerRegistrationRequest,
) -> Result<ToolRegistrationResponse, String> {
    mcp_core.register_tool(request).await
}

/// List all registered tools
#[tauri::command]
pub async fn list_servers(mcp_core: State<'_, MCPCore>) -> Result<Vec<RuntimeServer>, String> {
    mcp_core.list_servers().await
}

/// List all available tools from all running MCP servers
#[tauri::command]
pub async fn list_all_server_tools(
    mcp_core: State<'_, MCPCore>,
) -> Result<Vec<ServerToolInfo>, String> {
    mcp_core.list_all_server_tools().await
}

/// Discover tools from a specific MCP server
#[tauri::command]
pub async fn discover_tools(
    mcp_core: State<'_, MCPCore>,
    request: DiscoverServerToolsRequest,
) -> Result<Vec<ServerToolInfo>, String> {
    mcp_core.list_server_tools(request).await
}

/// Execute a tool from an MCP server
#[tauri::command]
pub async fn execute_proxy_tool(
    mcp_core: State<'_, MCPCore>,
    request: ToolExecutionRequest,
) -> Result<ToolExecutionResponse, String> {
    mcp_core.execute_proxy_tool(request).await
}

/// Update a tool's status (enabled/disabled)
#[tauri::command]
pub async fn update_tool_status(
    mcp_core: State<'_, MCPCore>,
    request: ToolUpdateRequest,
) -> Result<ToolUpdateResponse, String> {
    mcp_core.update_tool_status(request).await
}

/// Update a tool's configuration (environment variables)
#[tauri::command]
pub async fn update_tool_config(
    mcp_core: State<'_, MCPCore>,
    request: ToolConfigUpdateRequest,
) -> Result<ToolConfigUpdateResponse, String> {
    mcp_core.update_tool_config(request).await
}

/// Uninstall a registered tool
#[tauri::command]
pub async fn uninstall_tool(
    mcp_core: State<'_, MCPCore>,
    request: ToolUninstallRequest,
) -> Result<ToolUninstallResponse, String> {
    mcp_core.uninstall_tool(request).await
}

/// Check if the database exists and has data
#[tauri::command]
pub async fn check_database_exists_command(mcp_core: State<'_, MCPCore>) -> Result<bool, String> {
    mcp_core.check_database_exists().await
}

/// Clear all data from the database
#[tauri::command]
pub async fn clear_database_command(mcp_core: State<'_, MCPCore>) -> Result<(), String> {
    mcp_core.clear_database().await
}

/// Restart a tool by its ID
#[tauri::command(rename_all = "camelCase")]
pub async fn restart_tool_command(
    mcp_core: State<'_, MCPCore>,
    tool_id: String,
) -> Result<ToolUpdateResponse, String> {
    mcp_core.restart_tool_command(tool_id).await
}
