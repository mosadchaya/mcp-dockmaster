use crate::MCPError;
use log::info;
use serde_json::{json, Value};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt},
    time::Duration,
};

pub async fn init_rpc_call(
    server_id: &str,
    stdin: &mut tokio::process::ChildStdin,
    stdout: &mut tokio::process::ChildStdout,
) -> Result<(), MCPError> {
    info!("Initializing connection to server {}", server_id);

    let execute_cmd = json!({
        "jsonrpc": "2.0",
        "method": "notifications/initialized"
    });

    let cmd_str = serde_json::to_string(&execute_cmd)
        .map_err(|e| MCPError::SerializationError(e.to_string()))?
        + "\n";

    info!("Command: {}", cmd_str.trim());

    stdin
        .write_all(cmd_str.as_bytes())
        .await
        .map_err(|e| MCPError::StdinWriteError(e.to_string()))?;
    stdin
        .flush()
        .await
        .map_err(|e| MCPError::StdinFlushError(e.to_string()))?;

    let mut reader = tokio::io::BufReader::new(&mut *stdout);
    let mut response_line = String::new();

    let read_result =
        tokio::time::timeout(Duration::from_secs(1), reader.read_line(&mut response_line)).await;

    match read_result {
        Ok(Ok(0)) => return Err(MCPError::ServerClosedConnection),
        Ok(Ok(_)) => {}
        Ok(Err(e)) => return Err(MCPError::StdoutReadError(e.to_string())),
        Err(_) => return Err(MCPError::TimeoutError(server_id.to_string())),
    }

    Ok(())
}

/// Send a JSON-RPC command to a process and read the response
pub async fn rpc_call(
    server_id: &str,
    method: &str,
    params: Value,
    stdin: &mut tokio::process::ChildStdin,
    stdout: &mut tokio::process::ChildStdout,
    timeout_secs: u64,
) -> Result<Value, MCPError> {
    let command = json!({
        "jsonrpc": "2.0",
        "id": format!("{}_{}", server_id, method),
        "method": method,
        "params": params
    });

    let cmd_str = serde_json::to_string(&command)
        .map_err(|e| MCPError::SerializationError(e.to_string()))?
        + "\n";

    info!("Sending command to {}: {}", server_id, cmd_str.trim());

    // Write command to stdin with better error handling
    match stdin.write_all(cmd_str.as_bytes()).await {
        Ok(_) => {}
        Err(e) => {
            // If the pipe is broken, the process might have died
            if e.kind() == std::io::ErrorKind::BrokenPipe {
                return Err(MCPError::StdinWriteError(format!(
                    "Process has died (broken pipe): {}",
                    e
                )));
            }
            return Err(MCPError::StdinWriteError(e.to_string()));
        }
    }

    // Flush stdin with better error handling
    match stdin.flush().await {
        Ok(_) => {}
        Err(e) => {
            // If the pipe is broken, the process might have died
            if e.kind() == std::io::ErrorKind::BrokenPipe {
                return Err(MCPError::StdinFlushError(format!(
                    "Process has died (broken pipe during flush): {}",
                    e
                )));
            }
            return Err(MCPError::StdinFlushError(e.to_string()));
        }
    };

    let mut reader = tokio::io::BufReader::new(&mut *stdout);
    let mut response_line = String::new();

    let read_result = tokio::time::timeout(
        Duration::from_secs(timeout_secs),
        reader.read_line(&mut response_line),
    )
    .await;

    match read_result {
        Ok(Ok(0)) => return Err(MCPError::ServerClosedConnection),
        Ok(Ok(_)) => {
            info!(
                "Received response from {}: {}",
                server_id,
                response_line.trim()
            );
        }
        Ok(Err(e)) => return Err(MCPError::StdoutReadError(e.to_string())),
        Err(_) => return Err(MCPError::TimeoutError(server_id.to_string())),
    }

    if response_line.is_empty() {
        return Err(MCPError::NoResponse);
    }

    let response: Value = serde_json::from_str(&response_line)
        .map_err(|e| MCPError::JsonParseError(e.to_string()))?;

    if let Some(error) = response.get("error") {
        return Err(MCPError::ToolExecutionError(error.to_string()));
    }

    response
        .get("result")
        .cloned()
        .ok_or(MCPError::NoResultField)
}
