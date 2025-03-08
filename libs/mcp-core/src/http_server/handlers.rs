use axum::response::IntoResponse;
use axum::{http::StatusCode, Extension, Json};
use log::info;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::core::mcp_core::MCPCore;
use crate::core::mcp_core_proxy_ext::McpCoreProxyExt;
use crate::models::types::{
    Distribution, InputSchema, ServerRegistrationRequest, ServerToolInfo, ToolConfiguration,
    ToolEnvironment, ToolExecutionRequest,
};

#[derive(Deserialize)]
pub struct JsonRpcRequest {
    #[allow(dead_code)]
    jsonrpc: String,
    id: Value,
    method: String,
    params: Option<Value>,
}

#[derive(Serialize)]
pub struct JsonRpcResponse {
    jsonrpc: String,
    id: Value,
    result: Option<Value>,
    error: Option<JsonRpcError>,
}

#[derive(Serialize)]
pub struct JsonRpcError {
    code: i32,
    message: String,
    data: Option<Value>,
}

pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "MCP Server is running!")
}

pub async fn handle_mcp_request(
    Extension(mcp_core): Extension<MCPCore>,
    Json(request): Json<JsonRpcRequest>,
) -> Json<JsonRpcResponse> {
    info!("Received MCP request: method={}", request.method);

    let result = match request.method.as_str() {
        "tools/list" => handle_list_tools(mcp_core).await,
        "tools/call" => {
            if let Some(params) = request.params {
                handle_invoke_tool(mcp_core, params).await
            } else {
                Err(json!({
                    "code": -32602,
                    "message": "Missing parameters"
                }))
            }
        }
        "prompts/list" => handle_list_prompts().await,
        "resources/list" => handle_list_resources().await,
        "resources/read" => {
            if let Some(params) = request.params {
                handle_read_resource(params).await
            } else {
                Err(json!({
                    "code": -32602,
                    "message": "Invalid params - missing parameters for resource reading"
                }))
            }
        }
        "prompts/get" => {
            if let Some(params) = request.params {
                handle_get_prompt(params).await
            } else {
                Err(json!({
                    "code": -32602,
                    "message": "Invalid params - missing parameters for prompt retrieval"
                }))
            }
        }
        "registry/install" => {
            if let Some(params) = request.params {
                handle_register_tool(mcp_core, params).await
            } else {
                Err(json!({
                    "code": -32602,
                    "message": "Invalid params - missing parameters for tool registration"
                }))
            }
        }
        "registry/list" => handle_list_all_tools(mcp_core).await,
        _ => Err(json!({
            "code": -32601,
            "message": format!("Method '{}' not found", request.method)
        })),
    };

    match result {
        Ok(result) => Json(JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: request.id,
            result: Some(result),
            error: None,
        }),
        Err(error) => {
            let error_obj = error.as_object().unwrap();
            Json(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(JsonRpcError {
                    code: error_obj
                        .get("code")
                        .and_then(|v| v.as_i64())
                        .unwrap_or(-32000) as i32,
                    message: error_obj
                        .get("message")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Unknown error")
                        .to_string(),
                    data: None,
                }),
            })
        }
    }
}

async fn handle_list_tools(mcp_core: MCPCore) -> Result<Value, Value> {
    let result = mcp_core.list_all_server_tools().await;

    match result {
        Ok(tools) => {
            let tools_with_defaults: Vec<ServerToolInfo> = tools
                .into_iter()
                .map(|tool| {
                    let mut tool = tool;
                    // Ensure input_schema has a default if not present
                    if tool.input_schema.is_none() {
                        tool.input_schema = Some(InputSchema {
                            properties: Default::default(),
                            required: Vec::new(),
                            r#type: "object".to_string(),
                        });
                    }
                    tool
                })
                .collect();

            Ok(json!({
                "tools": tools_with_defaults
            }))
        }
        Err(e) => Err(json!({
            "code": -32000,
            "message": format!("Failed to list tools: {}", e)
        })),
    }
}

async fn handle_register_tool(mcp_core: MCPCore, params: Value) -> Result<Value, Value> {
    match params.get("name") {
        Some(name) => {
            let tool_name = name.as_str().unwrap_or("").to_string();
            let description = params
                .get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let tool_type = params
                .get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("node")
                .to_string();

            let configuration = params.get("configuration").map(|config| {
                let command = config
                    .get("command")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let args = config.get("args").map(|args| {
                    args.as_array()
                        .unwrap_or(&Vec::new())
                        .iter()
                        .filter_map(|v| v.as_str())
                        .map(|s| s.to_string())
                        .collect()
                });
                let env = config.get("env").map(|env| {
                    env.as_object()
                        .unwrap_or(&serde_json::Map::new())
                        .iter()
                        .map(|(k, v)| {
                            (
                                k.clone(),
                                ToolEnvironment {
                                    description: "".to_string(),
                                    default: v.as_str().map(|s| s.to_string()),
                                    required: false,
                                },
                            )
                        })
                        .collect()
                });

                ToolConfiguration { command, args, env }
            });

            let distribution = params.get("distribution").map(|dist| Distribution {
                r#type: dist
                    .get("type")
                    .and_then(|v| v.as_str())
                    .unwrap_or("error")
                    .to_string(),
                package: dist
                    .get("package")
                    .and_then(|v| v.as_str())
                    .unwrap_or("error")
                    .to_string(),
            });

            let tool = ServerRegistrationRequest {
                tool_name,
                description,
                tool_type,
                configuration,
                distribution,
            };

            println!("[POST] handle_register_tool: tool {:?}", tool);
            let r = mcp_core.register_tool(tool).await;
            println!("[INSTALLATION] handle_register_tool: r {:?}", r);
            Ok(json!({
                "code": 0,
                "message": "Tool installed successfully"
            }))
        }
        None => Err(json!({
            "code": -32602,
            "message": format!("Tool not found: {:?}", params.get("name"))
        })),
    }
}

