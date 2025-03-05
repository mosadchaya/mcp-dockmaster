use crate::database;
use crate::error::{MCPError, MCPResult};
use crate::models::{ToolId, ToolType, ToolConfiguration, ToolMetadata};
use crate::registry::{ToolRegistry as RegistryToolRegistry, MCPState};
use crate::registry::ToolRegistry;
use log::{error, info};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::process::Stdio;
use std::collections::HashMap;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt},
    process::{Child, Command},
};

// Legacy ToolRegistry struct for database compatibility
#[derive(Default)]
pub struct LegacyToolRegistry {
    pub tools: HashMap<String, Value>,
    pub server_tools: HashMap<String, Vec<Value>>,
}

// Note: The ToolRegistry and MCPState structs have been moved to registry.rs

/// MCP tool registration request
#[derive(Deserialize)]
pub struct ToolRegistrationRequest {
    tool_name: String,
    description: String,
    authentication: Option<Value>,
    tool_type: String, // "node", "python", "docker"
    configuration: Option<Value>,
    distribution: Option<Value>,
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

/// MCP tool config update request
#[derive(Deserialize)]
pub struct ToolConfigUpdateRequest {
    tool_id: String,
    config: ToolConfig,
}

/// MCP tool config
#[derive(Deserialize)]
pub struct ToolConfig {
    env: HashMap<String, String>,
}

/// MCP tool config update response
#[derive(Serialize)]
pub struct ToolConfigUpdateResponse {
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

/// MCP server discovery request
#[derive(Deserialize)]
pub struct DiscoverServerToolsRequest {
    server_id: String,
}

/// MCP server discovery response
#[derive(Serialize)]
pub struct DiscoverServerToolsResponse {
    success: bool,
    tools: Option<Vec<Value>>,
    error: Option<String>,
}

// MCPError is now imported from crate::error

/// Discover tools available from an MCP server
async fn discover_server_tools(
    server_id: &str,
    registry: &mut crate::registry::ToolRegistry,
) -> Result<Vec<Value>, String> {
    // Convert string server_id to ToolId
    let tool_id = ToolId::new(server_id.to_string());
    
    info!("Discovering tools from server {}", server_id);

    // Use the discover_tools method from the registry
    registry.discover_tools(&tool_id).await.map_err(|e| e.to_string())
}

/// Execute a tool on an MCP server
async fn execute_server_tool(
    server_id: &str,
    tool_name: &str,
    parameters: Value,
    registry: &mut crate::registry::ToolRegistry,
) -> Result<Value, MCPError> {
    // Convert string server_id to ToolId
    let tool_id = ToolId::new(server_id.to_string());
    
    // Use the execute_tool method from the registry
    registry.execute_tool(&tool_id, tool_name, parameters).await
        .map_err(|e| match e {
            MCPError::ServerNotFound(_) => MCPError::ServerNotFound(server_id.to_string()),
            MCPError::TimeoutError(_) => MCPError::TimeoutError(server_id.to_string()),
            other => other,
        })
}

/// Spawn a Node.js MCP server process
async fn spawn_nodejs_process(
    configuration: &Value,
    tool_id: &str,
    env_vars: Option<&HashMap<String, String>>,
) -> Result<
    (
        Child,
        tokio::process::ChildStdin,
        tokio::process::ChildStdout,
    ),
    String,
> {
    info!("Spawning Node.js process for tool ID: {}", tool_id);
    info!("Configuration: {}", configuration);

    let mut cmd;

    let command = configuration
        .get("command")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            error!(
                "Configuration missing 'command' field or not a string for tool: {}",
                tool_id
            );
            format!("Configuration missing 'command' field or not a string")
        })?;

