use std::sync::Arc;
use async_trait::async_trait;
use serde_json::{Value};
use std::future::Future;
use std::pin::Pin;
use log::{info, error, warn};
use std::collections::HashMap;
use tokio::sync::{RwLock, mpsc};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

// Import SDK modules
use mcp_sdk_core::{Tool, Content, Resource};
use mcp_sdk_core::protocol;
use mcp_sdk_core::handler::{ToolError, ResourceError, PromptError};
use mcp_sdk_core::prompt;
use mcp_sdk_server::{Router, Server, ByteTransport};
use mcp_sdk_server::router::{RouterService, CapabilitiesBuilder};

/// Trait for client managers to implement
#[async_trait]
pub trait ClientManagerTrait: Send + Sync {
    /// Handle a tool call
    async fn handle_tool_call(&self, tool_name: String, arguments: Value) -> Result<Value, String>;
    
    /// List available tools
    async fn list_tools(&self) -> Result<Vec<Tool>, String>;
    
    /// Synchronous version of list_tools for use in non-async contexts
    fn list_tools_sync(&self) -> Vec<Tool>;
    
    /// Update the tool cache
    async fn update_tools_cache(&self);
}

/// Default implementation of ClientManager
pub struct ClientManager {}

#[async_trait]
impl ClientManagerTrait for ClientManager {
    async fn handle_tool_call(&self, tool_name: String, _arguments: Value) -> Result<Value, String> {
        // Default implementation just returns an error
        Err(format!("Tool '{}' not implemented", tool_name))
    }
    
    async fn list_tools(&self) -> Result<Vec<Tool>, String> {
        // Default implementation returns an empty list
        Ok(vec![])
    }
    
    fn list_tools_sync(&self) -> Vec<Tool> {
        // Default implementation returns an empty list
        vec![]
    }
    
    async fn update_tools_cache(&self) {
        // Default implementation does nothing
    }
}

/// MCP Router implementation that uses a ClientManager
#[derive(Clone)]
pub struct MCPRouter {
    name: String,
    client_manager: Arc<dyn ClientManagerTrait>,
    capabilities: protocol::ServerCapabilities,
}

impl MCPRouter {
    fn new(name: String, client_manager: Arc<dyn ClientManagerTrait>) -> Self {
        // Create capabilities
        let mut builder = CapabilitiesBuilder::new();
        builder = builder.with_tools(true);
        
        Self {
            name,
            client_manager,
            capabilities: builder.build(),
        }
    }
}

impl Router for MCPRouter {
    fn name(&self) -> String {
        self.name.clone()
    }
    
    fn instructions(&self) -> String {
        "MCP Dockmaster Server".to_string()
    }
    
    fn capabilities(&self) -> protocol::ServerCapabilities {
        self.capabilities.clone()
    }
    
    fn list_tools(&self) -> Vec<Tool> {
        // Use the synchronous version of list_tools to avoid runtime panic
        self.client_manager.list_tools_sync()
    }
    
    fn call_tool(
        &self,
        tool_name: &str,
        arguments: Value,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Content>, ToolError>> + Send + 'static>> {
        let client_manager = self.client_manager.clone();
        let tool_name = tool_name.to_string();
        
        Box::pin(async move {
            match client_manager.handle_tool_call(tool_name, arguments).await {
                Ok(result) => {
                    // Convert the result to Content using the text method
                    let result_str = serde_json::to_string(&result).unwrap_or_default();
                    let content = Content::text(result_str);
                    Ok(vec![content])
                },
                Err(err) => Err(ToolError::ExecutionError(err)),
            }
        })
    }
    
    fn list_resources(&self) -> Vec<Resource> {
        vec![]
    }
    
    fn read_resource(
        &self,
        _uri: &str,
    ) -> Pin<Box<dyn Future<Output = Result<String, ResourceError>> + Send + 'static>> {
        Box::pin(async {
            Err(ResourceError::NotFound("Resource not found".to_string()))
        })
    }
    
    fn list_prompts(&self) -> Vec<prompt::Prompt> {
        vec![]
    }
    
    fn get_prompt(
        &self,
        _prompt_name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<String, PromptError>> + Send + 'static>> {
        Box::pin(async {
            Err(PromptError::NotFound("Prompt not found".to_string()))
        })
    }
}

