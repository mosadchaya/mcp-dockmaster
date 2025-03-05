use std::time::Duration;
use tokio::process::{Child, Command};
use tokio::time::sleep;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use serde_json::{json, Value};
use crate::mcp_proxy::MCPState;
use crate::error::{MCPError, MCPResult};

/// A simple MCP server mockup for testing
struct MCPServerMockup {
    child: Child,
    tool_id: String,
}

impl MCPServerMockup {
    /// Create a new MCP server mockup
    async fn new(tool_id: &str) -> MCPResult<Self> {
        // Create a simple Rust script that responds to JSON-RPC requests
        let script = r#"
extern crate serde;
extern crate serde_json;

use std::io::{self, BufRead, Write};
use serde_json::{json, Value};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut lines = stdin.lock().lines();

    while let Some(Ok(line)) = lines.next() {
        let request: Value = match serde_json::from_str(&line) {
            Ok(req) => req,
            Err(_) => continue,
        };

        let id = request.get("id").cloned().unwrap_or(json!(null));
        let method = request.get("method").and_then(|m| m.as_str()).unwrap_or("");

        let response = match method {
            "tools/list" => json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": [
                    {
                        "id": "hello_world",
                        "name": "Hello World",
                        "description": "A simple hello world tool",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "name": {
                                    "type": "string",
                                    "description": "Name to greet"
                                }
                            }
                        }
                    }
                ]
            }),
            "tools/call" => {
                let name = request
                    .get("params")
                    .and_then(|p| p.get("arguments"))
                    .and_then(|a| a.get("name"))
                    .and_then(|n| n.as_str())
                    .unwrap_or("World");

                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "message": format!("Hello, {}!", name)
                    }
                })
            },
            _ => json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": {
                    "code": -32601,
                    "message": format!("Method not found: {}", method)
                }
            }),
        };

        let response_str = serde_json::to_string(&response).unwrap() + "\n";
        stdout.write_all(response_str.as_bytes()).unwrap();
        stdout.flush().unwrap();
    }
}
"#;

        // Create a temporary file for the script
        let temp_dir = std::env::temp_dir();
        let script_path = temp_dir.join("mcp_mockup.rs");
        std::fs::write(&script_path, script)?;

        // Create a Cargo.toml file for the script
        let cargo_toml = r#"
[package]
name = "mcp_mockup"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0"
serde_json = "1.0"
"#;
        
        // Create a temporary directory for the Cargo project
        let project_dir = temp_dir.join("mcp_mockup_project");
        std::fs::create_dir_all(&project_dir)?;
        std::fs::create_dir_all(project_dir.join("src"))?;
        
        // Write the Cargo.toml file
        std::fs::write(project_dir.join("Cargo.toml"), cargo_toml)?;
        
        // Write the script to main.rs
        std::fs::write(project_dir.join("src/main.rs"), script)?;
        
        // Compile the script with cargo
        let status = Command::new("cargo")
            .arg("build")
            .arg("--release")
            .current_dir(&project_dir)
            .status()
            .await?;

        if !status.success() {
            return Err(MCPError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to compile mockup server script"
            )));
        }

        // Run the compiled binary
        let mut child = Command::new(project_dir.join("target/release/mcp_mockup"))
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;

        // Return the mockup server
        Ok(Self {
            child,
            tool_id: tool_id.to_string(),
        })
    }

    /// Clean up resources
    async fn cleanup(&mut self) -> MCPResult<()> {
        if let Err(e) = self.child.kill().await {
            eprintln!("Failed to kill mockup server: {}", e);
        }
        Ok(())
    }
}