    if command.contains("npx") || command.contains("node") {
        info!(
            "Using command to start process for tool {}: {}",
            tool_id, command
        );
        cmd = Command::new(command);
        // Add args if they exist
        if let Some(args) = configuration.get("args").and_then(|v| v.as_array()) {
            info!(
                "Adding {} arguments to command for tool {}",
                args.len(),
                tool_id
            );
            for (i, arg) in args.iter().enumerate() {
                if let Some(arg_str) = arg.as_str() {
                    info!("Arg {}: {}", i, arg_str);
                    cmd.arg(arg_str);
                }
            }
        } else {
            info!("No arguments found in configuration for tool {}", tool_id);
        }

        info!("Adding tool-id argument: {}", tool_id);
        cmd.arg("--tool-id").arg(tool_id);

        cmd.stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
    } else {
        // Otherwise, assume it's a file path that doesn't exist yet
        error!("Entry point doesn't exist and doesn't look like an npm package or node command for tool {}: {}", tool_id, command);
        return Err(format!("Entry point file '{}' does not exist", command));
    }

    // Add environment variables if provided
    if let Some(env_map) = env_vars {
        info!(
            "Setting {} environment variables for tool {}",
            env_map.len(),
            tool_id
        );
        for (key, value) in env_map {
            info!(
                "Setting environment variable for tool {}: {}={}",
                tool_id, key, value
            );
            cmd.env(key, value);
        }
    } else {
        info!("No environment variables provided for tool {}", tool_id);
    }

    // Log the command we're about to run
    info!(
        "Spawning process for tool {}: {:?} with args: {:?}",
        tool_id,
        cmd.as_std().get_program(),
        cmd.as_std().get_args().collect::<Vec<_>>()
    );

    // Spawn the process
    let mut child = match cmd.spawn() {
        Ok(child) => {
            info!("Successfully spawned process for tool {}", tool_id);
            child
        }
        Err(e) => {
            error!("Failed to spawn process for tool {}: {}", tool_id, e);
            return Err(format!("Failed to spawn process: {}", e));
        }
    };

    // Capture stderr to a separate task that logs errors
    if let Some(stderr) = child.stderr.take() {
        let tool_id_clone = tool_id.to_string();
        tokio::spawn(async move {
            let mut stderr_reader = tokio::io::BufReader::new(stderr);
            let mut line = String::new();
            while let Ok(bytes_read) = stderr_reader.read_line(&mut line).await {
                if bytes_read == 0 {
                    break;
                }
                info!("[{} stderr]: {}", tool_id_clone, line.trim());
                line.clear();
            }
        });
    }

    // Take ownership of the pipes
    let stdin = match child.stdin.take() {
        Some(stdin) => stdin,
        None => {
            error!("Failed to open stdin for process");
            if let Err(e) = child.kill().await {
                error!("Failed to kill process after stdin error: {}", e);
            }
            return Err(String::from("Failed to open stdin"));
        }
    };

    let stdout = match child.stdout.take() {
        Some(stdout) => stdout,
        None => {
            error!("Failed to open stdout for process");
            if let Err(e) = child.kill().await {
                error!("Failed to kill process after stdout error: {}", e);
            }
            return Err(String::from("Failed to open stdout"));
        }
    };

    info!("Process spawned successfully with stdin and stdout pipes");
    // Return the process and pipes
    Ok((child, stdin, stdout))
}

/// Spawn a Python MCP server process
async fn spawn_python_process(
    configuration: &Value,
    tool_id: &str,
    env_vars: Option<&HashMap<String, String>>,
) -> Result<
    (
        Child,
        tokio::process::ChildStdin,
        tokio::process::ChildStdout,
    ),
    String,
