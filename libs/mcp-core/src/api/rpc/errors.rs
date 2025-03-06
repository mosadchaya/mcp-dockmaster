use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    pub data: Option<Value>,
}
