use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolId(String);

impl ToolId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Hash for ToolId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl PartialEq for ToolId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for ToolId {}

impl fmt::Display for ToolId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ToolConfiguration {
    pub command: String,
    pub args: Option<Vec<String>>,
    pub env: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
pub struct ToolRegistrationRequest {
    pub tool_name: String,
    pub description: String,
    pub authentication: Option<serde_json::Value>,
    pub tool_type: ToolType,
    pub configuration: Option<ToolConfiguration>,
    pub distribution: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct ToolRegistrationResponse {
    pub success: bool,
    pub message: String,
    pub tool_id: Option<ToolId>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ToolType {
    #[default]
    Node,
    Python,
    Docker,
}

#[derive(Debug, Deserialize)]
pub struct ToolExecutionRequest {
    pub tool_id: ToolId,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct ToolExecutionResponse {
    pub success: bool,
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ToolUpdateRequest {
    pub tool_id: ToolId,
    pub enabled: bool,
}

#[derive(Debug, Serialize)]
pub struct ToolUpdateResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct ToolConfigUpdateRequest {
    pub tool_id: ToolId,
    pub config: ToolConfig,
}

#[derive(Debug, Deserialize)]
pub struct ToolConfig {
    pub env: HashMap<String, String>,
}

#[derive(Debug, Serialize)]
pub struct ToolConfigUpdateResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct ToolUninstallRequest {
    pub tool_id: ToolId,
}

#[derive(Debug, Serialize)]
pub struct ToolUninstallResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct DiscoverServerToolsRequest {
    pub server_id: ToolId,
}

#[derive(Debug, Serialize)]
pub struct DiscoverServerToolsResponse {
    pub success: bool,
    pub tools: Option<Vec<serde_json::Value>>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ToolMetadata {
    pub name: String,
    pub description: String,
    pub tool_type: ToolType,
    pub enabled: bool,
    pub configuration: Option<ToolConfiguration>,
    pub process_running: bool,
    pub tool_count: usize,
} 