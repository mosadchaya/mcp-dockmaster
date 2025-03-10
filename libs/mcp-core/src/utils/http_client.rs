use log::{error, info};
use reqwest::Client;
use serde_json::{json, Value};

/// Send a notification to the mcp-proxy-server that the list of tools has changed
pub async fn notify_tools_changed(server_id: &str, added_tools: Vec<String>, removed_tools: Vec<String>, updated_tools: Vec<String>) {
    info!("Sending tools/listChanged notification for server: {}", server_id);
    
    // Create the JSON-RPC 2.0 request payload
    let payload = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/listChanged",
        "params": {
            "server_id": server_id,
            "added_tools": added_tools,
            "removed_tools": removed_tools,
            "updated_tools": updated_tools
        }
    });

    // Send the notification to the proxy server
    match send_notification(payload).await {
        Ok(_) => info!("Successfully sent tools/listChanged notification"),
        Err(e) => error!("Failed to send tools/listChanged notification: {}", e),
    }
}

/// Send a JSON-RPC 2.0 notification to the mcp-proxy-server
async fn send_notification(payload: Value) -> Result<(), String> {
    // Create HTTP client
    let client = Client::builder()
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;
    
    // Send the notification to the proxy server
    let response = client
        .post("http://localhost:3000/mcp-proxy")
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Failed to send notification: {}", e))?;
    
    // Check if the request was successful
    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("Notification failed with status {}: {}", status, body));
    }
    
    Ok(())
}