> {
    let mut cmd;

    let command = configuration
        .get("command")
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("Configuration missing 'command' field or not a string"))?;

    info!("Using Python command: {}", command);
    cmd = Command::new(command);

    // Add args if they exist
    info!("Args: {:?}", configuration.get("args"));
    if let Some(args) = configuration.get("args").and_then(|v| v.as_array()) {
        for arg in args {
            if let Some(arg_str) = arg.as_str() {
                cmd.arg(arg_str);
            }
        }
    }

    cmd.stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    // Add environment variables if provided
    if let Some(env_map) = env_vars {
        for (key, value) in env_map {
            info!("Setting environment variable: {}={}", key, value);
            cmd.env(key, value);
        }
    }

    // Log the command we're about to run
    info!(
        "Spawning Python process: {:?} with args: {:?}",
        cmd.as_std().get_program(),
        cmd.as_std().get_args().collect::<Vec<_>>()
    );

    // Spawn the process
    let mut child = match cmd.spawn() {
        Ok(child) => child,
        Err(e) => {
            error!("Failed to spawn Python process: {}", e);
            return Err(format!("Failed to spawn Python process: {}", e));
        }
    };

    // Capture stderr to a separate task that logs errors
    if let Some(stderr) = child.stderr.take() {
        let tool_id_clone = tool_id.to_string();
        tokio::spawn(async move {
            let mut stderr_reader = tokio::io::BufReader::new(stderr);
            let mut line = String::new();
            while let Ok(bytes_read) = stderr_reader.read_line(&mut line).await {
                if bytes_read == 0 {
                    break;
                }
                info!("[{} stderr]: {}", tool_id_clone, line.trim());
                line.clear();
            }
        });
    }

    // Take ownership of the pipes
    let stdin = match child.stdin.take() {
        Some(stdin) => stdin,
        None => {
            error!("Failed to open stdin for Python process");
            if let Err(e) = child.kill().await {
                error!("Failed to kill process after stdin error: {}", e);
            }
            return Err(String::from("Failed to open stdin"));
        }
    };

    let stdout = match child.stdout.take() {
        Some(stdout) => stdout,
        None => {
            error!("Failed to open stdout for Python process");
            if let Err(e) = child.kill().await {
                error!("Failed to kill process after stdout error: {}", e);
            }
            return Err(String::from("Failed to open stdout"));
        }
    };

    info!("Python process spawned successfully with stdin and stdout pipes");
    // Return the process and pipes
    Ok((child, stdin, stdout))
}

/// Spawn a Docker MCP server process
async fn spawn_docker_process(
    configuration: &Value,
    tool_id: &str,
    env_vars: Option<&HashMap<String, String>>,
) -> Result<
    (
        Child,
        tokio::process::ChildStdin,
        tokio::process::ChildStdout,
    ),
    String,
> {
    let command = configuration
        .get("command")
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("Configuration missing 'command' field or not a string"))?;

    if command != "docker" {
        return Err(format!(
            "Expected 'docker' command for Docker runtime, got '{}'",
            command
        ));
    }

    info!("Using Docker command");
    let mut cmd = Command::new(command);

    cmd.arg("run")
        .arg("-i")
        .arg("--name")
        .arg(tool_id)
        .arg("-a")
        .arg("stdout")
        .arg("-a")
        .arg("stderr")
        .arg("-a")
        .arg("stdin");

    // Add environment variables as Docker -e flags if provided
    if let Some(env_map) = env_vars {
        for (key, value) in env_map {
            info!("Setting Docker environment variable: {}={}", key, value);
            cmd.arg("-e").arg(format!("{}={}", key, value));
        }
    }

    cmd.arg("--rm");

    // Add args if they exist
    info!("Args: {:?}", configuration.get("args"));
    if let Some(args) = configuration.get("args").and_then(|v| v.as_array()) {
        for arg in args {
            if let Some(arg_str) = arg.as_str() {
                cmd.arg(arg_str);
            }
        }
    }

    cmd.stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    // Log the command we're about to run
    info!(
        "Spawning Docker process: {:?} with args: {:?}",
        cmd.as_std().get_program(),
        cmd.as_std().get_args().collect::<Vec<_>>()
    );

    // Spawn the process
    let mut child = match cmd.spawn() {
        Ok(child) => child,
        Err(e) => {
            error!("Failed to spawn Docker process: {}", e);
            return Err(format!("Failed to spawn Docker process: {}", e));
        }
    };

    // Capture stderr to a separate task that logs errors
    if let Some(stderr) = child.stderr.take() {
        let tool_id_clone = tool_id.to_string();
        tokio::spawn(async move {
            let mut stderr_reader = tokio::io::BufReader::new(stderr);
            let mut line = String::new();
            while let Ok(bytes_read) = stderr_reader.read_line(&mut line).await {
                if bytes_read == 0 {
                    break;
                }
                info!("[{} stderr]: {}", tool_id_clone, line.trim());
                line.clear();
            }
        });
    }

    // Take ownership of the pipes
    let stdin = match child.stdin.take() {
        Some(stdin) => stdin,
        None => {
            error!("Failed to open stdin for Docker process");
            if let Err(e) = child.kill().await {
                error!("Failed to kill process after stdin error: {}", e);
            }
            return Err(String::from("Failed to open stdin"));
        }
    };

    let stdout = match child.stdout.take() {
        Some(stdout) => stdout,
        None => {
            error!("Failed to open stdout for Docker process");
            if let Err(e) = child.kill().await {
                error!("Failed to kill process after stdout error: {}", e);
            }
            return Err(String::from("Failed to open stdout"));
        }
    };

    info!("Docker process spawned successfully with stdin and stdout pipes");
    // Return the process and pipes
    Ok((child, stdin, stdout))
}

