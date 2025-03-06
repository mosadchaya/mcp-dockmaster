use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::domain::entities::ToolConfig;

/// MCP tool registration request DTO
#[derive(Deserialize)]
pub struct ToolRegistrationDTO {
    pub tool_name: String,
    pub description: String,
    pub authentication: Option<Value>,
    pub tool_type: String, // "node", "python", "docker"
    pub configuration: Option<Value>,
    pub distribution: Option<Value>,
}

/// MCP tool registration response DTO
#[derive(Serialize)]
pub struct ToolRegistrationResponseDTO {
    pub success: bool,
    pub message: String,
    pub tool_id: Option<String>,
}

/// MCP tool execution request DTO
#[derive(Deserialize)]
pub struct ToolExecutionDTO {
    pub tool_id: String,
    pub parameters: Value,
}

/// MCP tool execution response DTO
#[derive(Serialize)]
pub struct ToolExecutionResponseDTO {
    pub success: bool,
    pub result: Option<Value>,
    pub error: Option<String>,
}

/// MCP tool update request DTO
#[derive(Deserialize)]
pub struct ToolUpdateDTO {
    pub tool_id: String,
    pub enabled: bool,
}

/// MCP tool update response DTO
#[derive(Serialize)]
pub struct ToolUpdateResponseDTO {
    pub success: bool,
    pub message: String,
}

/// MCP tool config update response DTO
#[derive(Debug, Serialize)]
pub struct ToolConfigUpdateResponseDTO {
    pub success: bool,
    pub message: String,
}

/// MCP tool config update request DTO
#[derive(Deserialize)]
pub struct ToolConfigUpdateDTO {
    pub tool_id: String,
    pub config: ToolConfig,
}

/// MCP tool uninstall request DTO
#[derive(Deserialize)]
pub struct ToolUninstallDTO {
    pub tool_id: String,
}

/// MCP tool uninstall response DTO
#[derive(Serialize)]
pub struct ToolUninstallResponseDTO {
    pub success: bool,
    pub message: String,
}

/// MCP server discovery request DTO
#[derive(Deserialize)]
pub struct DiscoverServerToolsDTO {
    pub server_id: String,
}

/// MCP server discovery response DTO
#[derive(Serialize)]
pub struct DiscoverServerToolsResponseDTO {
    pub success: bool,
    pub tools: Option<Vec<Value>>,
    pub error: Option<String>,
}
