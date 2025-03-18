use mcp_sdk_core::Tool;
use serde_json::json;

/// Constants for tool names
pub const TOOL_REGISTER_SERVER: &str = "register_server";

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
