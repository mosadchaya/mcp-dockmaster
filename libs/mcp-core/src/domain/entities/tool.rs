use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};

/// Tool definition with all properties
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub tool_type: String,
    #[serde(default)]
    pub entry_point: Option<String>,
    #[serde(default)]
    pub configuration: Option<ToolConfiguration>,
    #[serde(default)]
    pub distribution: Option<Value>,
    #[serde(default)]
    pub config: Option<ToolConfig>,
    #[serde(default)]
    pub authentication: Option<Value>,
}

/// Tool configuration for command and arguments
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolConfiguration {
    pub command: String,
    #[serde(default)]
    pub args: Option<Vec<String>>,
}

/// Tool config for environment variables and optional command
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolConfig {
    #[serde(default)]
    pub env: Option<HashMap<String, String>>,
    #[serde(default)]
    pub command: Option<String>,
    #[serde(default)]
    pub args: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ToolType {
    #[default]
    Node,
    Python,
    Docker,
}

/// MCP tool registration request
#[derive(Deserialize)]
pub struct ToolRegistrationRequest {
    pub tool_name: String,
    pub description: String,
    pub authentication: Option<Value>,
    pub tool_type: String, // "node", "python", "docker"
    pub configuration: Option<Value>,
    pub distribution: Option<Value>,
}

/// MCP tool registration response
#[derive(Serialize)]
pub struct ToolRegistrationResponse {
    pub success: bool,
    pub message: String,
    pub tool_id: Option<String>,
}

/// MCP tool execution request
#[derive(Deserialize)]
pub struct ToolExecutionRequest {
    pub tool_id: String,
    pub parameters: Value,
}

/// MCP tool execution response
#[derive(Serialize)]
pub struct ToolExecutionResponse {
    pub success: bool,
    pub result: Option<Value>,
    pub error: Option<String>,
}

/// MCP tool update request
#[derive(Deserialize)]
pub struct ToolUpdateRequest {
    pub tool_id: String,
    pub enabled: bool,
}

/// MCP tool update response
#[derive(Serialize)]
pub struct ToolUpdateResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ToolConfigUpdateResponse {
    pub success: bool,
    pub message: String,
}

/// Tool config update data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolConfigUpdate {
    pub env: Option<HashMap<String, String>>,
}

/// MCP tool config update request
#[derive(Deserialize)]
pub struct ToolConfigUpdateRequest {
    pub tool_id: String,
    pub config: ToolConfigUpdate,
}

/// MCP tool uninstall request
#[derive(Deserialize)]
pub struct ToolUninstallRequest {
    pub tool_id: String,
}

/// MCP tool uninstall response
#[derive(Serialize)]
pub struct ToolUninstallResponse {
    pub success: bool,
    pub message: String,
}

/// MCP server discovery request
#[derive(Deserialize)]
pub struct DiscoverServerToolsRequest {
    pub server_id: String,
}

/// MCP server discovery response
#[derive(Serialize)]
pub struct DiscoverServerToolsResponse {
    pub success: bool,
    pub tools: Option<Vec<Value>>,
    pub error: Option<String>,
}