/// Kill a running process
async fn kill_process(process: &mut Child) -> Result<(), String> {
    match process.kill().await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to kill process: {}", e)),
    }
}

/// Register a new tool with the MCP server
pub async fn register_tool(
    mcp_state: &MCPState,
    request: ToolRegistrationRequest,
) -> Result<ToolRegistrationResponse, String> {
    info!("Starting registration of tool: {}", request.tool_name);

    // Safely access the command field if configuration exists
    if let Some(config) = &request.configuration {
        if let Some(cmd) = config.get("command") {
            info!("Command: {}", cmd);
        } else {
            info!("Command: Not specified in configuration");
        }
    } else {
        info!("Configuration not provided");
    }

    // Convert the request to the new format
    let tool_request = crate::models::ToolRegistrationRequest {
        tool_name: request.tool_name.clone(),
        description: request.description.clone(),
        tool_type: match request.tool_type.as_str() {
            "node" => ToolType::Node,
            "python" => ToolType::Python,
            "docker" => ToolType::Docker,
            _ => return Err(format!("Unsupported tool type: {}", request.tool_type)),
        },
        configuration: if let Some(config) = &request.configuration {
            // Convert the config to the new format
            let command = config.get("command")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            
            let mut args = None;
            if let Some(args_array) = config.get("args").and_then(|v| v.as_array()) {
                let args_vec: Vec<String> = args_array.iter()
                    .filter_map(|a| a.as_str().map(|s| s.to_string()))
                    .collect();
                if !args_vec.is_empty() {
                    args = Some(args_vec);
                }
            }
            
            let mut env = None;
            if let Some(auth) = &request.authentication {
                if let Some(env_obj) = auth.get("env").and_then(|v| v.as_object()) {
                    let mut env_map = std::collections::HashMap::new();
                    for (key, value) in env_obj {
                        if let Some(value_str) = value.as_str() {
                            env_map.insert(key.clone(), value_str.to_string());
                        } else {
                            env_map.insert(key.clone(), value.to_string());
                        }
                    }
                    if !env_map.is_empty() {
                        env = Some(env_map);
                    }
                }
            }
            
            Some(ToolConfiguration {
                command,
                args,
                env,
            })
        } else {
            None
        },
        authentication: request.authentication.clone(),
        distribution: None,
    };

    // Use the register_tool method from the registry
    let mut registry = mcp_state.tool_registry.write().await;
    match registry.register_tool(tool_request).await {
        Ok(response) => {
            // Convert the response back to the old format
            Ok(ToolRegistrationResponse {
                success: response.success,
                message: response.message,
                tool_id: response.tool_id.map(|id| id.to_string()),
            })
        }
        Err(e) => {
            error!("Failed to register tool: {}", e);
            Ok(ToolRegistrationResponse {
                success: false,
                message: format!("Failed to register tool: {}", e),
                tool_id: None,
            })
        }
    }
}

/// List all registered tools
pub async fn list_tools(mcp_state: &MCPState) -> Result<Vec<Value>, String> {
    let registry = mcp_state.tool_registry.read().await;
    let mut tools = Vec::new();

    for (id, metadata) in registry.tools.iter() {
        // Create a JSON representation of the tool
        let mut tool = json!({
            "id": id.to_string(),
            "name": metadata.name,
            "description": metadata.description,
            "enabled": metadata.enabled,
            "tool_type": match metadata.tool_type {
                ToolType::Node => "node",
                ToolType::Python => "python",
                ToolType::Docker => "docker",
            },
            "process_running": metadata.process_running,
            "tool_count": metadata.tool_count,
        });

        // Add configuration if available
        if let Some(config) = &metadata.configuration {
            let mut config_json = json!({
                "command": config.command,
            });
            
            if let Some(args) = &config.args {
                config_json["args"] = json!(args);
            }
            
            if let Some(env) = &config.env {
                config_json["env"] = json!(env);
            }
            
            if let Some(obj) = tool.as_object_mut() {
                obj.insert("configuration".to_string(), config_json);
            }
        }

        tools.push(tool);
    }
    Ok(tools)
}

