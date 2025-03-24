use mcp_sdk_core::Tool;
use serde_json::json;

/// Constants for tool names
pub const TOOL_REGISTER_SERVER: &str = "register_server";
pub const TOOL_SEARCH_SERVER: &str = "search_server";
pub const TOOL_CONFIGURE_SERVER: &str = "configure_server";
pub const TOOL_UNINSTALL_SERVER: &str = "uninstall_server";
/// Get the register_server tool definition
pub fn get_register_server_tool() -> Tool {
    Tool {
        name: TOOL_REGISTER_SERVER.to_string(),
        description: "Register a new server with MCP using its registry tool ID".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "tool_id": {
                    "type": "string",
                    "description": "ID of the tool in the registry to install"
                }
            },
            "required": ["tool_id"]
        }),
    }
}

pub fn get_search_server_tool() -> Tool {
    Tool {
        name: TOOL_SEARCH_SERVER.to_string(),
        description: "Search for MCP Servers in the registry".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "Search query"
                }
            },
            "required": ["query"]
        }),
    }
}

pub fn get_configure_server_tool() -> Tool {
    Tool {
        name: TOOL_CONFIGURE_SERVER.to_string(),
        description: "Configure a server and its environment variables".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "server_id": {
                    "type": "string",
                    "description": "ID of the server to configure"
                },
                "config": {
                    "type": "object",
                    "description": "Configuration for the server"
                }
            },
            "required": ["server_id", "config"]
        }),
    }
}

/// Get the uninstall_server tool definition
pub fn get_uninstall_server_tool() -> Tool {
    Tool {
        name: TOOL_UNINSTALL_SERVER.to_string(),
        description: "Uninstall a server from MCP using its registry tool ID".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "server_id": {
                    "type": "string",
                    "description": "ID of the server to uninstall"
                }
            },
            "required": ["server_id"]
        }),
    }
}