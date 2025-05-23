use rmcp::model::JsonObject;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerId(String);

impl ServerId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// Helper function to convert rmcp::model::InputSchema to JsonObject
// This is speculative as the structure of rmcp::model::InputSchema is not fully known.
// We assume it's serializable to serde_json::Value.
fn convert_rmcp_input_schema_to_json_object(rmcp_schema: Option<rmcp::model::InputSchema>) -> JsonObject {
    match rmcp_schema {
        Some(schema) => {
            // Attempt to convert the rmcp::model::InputSchema to a serde_json::Value,
            // then try to cast that to a Map<String, Value> (JsonObject).
            // This part is highly dependent on the actual structure of rmcp::model::InputSchema.
            // A common approach is to serialize to a generic Value and check if it's an object.
            match serde_json::to_value(schema) {
                Ok(serde_json::Value::Object(map)) => map,
                Ok(_) => {
                    // Log or handle cases where it's a valid JSON value but not an object
                    log::warn!("rmcp::model::InputSchema did not convert to a JSON object, returning empty schema.");
                    JsonObject::new()
                }
                Err(e) => {
                    log::error!("Error serializing rmcp::model::InputSchema to serde_json::Value: {}. Returning empty schema.", e);
                    JsonObject::new()
                }
            }
        }
        None => JsonObject::new(),
    }
}

impl ServerToolInfo {
    // Custom conversion function from rmcp::model::Tool to ServerToolInfo.
    // It requires server_id as an additional parameter because rmcp::model::Tool doesn't contain it.
    pub fn from_rmcp_tool(tool: &rmcp::model::Tool, server_id: &str) -> Self {
        let tool_id = tool.name.clone(); // Use the tool's name as its ID within the server context
        
        // Ensure input_schema is properly cloned if Option<InputSchema> contains non-Clone types.
        // If rmcp::model::InputSchema is not Clone, this approach needs adjustment.
        // Assuming rmcp::model::InputSchema is Clone for this conversion.
        let input_schema_to_convert = tool.input_schema.clone();

        ServerToolInfo {
            id: tool_id.clone(),
            name: tool.name.clone(),
            description: tool.description.clone().unwrap_or_default(),
            input_schema: convert_rmcp_input_schema_to_json_object(input_schema_to_convert),
            server_id: server_id.to_string(),
            proxy_id: Some(format!("{}:{}", server_id, tool_id)),
            is_active: true, // Default to active, can be changed later if needed
        }
    }
}

// Conversion from ServerToolInfo to rmcp::model::Tool
// This is needed when we expose our internally managed tools via the RMCP protocol.
impl From<&ServerToolInfo> for rmcp::model::Tool {
    fn from(tool_info: &ServerToolInfo) -> Self {
        rmcp::model::Tool {
            name: std::borrow::Cow::Owned(tool_info.name.clone()),
            description: Some(std::borrow::Cow::Owned(tool_info.description.clone())),
            // input_schema in rmcp::model::Tool is Arc<Map<String, Value>>
            // input_schema in ServerToolInfo is Map<String, Value> (JsonObject)
            input_schema: std::sync::Arc::new(tool_info.input_schema.clone()),
            annotations: None, // No annotations defined for now
        }
    }
}

impl Hash for ServerId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl PartialEq for ServerId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for ServerId {}

impl fmt::Display for ServerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for ServerStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerStatus::Running => write!(f, "Running"),
            ServerStatus::Stopped => write!(f, "Stopped"),
            ServerStatus::Starting => write!(f, "Starting"),
            ServerStatus::Error(msg) => write!(f, "Error: {}", msg),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ToolType {
    #[default]
    Node,
    Python,
    Docker,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ServerStatus {
    Running,
    Stopped,
    Starting,
    #[serde(
        serialize_with = "serialize_error",
        deserialize_with = "deserialize_error"
    )]
    Error(String),
}

#[derive(Debug, Serialize)]
pub struct ToolConfigUpdateResponse {
    pub success: bool,
    pub message: String,
}