/// List all available tools from all running MCP servers
pub async fn list_all_server_tools(mcp_state: &MCPState) -> Result<Vec<Value>, String> {
    let registry = mcp_state.tool_registry.read().await;
    let mut all_tools = Vec::new();

    for (server_id, tools) in &registry.server_tools {
        for tool in tools {
            // Extract the basic tool information
            let mut tool_info = serde_json::Map::new();

            // Copy the original tool properties
            if let Some(obj) = tool.as_object() {
                for (key, value) in obj {
                    tool_info.insert(key.clone(), value.clone());
                }
            }

            // Add server_id
            tool_info.insert("server_id".to_string(), json!(server_id.to_string()));

            // Create a proxy ID
            let tool_id = tool.get("id").and_then(|v| v.as_str()).unwrap_or("");
            let proxy_id = format!("{}:{}", server_id, tool_id);
            tool_info.insert("proxy_id".to_string(), json!(proxy_id));

            all_tools.push(json!(tool_info));
        }
    }

    Ok(all_tools)
}

/// Discover tools from a specific MCP server
pub async fn discover_tools(
    mcp_state: &MCPState,
    request: DiscoverServerToolsRequest,
) -> Result<DiscoverServerToolsResponse, String> {
    // Convert string server_id to ToolId
    let tool_id = ToolId::new(request.server_id.clone());
    
    // Check if the server exists and is running
    let server_running = {
        let registry = mcp_state.tool_registry.read().await;
        let metadata = registry.tools.get(&tool_id);
        metadata.map_or(false, |m| m.process_running)
    };

    if !server_running {
        return Ok(DiscoverServerToolsResponse {
            success: false,
            tools: None,
            error: Some(format!(
                "Server with ID '{}' is not running",
                request.server_id
            )),
        });
    }

    // Discover tools from the server
    let mut registry = mcp_state.tool_registry.write().await;
    match discover_server_tools(&request.server_id, &mut registry).await {
        Ok(tools) => {
            // Store the discovered tools
            registry
                .server_tools
                .insert(tool_id, tools.clone());

            Ok(DiscoverServerToolsResponse {
                success: true,
                tools: Some(tools),
                error: None,
            })
        }
        Err(e) => Ok(DiscoverServerToolsResponse {
            success: false,
            tools: None,
            error: Some(format!("Failed to discover tools: {}", e)),
        }),
    }
}

/// Execute a tool from an MCP server
pub async fn execute_proxy_tool(
    mcp_state: &MCPState,
    request: ToolExecutionRequest,
) -> Result<ToolExecutionResponse, String> {
    // Extract server_id and tool_id from the proxy_id
    let parts: Vec<&str> = request.tool_id.split(':').collect();
    if parts.len() != 2 {
        return Err("Invalid tool_id format. Expected 'server_id:tool_id'".to_string());
    }

    let server_id = parts[0];
    let tool_id = parts[1];

    // Convert string server_id to ToolId
    let tool_id_obj = ToolId::new(server_id.to_string());

    // Execute the tool on the server
    let mut registry = mcp_state.tool_registry.write().await;
    match registry.execute_tool(&tool_id_obj, tool_id, request.parameters.clone()).await {
        Ok(result) => Ok(ToolExecutionResponse {
            success: true,
            result: Some(result),
            error: None,
        }),
        Err(e) => Ok(ToolExecutionResponse {
            success: false,
            result: None,
            error: Some(e.to_string()),
        }),
    }
}

