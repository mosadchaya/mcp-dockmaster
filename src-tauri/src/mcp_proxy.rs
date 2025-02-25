use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tauri::State;

/// Structure to hold registered tools
#[derive(Default)]
pub struct ToolRegistry {
    tools: HashMap<String, Value>,
}

/// State management for registered tools
pub struct MCPState {
    pub tool_registry: Arc<Mutex<ToolRegistry>>,
}

impl Default for MCPState {
    fn default() -> Self {
        Self {
            tool_registry: Arc::new(Mutex::new(ToolRegistry::default())),
        }
    }
}

/// MCP tool registration request
#[derive(Deserialize)]
pub struct ToolRegistrationRequest {
    tool_name: String,
    description: String,
    authentication: Option<Value>,
}

/// MCP tool registration response
#[derive(Serialize)]
pub struct ToolRegistrationResponse {
    success: bool,
    message: String,
    tool_id: Option<String>,
}

/// MCP tool execution request
#[derive(Deserialize)]
pub struct ToolExecutionRequest {
    tool_id: String,
    parameters: Value,
}

/// MCP tool execution response
#[derive(Serialize)]
pub struct ToolExecutionResponse {
    success: bool,
    result: Option<Value>,
    error: Option<String>,
}

/// MCP tool update request
#[derive(Deserialize)]
pub struct ToolUpdateRequest {
    tool_id: String,
    enabled: bool,
}

/// MCP tool update response
#[derive(Serialize)]
pub struct ToolUpdateResponse {
    success: bool,
    message: String,
}

/// MCP tool uninstall request
#[derive(Deserialize)]
pub struct ToolUninstallRequest {
    tool_id: String,
}

/// MCP tool uninstall response
#[derive(Serialize)]
pub struct ToolUninstallResponse {
    success: bool,
    message: String,
}

/// Register a new tool with the MCP server
#[tauri::command]
pub fn register_tool(
    state: State<MCPState>,
    request: ToolRegistrationRequest,
) -> ToolRegistrationResponse {
    let mut registry = state.tool_registry.lock().unwrap();
    
    // Generate a simple tool ID (in production, use UUIDs)
    let tool_id = format!("tool_{}", registry.tools.len() + 1);
    
    // Store the tool definition
    registry.tools.insert(
        tool_id.clone(),
        json!({
            "name": request.tool_name,
            "description": request.description,
            "authentication": request.authentication,
            "enabled": true, // Default to enabled
        }),
    );
    
    ToolRegistrationResponse {
        success: true,
        message: format!("Tool '{}' registered successfully", request.tool_name),
        tool_id: Some(tool_id),
    }
}

/// List all registered tools
#[tauri::command]
pub fn list_tools(state: State<MCPState>) -> Vec<Value> {
    let registry = state.tool_registry.lock().unwrap();
    let mut tools = Vec::new();
    
    for (id, mut tool_value) in registry.tools.iter() {
        // Clone the value so we can modify it
        let mut tool = tool_value.clone();
        
        // Ensure the tool has an ID field
        if let Some(obj) = tool.as_object_mut() {
            obj.insert("id".to_string(), json!(id));
        }
        
        tools.push(tool);
    }
    
    tools
}

/// Update a tool's status (enabled/disabled)
#[tauri::command]
pub fn update_tool_status(
    state: State<MCPState>,
    request: ToolUpdateRequest,
) -> ToolUpdateResponse {
    let mut registry = state.tool_registry.lock().unwrap();
    
    // Check if the tool exists
    if let Some(tool) = registry.tools.get_mut(&request.tool_id) {
        // Update the enabled status
        if let Some(obj) = tool.as_object_mut() {
            obj.insert("enabled".to_string(), json!(request.enabled));
        }
        
        ToolUpdateResponse {
            success: true,
            message: format!("Tool status updated successfully"),
        }
    } else {
        ToolUpdateResponse {
            success: false,
            message: format!("Tool with ID '{}' not found", request.tool_id),
        }
    }
}

/// Uninstall a registered tool
#[tauri::command]
pub fn uninstall_tool(
    state: State<MCPState>,
    request: ToolUninstallRequest,
) -> ToolUninstallResponse {
    let mut registry = state.tool_registry.lock().unwrap();
    
    // Remove the tool from the registry
    if registry.tools.remove(&request.tool_id).is_some() {
        ToolUninstallResponse {
            success: true,
            message: format!("Tool uninstalled successfully"),
        }
    } else {
        ToolUninstallResponse {
            success: false,
            message: format!("Tool with ID '{}' not found", request.tool_id),
        }
    }
}

/// Execute a registered tool
#[tauri::command]
pub fn execute_tool(
    state: State<MCPState>,
    request: ToolExecutionRequest,
) -> ToolExecutionResponse {
    let registry = state.tool_registry.lock().unwrap();
    
    // Check if the tool exists
    if let Some(tool) = registry.tools.get(&request.tool_id) {
        // In a real implementation, this would actually invoke the tool
        // Here we just echo back the tool information and parameters as a proof of concept
        ToolExecutionResponse {
            success: true,
            result: Some(json!({
                "tool": tool,
                "parameters": request.parameters,
                "result": "Hello from MCP Tool!"
            })),
            error: None,
        }
    } else {
        ToolExecutionResponse {
            success: false,
            result: None,
            error: Some(format!("Tool with ID '{}' not found", request.tool_id)),
        }
    }
}

/// Hello world test command for MCP
#[tauri::command]
pub fn mcp_hello_world() -> Value {
    json!({
        "message": "Hello from MCP Server Proxy!",
        "status": "OK"
    })
} 