/// Tool configuration for command and arguments
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerConfiguration {
    #[serde(default)]
    pub command: Option<String>,
    #[serde(default)]
    pub args: Option<Vec<String>>,
    #[serde(default)]
    pub env: Option<HashMap<String, ServerEnvironment>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerEnvironment {
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub default: Option<String>,
    #[serde(default)]
    pub required: bool,
}

// ToolConfig struct has been removed and merged into ToolConfiguration

/// Server definition with all properties
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerDefinition {
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub tools_type: String,
    #[serde(default)]
    pub entry_point: Option<String>,
    #[serde(default)]
    pub configuration: Option<ServerConfiguration>,
    #[serde(default)]
    pub distribution: Option<Distribution>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RuntimeServer {
    #[serde(flatten)]
    pub definition: ServerDefinition,
    pub id: ServerId,
    pub status: ServerStatus,
    pub tool_count: usize,
}

/// MCP server registration request
#[derive(Debug, Deserialize)]
pub struct ServerRegistrationRequest {
    pub server_id: String,
    pub server_name: String,
    pub description: String,
    pub tools_type: String, // "node", "python", "docker"
    pub configuration: Option<ServerConfiguration>,
    pub distribution: Option<Distribution>,
}

#[derive(Debug, Deserialize)]
pub struct IsProcessRunningRequest {
    pub process_name: String,
}

/// MCP tool registration response
#[derive(Debug, Serialize)]
pub struct ServerRegistrationResponse {
    pub success: bool,
    pub message: String,
    pub tool_id: Option<String>,
}

/// MCP tool execution request
#[derive(Deserialize)]
pub struct ToolExecutionRequest {
    pub tool_id: String,
    pub parameters: Option<Map<String, Value>>,
}

/// MCP tool execution response
#[derive(Serialize, Debug)]
pub struct ToolExecutionResponse {
    pub success: bool,
    pub result: Option<Value>,
    pub error: Option<String>,
}

/// MCP tool update request
#[derive(Deserialize)]
pub struct ServerUpdateRequest {
    pub server_id: String,
    pub enabled: bool,
}

/// MCP tool update response
#[derive(Serialize, Debug)]
pub struct ToolUpdateResponse {
    pub success: bool,
    pub message: String,
}

/// MCP tool config update request
#[derive(Deserialize)]
pub struct ServerConfigUpdateRequest {
    pub server_id: String,
    pub config: HashMap<String, String>,
}

/// MCP tool uninstall request
#[derive(Deserialize)]
pub struct ToolUninstallRequest {
    pub server_id: String,
}

/// MCP tool uninstall response
#[derive(Serialize)]
pub struct ServerUninstallResponse {
    pub success: bool,
    pub message: String,
}

/// MCP server discovery request
#[derive(Serialize, Deserialize, Debug)]
pub struct DiscoverServerToolsRequest {
    pub server_id: String,
}

/// Distribution information for a tool
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Distribution {
    pub r#type: String,
    pub package: String,
}

// Custom serializer for the Error variant to format as "Error: message"
fn serialize_error<S>(error_message: &String, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&format!("Error: {}", error_message))
}

// Custom deserializer for the Error variant that handles "Error: message" format
fn deserialize_error<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.starts_with("Error: ") {
        // Use a safer way to remove the prefix
        Ok(s.trim_start_matches("Error: ").to_string())
    } else {
        Ok(s) // Return as is if it doesn't have the prefix
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Publisher {
    pub id: String,
    pub name: String,
    pub url: String,
}

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct Config {
//     pub command: String,
//     pub args: Vec<String>,
//     pub env: HashMap<String, String>,
// }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegistryTool {
    pub id: String,
    pub name: String,
    pub description: String,
    pub short_description: String,
    pub publisher: Publisher,
    pub is_official: Option<bool>,
    pub source_url: Option<String>,
    pub distribution: Distribution,
    pub license: String,
    pub runtime: String,
    pub config: ServerConfiguration,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
}

/// Response for registry tools listing
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegistryToolsResponse {
    pub count: u32,
    pub version: u64,
    pub categories: HashMap<String, u32>,
    pub tags: HashMap<String, u32>,
    pub tools: Vec<RegistryTool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigUpdateRequest {
    pub tool_id: String,
    pub config: HashMap<String, String>,
}

fn default_is_active() -> bool {
    true
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerToolInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub input_schema: JsonObject,
    pub server_id: String,
    #[serde(default)]
    pub proxy_id: Option<String>,
    #[serde(default = "default_is_active")]
    pub is_active: bool,
}

// Conversions for ServerToolInfo and DBServerTool

use crate::models::tool_db::DBServerTool; // Assuming DBServerTool is in this path

impl From<DBServerTool> for ServerToolInfo {
    fn from(db_tool: DBServerTool) -> Self {
        let input_schema = db_tool.input_schema
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_else(JsonObject::new);

        ServerToolInfo {
            id: db_tool.id,
            name: db_tool.name,
            description: db_tool.description,
            input_schema,
            server_id: db_tool.server_id,
            proxy_id: db_tool.proxy_id,
            is_active: db_tool.is_active,
        }
    }
}

impl From<&ServerToolInfo> for DBServerTool {
    fn from(tool_info: &ServerToolInfo) -> Self {
        let input_schema_str = serde_json::to_string(&tool_info.input_schema).ok();

        DBServerTool {
            id: tool_info.id.clone(),
            name: tool_info.name.clone(),
            description: tool_info.description.clone(),
            input_schema: input_schema_str,
            server_id: tool_info.server_id.clone(),
            proxy_id: tool_info.proxy_id.clone(),
            is_active: tool_info.is_active,
        }
    }
}