/// Update a tool's status (enabled/disabled)
pub async fn update_tool_status(
    mcp_state: &MCPState,
    request: ToolUpdateRequest,
) -> Result<ToolUpdateResponse, String> {
    // Convert string tool_id to ToolId
    let tool_id = ToolId::new(request.tool_id.clone());
    
    // First, check if the tool exists and get the necessary information
    let tool_info = {
        let registry = mcp_state.tool_registry.read().await;

        if let Some(metadata) = registry.tools.get(&tool_id) {
            // Extract and clone the necessary values
            let tool_type = match metadata.tool_type {
                ToolType::Node => "node",
                ToolType::Python => "python",
                ToolType::Docker => "docker",
            }.to_string();

            // We don't have entry_point in the new structure, use empty string
            let entry_point = "".to_string();

            let process_running = registry
                .processes
                .get(&tool_id)
                .is_some_and(|p| p.is_some());

            Some((tool_type, entry_point, process_running))
        } else {
            None
        }
    };

    // If the tool doesn't exist, return an error
    if tool_info.is_none() {
        return Ok(ToolUpdateResponse {
            success: false,
            message: format!("Tool with ID '{}' not found", request.tool_id),
        });
    }

    // Now handle the process based on the enabled status
    let result = {
        let mut registry = mcp_state.tool_registry.write().await;

        // Update the enabled status in the tool definition
        if let Some(metadata) = registry.tools.get_mut(&tool_id) {
            metadata.enabled = request.enabled;
        }

        // Drop the write lock before trying to restart the tool
        drop(registry);

        if request.enabled {
            let mut registry = mcp_state.tool_registry.write().await;
            registry.restart_tool(&tool_id).await
        } else {
            Ok(())
        }
    };

    // Handle any errors from the process management
    if let Err(e) = result {
        return Ok(ToolUpdateResponse {
            success: false,
            message: e.to_string(),
        });
    }

    // Save the updated state to the database
    if let Err(e) = RegistryToolRegistry::save_mcp_state(mcp_state).await {
        error!("Failed to save MCP state after updating tool status: {}", e);
        // Continue even if saving fails
    } else {
        info!("Successfully saved MCP state after updating tool status");
    }

    // Return success
    Ok(ToolUpdateResponse {
        success: true,
        message: format!(
            "Tool '{}' status updated to {}",
            request.tool_id,
            if request.enabled {
                "enabled"
            } else {
                "disabled"
            }
        ),
    })
}

/// Update a tool's configuration (environment variables)
pub async fn update_tool_config(
    mcp_state: &MCPState,
    request: ToolConfigUpdateRequest,
) -> Result<ToolConfigUpdateResponse, String> {
    info!("Updating configuration for tool: {}", request.tool_id);

    // Convert string tool_id to ToolId
    let tool_id = ToolId::new(request.tool_id.clone());
    
    // First, check if the tool exists
    let tool_exists = {
        let registry = mcp_state.tool_registry.read().await;
        registry.tools.contains_key(&tool_id)
    };

    // If the tool doesn't exist, return an error
    if !tool_exists {
        error!("Tool with ID '{}' not found", request.tool_id);
        return Ok(ToolConfigUpdateResponse {
            success: false,
            message: format!("Tool with ID '{}' not found", request.tool_id),
        });
    }

    info!("Tool '{}' found, updating configuration", request.tool_id);

    // Update the tool configuration
    let mut registry = mcp_state.tool_registry.write().await;

    // Update the configuration in the tool metadata
    if let Some(metadata) = registry.tools.get_mut(&tool_id) {
        // Create or update the environment variables in the configuration
        let mut env_map = HashMap::new();
        for (key, value) in &request.config.env {
            info!(
                "Setting environment variable for tool {}: {}={}",
                request.tool_id, key, value
            );
            env_map.insert(key.clone(), value.clone());
        }
        
        // Update or create the configuration
        if let Some(config) = &mut metadata.configuration {
            // Update existing configuration
            config.env = Some(env_map);
        } else {
            // Create new configuration
            metadata.configuration = Some(ToolConfiguration {
                command: String::new(), // Default empty command
                args: None,
                env: Some(env_map),
            });
        }
    }

    // Release the registry lock before saving state
    drop(registry);

    // Save the updated state to the database
    if let Err(e) = RegistryToolRegistry::save_mcp_state(mcp_state).await {
        error!("Failed to save MCP state after updating tool config: {}", e);
        // Continue even if saving fails
    } else {
        info!(
            "Successfully saved MCP state after updating tool config for tool: {}",
            request.tool_id
        );
    }

    // Return success
    Ok(ToolConfigUpdateResponse {
        success: true,
        message: format!("Tool '{}' configuration updated", request.tool_id),
    })
}