#[tokio::test]
async fn test_mcp_core_with_mockup_server() -> MCPResult<()> {
    // Initialize the MCP state
    let mcp_state = MCPState::default();
    
    // Create a mockup server
    let mut mockup = MCPServerMockup::new("mockup_server").await?;
    
    // Register the mockup server with the MCP core
    {
        let mut registry = mcp_state.tool_registry.write().await;
        let tool_id = mockup.tool_id.clone();
        
        // Add the mockup server to the registry
        registry.tools.insert(
            tool_id.clone(),
            json!({
                "name": "Mockup Server",
                "description": "A simple MCP server mockup for testing",
                "enabled": true,
                "tool_type": "node", // We're pretending it's a Node.js server
                "configuration": {
                    "command": mockup.child.id().map(|id| id.to_string()).unwrap_or_else(|| "unknown".to_string()),
                }
            })
        );
        
        // Add the mockup server's stdin/stdout to the registry
        if let (Some(stdin), Some(stdout)) = (mockup.child.stdin.take(), mockup.child.stdout.take()) {
            registry.process_ios.insert(tool_id.clone(), (stdin, stdout));
            
            // Create a dummy child process for cleanup since we're moving the real one
            let dummy_child = Command::new("echo")
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .spawn()
                .unwrap_or_else(|_| panic!("Failed to create dummy process"));
                
            // Store the real child in the registry
            registry.processes.insert(tool_id, Some(mockup.child));
            
            // Replace mockup's child with the dummy
            mockup.child = dummy_child;
        } else {
            return Err(MCPError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to get mockup server stdin/stdout"
            )));
        }
    }
    
    // Wait for the server to start
    sleep(Duration::from_millis(100)).await;
    
    // Wait a bit longer for the server to start
    sleep(Duration::from_secs(2)).await;
    
    // Discover tools from the mockup server
    {
        let mut registry = mcp_state.tool_registry.write().await;
        
        // Directly send a tools/list command to the mockup server
        // This is a simplified version of what execute_tool does
        if let Some((stdin, stdout)) = registry.process_ios.get_mut(&mockup.tool_id) {
            // Send the tools/list command
            let discover_cmd = json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "tools/list",
                "params": {}
            });
            
            let cmd_str = serde_json::to_string(&discover_cmd).unwrap() + "\n";
            println!("Sending command: {}", cmd_str.trim());
            
            // Write command to stdin
            stdin.write_all(cmd_str.as_bytes()).await.unwrap();
            stdin.flush().await.unwrap();
            
            // Read the response
            let mut reader = tokio::io::BufReader::new(&mut *stdout);
            let mut response_line = String::new();
            
            let read_result = tokio::time::timeout(
                Duration::from_secs(5),
                reader.read_line(&mut response_line)
            ).await;
            
            match read_result {
                Ok(Ok(0)) => println!("Server process closed connection"),
                Ok(Ok(_)) => println!("Received response: {}", response_line.trim()),
                Ok(Err(e)) => println!("Failed to read from process stdout: {}", e),
                Err(_) => println!("Timeout waiting for response"),
            }
            
            if !response_line.is_empty() {
                // Parse the response
                let response: Value = serde_json::from_str(&response_line).unwrap();
                
                // Extract the tools from the response
                if let Some(result) = response.get("result") {
                    if let Some(tools_array) = result.as_array() {
                        // Store the discovered tools
                        registry.server_tools.insert(mockup.tool_id.clone(), tools_array.clone());
                        
                        // Verify that we discovered the hello_world tool
                        assert!(!tools_array.is_empty(), "No tools discovered from mockup server");
                        
                        let hello_tool = tools_array.iter().find(|t| {
                            t.get("id").and_then(|id| id.as_str()) == Some("hello_world")
                        });
                        
                        assert!(hello_tool.is_some(), "Hello World tool not found in discovered tools");
                    } else {
                        println!("Result is not an array: {:?}", result);
                        assert!(false, "Result is not an array");
                    }
                } else {
                    println!("No result field in response: {:?}", response);
                    assert!(false, "No result field in response");
                }
            } else {
                assert!(false, "No response from mockup server");
            }
        } else {
            assert!(false, "Mockup server not found in process_ios");
        }
    }
    
    // Execute the hello_world tool
    {
        let mut registry = mcp_state.tool_registry.write().await;
        let result = match registry.execute_tool(
            &mockup.tool_id,
            "hello_world",
            json!({"name": "MCP Tester"})
        ).await {
            Ok(result) => result,
            Err(e) => return Err(MCPError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to execute hello_world tool: {}", e)
            ))),
        };
        
        // Verify the result
        let message = result.get("message").and_then(|m| m.as_str());
        assert_eq!(message, Some("Hello, MCP Tester!"), "Unexpected response from hello_world tool");
    }
    
    // Clean up
    mockup.cleanup().await?;
    
    Ok(())
}
