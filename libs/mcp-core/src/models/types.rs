use serde::{Deserialize, Serialize};
use serde_json::Value;
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
    #[serde(serialize_with = "serialize_error", deserialize_with = "deserialize_error")]
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
pub struct ServerUpdateRequest {
    pub server_id: String,
    pub enabled: bool,
}

/// MCP tool update response
#[derive(Serialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputSchemaProperty {
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_type_field")]
    pub r#type: String,
}

// Custom deserializer for the type field that can handle both string and array
fn deserialize_type_field<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;

    let value = serde_json::Value::deserialize(deserializer)?;

    match value {
        // If it's a string, use it directly
        serde_json::Value::String(s) => Ok(s),

        // If it's an array, use the first string value or join them
        serde_json::Value::Array(arr) => {
            if arr.is_empty() {
                Ok("string".to_string()) // Default to string if empty array
            } else if let Some(first) = arr.first() {
                if let Some(s) = first.as_str() {
                    Ok(s.to_string())
                } else {
                    Err(Error::custom("Array's first element is not a string"))
                }
            } else {
                Err(Error::custom("Cannot convert empty array to type string"))
            }
        }

        // For any other type, return an error
        _ => Err(Error::custom(
            "Type field must be a string or array of strings",
        )),
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputSchema {
    #[serde(default)]
    pub properties: HashMap<String, InputSchemaProperty>,
    #[serde(default)]
    pub required: Vec<String>,
    #[serde(default)]
    pub r#type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerToolInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    #[serde(rename = "inputSchema")]
    pub input_schema: Option<InputSchema>,
    pub server_id: String,
    #[serde(default)]
    pub proxy_id: Option<String>,
    #[serde(default = "default_is_active")]
    pub is_active: bool,
}

fn default_is_active() -> bool {
    true
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
        Ok(s[7..].to_string()) // Remove "Error: " prefix
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

/// Response for server tools listing
#[derive(Debug, Serialize)]
pub struct ServerToolsResponse {
    pub tools: Vec<ServerToolInfo>,
}

/// Common error response structure
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: i32,
    pub message: String,
}

impl ServerToolInfo {
    /// Create a new ServerToolInfo from a JSON value
    pub fn from_value(value: Value, server_id: String) -> Result<ServerToolInfo, String> {
        // Generate id from name before deserializing
        let name = value
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or("missing name field")?;

        let id = name
            .to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '_' })
            .collect();

        // Create mutable copy and insert the generated fields
        let mut obj = value;

        // Handle potential array fields that should be strings
        if let Some(input_schema) = obj.get_mut("inputSchema").and_then(|v| v.as_object_mut()) {
            if let Some(properties) = input_schema
                .get_mut("properties")
                .and_then(|v| v.as_object_mut())
            {
                for (_, prop) in properties.iter_mut() {
                    if let Some(type_field) = prop.get_mut("type") {
                        // If type is an array, convert it to a string with the first value
                        if let Some(type_array) = type_field.as_array() {
                            if !type_array.is_empty() {
                                if let Some(first_type) =
                                    type_array.first().and_then(|t| t.as_str())
                                {
                                    *type_field = Value::String(first_type.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }

        obj.as_object_mut()
            .ok_or("value must be an object")?
            .insert("id".to_string(), Value::String(id));
        obj.as_object_mut()
            .unwrap()
            .insert("server_id".to_string(), Value::String(server_id));

        // Now deserialize the complete object
        serde_json::from_value(obj).map_err(|e| e.to_string())
    }
}