/// Uninstall a registered tool
pub async fn uninstall_tool(
    mcp_state: &MCPState,
    request: ToolUninstallRequest,
) -> Result<ToolUninstallResponse, String> {
    // Convert string tool_id to ToolId
    let tool_id = ToolId::new(request.tool_id.clone());
    
    let mut registry = mcp_state.tool_registry.write().await;

    // Stop the tool if it's running
    if let Some(metadata) = registry.tools.get(&tool_id) {
        if metadata.process_running {
            // Use kill_all_processes instead of stop_tool
            if let Err(e) = registry.kill_all_processes().await {
                error!("Failed to stop tool before uninstalling: {}", e);
                // Continue with uninstallation even if stopping fails
            }
        }
    }

    // Remove the tool from the registry
    if registry.tools.remove(&tool_id).is_some() {
        registry.server_tools.remove(&tool_id);

        Ok(ToolUninstallResponse {
            success: true,
            message: "Tool uninstalled successfully".to_string(),
        })
    } else {
        Ok(ToolUninstallResponse {
            success: false,
            message: format!("Tool with ID '{}' not found", request.tool_id),
        })
    }
}

/// Execute a registered tool
pub async fn execute_tool(
    mcp_state: &MCPState,
    request: ToolExecutionRequest,
) -> Result<ToolExecutionResponse, String> {
    // Convert string tool_id to ToolId
    let tool_id = ToolId::new(request.tool_id.clone());
    
    // Shortcut to execute_proxy_tool using a direct tool ID
    let registry = mcp_state.tool_registry.read().await;

    // Check if the tool exists
    if let Some(metadata) = registry.tools.get(&tool_id) {
        // Check if the process is running
        if !metadata.process_running {
            return Ok(ToolExecutionResponse {
                success: false,
                result: None,
                error: Some(format!("Tool with ID '{}' is not running", request.tool_id)),
            });
        }

        drop(registry);

        // Use the proxy format (server_id:tool_id)
        let proxy_request = ToolExecutionRequest {
            tool_id: format!("{}:main", request.tool_id),
            parameters: request.parameters,
        };

        // Execute the tool through the proxy
        execute_proxy_tool(mcp_state, proxy_request).await
    } else {
        Ok(ToolExecutionResponse {
            success: false,
            result: None,
            error: Some(format!("Tool with ID '{}' not found", request.tool_id)),
        })
    }
}

/// Get all server data in a single function to avoid multiple locks
pub async fn get_all_server_data(mcp_state: &MCPState) -> Result<Value, String> {
    // Acquire a single read lock for all operations
    let registry = mcp_state.tool_registry.read().await;

    // 1. Get registered servers
    let mut servers = Vec::new();
    for (id, metadata) in registry.tools.iter() {
        // Create a JSON representation of the tool
        let mut tool = json!({
            "id": id.to_string(),
            "name": metadata.name,
            "description": metadata.description,
            "enabled": metadata.enabled,
            "tool_type": match metadata.tool_type {
                ToolType::Node => "node",
                ToolType::Python => "python",
                ToolType::Docker => "docker",
            },
            "process_running": metadata.process_running,
            "tool_count": metadata.tool_count,
        });

        // Add configuration if available
        if let Some(config) = &metadata.configuration {
            let mut config_json = json!({
                "command": config.command,
            });
            
            if let Some(args) = &config.args {
                config_json["args"] = json!(args);
            }
            
            if let Some(env) = &config.env {
                config_json["env"] = json!(env);
            }
            
            if let Some(obj) = tool.as_object_mut() {
                obj.insert("configuration".to_string(), config_json);
            }
        }

        servers.push(tool);
    }

    // 2. Get all server tools
    let mut all_tools = Vec::new();
    for (server_id, tools) in &registry.server_tools {
        for tool in tools {
            // Extract the basic tool information
            let mut tool_info = serde_json::Map::new();

            // Copy the original tool properties
            if let Some(obj) = tool.as_object() {
                for (key, value) in obj {
                    tool_info.insert(key.clone(), value.clone());
                }
            }

            // Add server_id
            tool_info.insert("server_id".to_string(), json!(server_id.to_string()));

            // Create a proxy ID
            let tool_id = tool.get("id").and_then(|v| v.as_str()).unwrap_or("");
            let proxy_id = format!("{}:{}", server_id, tool_id);
            tool_info.insert("proxy_id".to_string(), json!(proxy_id));

            all_tools.push(json!(tool_info));
        }
    }

    // Return all data in one response
    Ok(json!({
        "servers": servers,
        "tools": all_tools
    }))
}