async fn fetch_tool_from_registry() -> Result<Value, Value> {
    // Fetch tools from remote URL
    let tools_url = "https://pub-790f7c5dc69a482998b623212fa27446.r2.dev/db.v0.json";

    let client = reqwest::Client::builder().build().unwrap_or_default();

    let response = client
        .get(tools_url)
        .header("Accept-Encoding", "gzip")
        .header("User-Agent", "MCP-Core/1.0")
        .send()
        .await
        .map_err(|e| {
            json!({
                "code": -32000,
                "message": format!("Failed to fetch tools from registry: {}", e)
            })
        })?;

    let tools: Vec<Value> = response.json().await.map_err(|e| {
        json!({
            "code": -32000,
            "message": format!("Failed to parse tools from registry: {}", e)
        })
    })?;

    let mut all_tools = Vec::new();

    for tool in tools {
        let mut tool_info = serde_json::Map::new();

        if let Some(obj) = tool.as_object() {
            for (key, value) in obj {
                tool_info.insert(key.clone(), value.clone());
            }
        }

        if !tool_info.contains_key("description") {
            tool_info.insert("description".to_string(), json!("Tool from registry"));
        }

        if !tool_info.contains_key("inputSchema") {
            tool_info.insert(
                "inputSchema".to_string(),
                json!({
                    "type": "object",
                    "properties": {}
                }),
            );
        }

        all_tools.push(json!(tool_info));
    }
    let v = json!({
        "tools": all_tools
    });
    Ok(v)
}

async fn handle_list_all_tools(mcp_core: MCPCore) -> Result<Value, Value> {
    let mcp_state = mcp_core.mcp_state.read().await;
    let registry = mcp_state.tool_registry.read().await;
    let installed_tools = registry.get_all_servers()?;
    let mut registry_tools = fetch_tool_from_registry().await?;

    for tool in registry_tools
        .get_mut("tools")
        .unwrap()
        .as_array_mut()
        .unwrap()
    {
        let tool_name = tool.get("name").unwrap().as_str().unwrap();
        if installed_tools.contains_key(tool_name) {
            println!("Tool {} is installed", tool_name);
            tool.as_object_mut()
                .unwrap()
                .insert("installed".to_string(), json!(true));
        } else {
            println!("Tool {} is not installed", tool_name);
            tool.as_object_mut()
                .unwrap()
                .insert("installed".to_string(), json!(false));
        }
    }

    Ok(registry_tools)
}

async fn handle_list_prompts() -> Result<Value, Value> {
    Ok(json!({
        "prompts": []
    }))
}

async fn handle_list_resources() -> Result<Value, Value> {
    Ok(json!({
        "resources": []
    }))
}

async fn handle_read_resource(_params: Value) -> Result<Value, Value> {
    Err(json!({
        "code": -32601,
        "message": "Resource reading not implemented yet"
    }))
}

async fn handle_get_prompt(_params: Value) -> Result<Value, Value> {
    Err(json!({
        "code": -32601,
        "message": "Prompt retrieval not implemented yet"
    }))
}

async fn handle_invoke_tool(mcp_core: MCPCore, params: Value) -> Result<Value, Value> {
    let tool_name = match params.get("name").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => {
            return Err(json!({
                "code": -32602,
                "message": "Missing name in parameters"
            }))
        }
    };

    let arguments = match params.get("arguments") {
        Some(args) => args.clone(),
        None => json!({}),
    };

    let mcp_state = mcp_core.mcp_state.read().await;
    let server_tools = mcp_state.server_tools.read().await;

    // Find which server has the requested tool
    let mut server_id = None;

    for (sid, tools) in &*server_tools {
        for tool in tools {
            if tool.id == tool_name {
                server_id = Some(sid.clone());
                break;
            }

            // Also check by name if id doesn't match
            if tool.name == tool_name {
                server_id = Some(sid.clone());
                break;
            }
        }

        if server_id.is_some() {
            break;
        }
    }

    // Drop the locks before proceeding
    drop(server_tools);
    drop(mcp_state);

    match server_id {
        Some(server_id) => {
            let request = ToolExecutionRequest {
                tool_id: format!("{}:{}", server_id, tool_name),
                parameters: arguments,
            };

            match mcp_core.execute_proxy_tool(request).await {
                Ok(response) => {
                    if response.success {
                        Ok(response.result.unwrap_or(json!(null)))
                    } else {
                        Err(json!({
                            "code": -32000,
                            "message": response.error.unwrap_or_else(|| "Unknown error".to_string())
                        }))
                    }
                }
                Err(e) => Err(json!({
                    "code": -32000,
                    "message": format!("Failed to execute tool: {}", e)
                })),
            }
        }
        None => Err(json!({
            "code": -32601,
            "message": format!("Tool '{}' not found", tool_name)
        })),
    }
}
