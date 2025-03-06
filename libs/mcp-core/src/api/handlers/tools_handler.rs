use axum::{Extension, Json};
use log::info;
use serde_json::json;
use std::sync::Arc;

use crate::api::rpc::{JsonRpcRequest, JsonRpcResponse, JsonRpcError};
use crate::application::AppContext;
use crate::application::dto::{ToolExecutionDTO};

pub async fn handle_mcp_request(
    Extension(app_context): Extension<Arc<AppContext>>,
    Json(request): Json<JsonRpcRequest>,
) -> Json<JsonRpcResponse> {
    info!("Received MCP request: method={}", request.method);

    let tool_service = &app_context.tool_service;

    let result = match request.method.as_str() {
        "tools/list" => {
            match tool_service.list_tools().await {
                Ok(tools) => Ok(json!({ "tools": tools })),
                Err(e) => Err(json!({"code": -32603, "message": e.to_string()}))
            }
        }
        "tools/call" => {
            if let Some(params) = request.params {
                // Extract tool name and arguments from params
                let tool_name = match params.get("name").and_then(|v| v.as_str()) {
                    Some(name) => name,
                    None => {
                        return Json(JsonRpcResponse {
                            jsonrpc: "2.0".to_string(),
                            id: request.id,
                            result: None,
                            error: Some(JsonRpcError {
                                code: -32602,
                                message: "Missing name in parameters".to_string(),
                                data: None,
                            }),
                        });
                    }
                };

                let arguments = match params.get("arguments") {
                    Some(args) => args.clone(),
                    None => json!({}),
                };

                // Call the application service to execute the tool
                match tool_service.execute_tool(ToolExecutionDTO {
                    tool_id: tool_name.to_string(),
                    parameters: arguments,
                }).await {
                    Ok(response) => {
                        if response.success {
                            Ok(json!({
                                "content": [
                                    {
                                        "type": "text",
                                        "text": response.result.unwrap_or(json!("No result")).to_string()
                                    }
                                ]
                            }))
                        } else {
                            Err(json!({
                                "code": -32000,
                                "message": response.error.unwrap_or_else(|| "Unknown error".to_string())
                            }))
                        }
                    },
                    Err(e) => Err(json!({
                        "code": -32000,
                        "message": format!("Tool execution error: {}", e)
                    })),
                }
            } else {
                Err(json!({
                    "code": -32602,
                    "message": "Invalid params - missing parameters for tool execution"
                }))
            }
        }
        "prompts/list" => Ok(json!({ "prompts": [] })),
        "resources/list" => Ok(json!({ "resources": [] })),
        "resources/read" => {
            Err(json!({
                "code": -32601,
                "message": "Resource reading not implemented yet"
            }))
        }
        "prompts/get" => {
            Err(json!({
                "code": -32601,
                "message": "Prompt retrieval not implemented yet"
            }))
        }
        _ => Err(json!({
            "code": -32601,
            "message": format!("Method not found: {}", request.method)
        }))
    };

    match result {
        Ok(result) => Json(JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: request.id,
            result: Some(result),
            error: None,
        }),
        Err(error) => {
            let code = error.get("code").and_then(|c| c.as_i64()).unwrap_or(-32603);
            let message = error.get("message").and_then(|m| m.as_str()).unwrap_or("Unknown error").to_string();
            let data = error.get("data").cloned();

            Json(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(JsonRpcError {
                    code: code as i32,
                    message,
                    data,
                }),
            })
        }
    }
}
