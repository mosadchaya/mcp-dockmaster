use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use log::{info, error};
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router, Extension,
};
use axum::response::IntoResponse;
use crate::features::mcp_proxy::MCPState;

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

pub async fn start_http_server(state: Arc<RwLock<MCPState>>) {
    let app = Router::new()
        .route("/mcp-proxy", post(handle_mcp_request))
        .route("/health", get(health_check))
        .layer(Extension(state));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("MCP HTTP server starting on {}", addr);
    
    tokio::spawn(async move {
        match axum::serve(
            tokio::net::TcpListener::bind(&addr).await.unwrap(),
            app
        ).await
        {
            Ok(_) => info!("MCP HTTP server terminated normally"),
            Err(e) => error!("MCP HTTP server error: {}", e),
        }
    });
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "MCP Server is running!")
}

async fn handle_mcp_request(
    Extension(state): Extension<Arc<RwLock<MCPState>>>,
    Json(request): Json<JsonRpcRequest>,
) -> Json<JsonRpcResponse> {
    info!("Received MCP request: method={}", request.method);
    
    let result = match request.method.as_str() {
        "list/tools" => handle_list_tools(state).await,
        "list/prompts" => handle_list_prompts().await,
        "list/resources" => handle_list_resources().await,
        "invoke/tool" => {
            if let Some(params) = request.params {
                handle_invoke_tool(state, params).await
            } else {
                Err(json!({
                    "code": -32602,
                    "message": "Invalid params - missing parameters for tool invocation"
                }))
            }
        },
        _ => Err(json!({
            "code": -32601,
            "message": format!("Method not found: {}", request.method)
        })),
    };
    
    match result {
        Ok(result_value) => Json(JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: request.id,
            result: Some(result_value),
            error: None,
        }),
        Err(error_value) => {
            let code = error_value.get("code")
                .and_then(|v| v.as_i64())
                .unwrap_or(-32000) as i32;
                
            let message = error_value.get("message")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown error")
                .to_string();
                
            Json(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(JsonRpcError {
                    code,
                    message,
                    data: None,
                }),
            })
        }
    }
}

async fn handle_list_tools(state: Arc<RwLock<MCPState>>) -> Result<Value, Value> {
    let mcp_state = state.read().await;
    let registry = mcp_state.tool_registry.read().await;
    
    let mut all_tools = Vec::new();
    
    for (server_id, tools) in &registry.server_tools {
        for tool in tools {
            let mut tool_info = serde_json::Map::new();
            
            if let Some(obj) = tool.as_object() {
                for (key, value) in obj {
                    tool_info.insert(key.clone(), value.clone());
                }
            }
            
            tool_info.insert("server_id".to_string(), json!(server_id));
            
            let tool_id = tool.get("id").and_then(|v| v.as_str()).unwrap_or("main");
            let proxy_id = format!("{}:{}", server_id, tool_id);
            tool_info.insert("proxy_id".to_string(), json!(proxy_id));
            
            all_tools.push(json!(tool_info));
        }
    }
    
    Ok(json!(all_tools))
}

async fn handle_list_prompts() -> Result<Value, Value> {
    Ok(json!([]))
}

async fn handle_list_resources() -> Result<Value, Value> {
    Ok(json!([]))
}

async fn handle_invoke_tool(state: Arc<RwLock<MCPState>>, params: Value) -> Result<Value, Value> {
    let tool_id = match params.get("tool_id").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => return Err(json!({
            "code": -32602,
            "message": "Missing tool_id in parameters"
        })),
    };
    
    let parameters = match params.get("parameters") {
        Some(p) => p.clone(),
        None => json!({}),
    };
    
    let parts: Vec<&str> = tool_id.split(':').collect();
    if parts.len() != 2 {
        return Err(json!({
            "code": -32602,
            "message": "Invalid tool_id format. Expected 'server_id:tool_id'"
        }));
    }
    
    let server_id = parts[0];
    let actual_tool_id = parts[1];
    
    let mcp_state = state.read().await;
    let mut registry = mcp_state.tool_registry.write().await;
    
    match registry.execute_tool(server_id, actual_tool_id, parameters).await {
        Ok(result) => Ok(result),
        Err(e) => Err(json!({
            "code": -32000,
            "message": format!("Tool execution error: {}", e)
        })),
    }
}