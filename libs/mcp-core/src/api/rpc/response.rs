use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::api::rpc::errors::JsonRpcError;

#[derive(Serialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Value,
    pub result: Option<Value>,
    pub error: Option<JsonRpcError>,
}
