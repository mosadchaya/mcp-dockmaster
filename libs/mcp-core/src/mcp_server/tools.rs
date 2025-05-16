use std::{borrow::Cow, sync::Arc};

use rmcp::model::Tool;
use serde_json::json;

/// Constants for tool names
pub const TOOL_REGISTER_SERVER: &str = "mcp_register_server";
pub const TOOL_SEARCH_SERVER: &str = "mcp_search_server";
pub const TOOL_CONFIGURE_SERVER: &str = "mcp_configure_server";
pub const TOOL_UNINSTALL_SERVER: &str = "mcp_uninstall_server";
pub const TOOL_LIST_INSTALLED_SERVERS: &str = "mcp_list_installed_servers";

/// Get the list installed servers tool definition
pub fn get_list_installed_servers_tool() -> Tool {
    Tool {
        name: Cow::Owned(TOOL_LIST_INSTALLED_SERVERS.to_string()),
        description: Some(Cow::Owned("List all installed servers".to_string())),
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
    Tool {
        name: Cow::Owned(TOOL_REGISTER_SERVER.to_string()),
        description: Some(Cow::Owned(
            "Register a new server with MCP using its registry tool ID".to_string(),
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
    Tool {
        name: Cow::Owned(TOOL_SEARCH_SERVER.to_string()),
        description: Some(Cow::Owned(
            "Search for MCP Servers in the registry".to_string(),
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
    Tool {
        name: Cow::Owned(TOOL_CONFIGURE_SERVER.to_string()),
        description: Some(Cow::Owned(
            "Configure a server and its environment variables".to_string(),
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
    Tool {
        name: Cow::Owned(TOOL_UNINSTALL_SERVER.to_string()),
        description: Some(Cow::Owned(
            "Uninstall a server from MCP using its registry tool ID".to_string(),
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