/// Save the current MCP state to the database
pub async fn save_mcp_state_command(mcp_state: &MCPState) -> Result<String, String> {
    match RegistryToolRegistry::save_mcp_state(mcp_state).await {
        Ok(_) => Ok("MCP state saved successfully".to_string()),
        Err(e) => Err(format!("Failed to save MCP state: {}", e)),
    }
}

/// Load MCP state from the database
pub async fn load_mcp_state_command(mcp_state: &MCPState) -> Result<String, String> {
    match database::DatabaseManager::new() {
        Ok(db_manager) => {
            match db_manager.load_tool_registry() {
                Ok(_legacy_registry) => {
                    // Initialize the MCP state using the init_mcp_state function
                    // which will properly convert the legacy registry format
                    let loaded_state = RegistryToolRegistry::init_mcp_state().await;
                    
                    // Copy the loaded state to the provided mcp_state
                    let loaded_registry = loaded_state.tool_registry.read().await;
                    let mut state_registry = mcp_state.tool_registry.write().await;
                    
                    // Copy tools and server_tools
                    state_registry.tools = loaded_registry.tools.clone();
                    state_registry.server_tools = loaded_registry.server_tools.clone();
                    
                    // Note: processes are not persisted and will be initialized as needed

                    Ok("MCP state loaded successfully".to_string())
                }
                Err(e) => Err(format!("Failed to load tool registry: {}", e)),
            }
        }
        Err(e) => Err(format!("Failed to initialize database: {}", e)),
    }
}

/// Check if the database exists and has data
pub async fn check_database_exists_command() -> Result<bool, String> {
    database::check_database_exists()
}

/// Clear all data from the database
pub async fn clear_database_command() -> Result<String, String> {
    let mut db_manager = database::DatabaseManager::new()?;
    match db_manager.clear_database() {
        Ok(_) => Ok("Database cleared successfully".to_string()),
        Err(e) => Err(format!("Failed to clear database: {}", e)),
    }
}

/// Restart a tool by its ID
pub async fn restart_tool_command(
    mcp_state: &MCPState,
    tool_id: String,
) -> Result<ToolUpdateResponse, String> {
    info!("Received request to restart tool: {}", tool_id);

    // Convert string to ToolId
    let tool_id_obj = ToolId::new(tool_id.clone());

    // Check if the tool exists
    let tool_exists = {
        let registry = mcp_state.tool_registry.read().await;
        registry.tools.contains_key(&tool_id_obj)
    };

    if !tool_exists {
        error!("Tool with ID '{}' not found for restart", tool_id);
        return Ok(ToolUpdateResponse {
            success: false,
            message: format!("Tool with ID '{}' not found", tool_id),
        });
    }

    info!("Tool '{}' found, attempting to restart", tool_id);

    // Get a write lock on the registry to restart the tool
    let restart_result = {
        let mut registry = mcp_state.tool_registry.write().await;
        registry.restart_tool(&tool_id_obj).await
    };

    match restart_result {
        Ok(_) => {
            info!("Successfully restarted tool: {}", tool_id);
            Ok(ToolUpdateResponse {
                success: true,
                message: format!("Tool '{}' restarted successfully", tool_id),
            })
        }
        Err(e) => {
            error!("Failed to restart tool {}: {}", tool_id, e);
            Ok(ToolUpdateResponse {
                success: false,
                message: format!("Failed to restart tool: {}", e),
            })
        }
    }
}