/// Manages active SSE sessions
#[derive(Clone)]
pub struct SessionManager {
    pub sessions: Arc<RwLock<HashMap<String, mpsc::Sender<Vec<u8>>>>>,
}

impl SessionManager {
    /// Create a new session manager
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Register a new session
    pub async fn register_session(&self, session_id: String, tx: mpsc::Sender<Vec<u8>>) {
        let mut sessions = self.sessions.write().await;
        sessions.insert(session_id, tx);
    }
    
    /// Remove a session
    pub async fn remove_session(&self, session_id: &str) {
        let mut sessions = self.sessions.write().await;
        sessions.remove(session_id);
    }
    
    /// Send a message to a session
    pub async fn send_to_session(&self, session_id: &str, data: Vec<u8>) -> Result<(), String> {
        let sessions = self.sessions.read().await;
        if let Some(tx) = sessions.get(session_id) {
            tx.send(data).await.map_err(|e| format!("Failed to send to session: {}", e))
        } else {
            Err(format!("Session not found: {}", session_id))
        }
    }
}

/// Create a new SSE handler
pub fn create_sse_handler(client_manager: Arc<dyn ClientManagerTrait>) -> (SessionManager, RouterService<MCPRouter>) {
    // Create the session manager
    let session_manager = SessionManager::new();
    
    // Create our router implementation
    let router = MCPRouter::new("MCP Dockmaster Server".to_string(), client_manager);
    
    // Wrap it in a RouterService
    let router_service = RouterService(router);
    
    (session_manager, router_service)
}

/// Process an incoming message from a client
pub async fn process_message(
    _server: &Server<RouterService<MCPRouter>>,
    session_id: &str,
    session_manager: &SessionManager,
    message: Vec<u8>,
) -> Result<(), String> {
    // Create simplex streams for bidirectional communication
    let (c2s_read, mut c2s_write) = io::simplex(1024);
    let (mut s2c_read, s2c_write) = io::simplex(1024);
    
    // Write the incoming message to the client-to-server write stream
    c2s_write.write_all(&message).await
        .map_err(|e| format!("Failed to write message to stream: {}", e))?;
    c2s_write.flush().await
        .map_err(|e| format!("Failed to flush write stream: {}", e))?;
    
    // Spawn a task to handle the response from the server
    let session_manager_clone = session_manager.clone();
    let session_id_owned = session_id.to_string();
    
    let response_task = tokio::spawn(async move {
        let mut buffer = [0u8; 4096];
        
        loop {
            match s2c_read.read(&mut buffer).await {
                Ok(0) => break, // EOF
                Ok(n) => {
                    // Forward the response to the client through the session manager
                    if let Err(e) = session_manager_clone.send_to_session(
                        &session_id_owned, 
                        buffer[..n].to_vec()
                    ).await {
                        warn!("Failed to send response to session {}: {}", session_id_owned, e);
                        break;
                    }
                },
                Err(e) => {
                    error!("Error reading from s2c stream: {}", e);
                    break;
                }
            }
        }
    });
    
    // Create a ByteTransport for the request
    let _transport = ByteTransport::new(c2s_read, s2c_write);
    
    // In a real implementation, we would use the server to process the message
    // But for now, we'll just echo the message back
    info!("Message received: {} bytes", message.len());
    
    // Wait for the response task to complete
    match response_task.await {
        Ok(_) => {},
        Err(e) => {
            error!("Error in response handling task: {}", e);
        }
    }
    
    Ok(())
}

/// Start the MCP server with the provided client manager
pub async fn start_mcp_server(client_manager: Arc<dyn ClientManagerTrait>) -> Result<(), Box<dyn std::error::Error>> {
    // Update the tool cache before starting the server
    info!("Updating tool cache before starting server...");
    client_manager.update_tools_cache().await;
    
    // Create the SSE handler
    let (_session_manager, router_service) = create_sse_handler(client_manager);
    
    // Create the server
    let _server = Server::new(router_service);
    
    // The HTTP server and SSE endpoints will be managed by the HTTP server component
    // which already exists in the MCP Core implementation
    info!("MCP Server initialized with SSE transport");
    
    Ok(())
} 