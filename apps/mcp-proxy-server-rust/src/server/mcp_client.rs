use anyhow::Result;
use mcp_client::client::{ClientCapabilities, ClientInfo, McpClient, McpClientTrait};
use mcp_client::transport::sse::SseTransportHandle;
use mcp_client::transport::{SseTransport, Transport};
use mcp_client::{McpService, TransportHandle};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tower::timeout::Timeout;
use tracing::{debug, info};

type McpClientInstance = McpClient<Timeout<McpService<SseTransportHandle>>>;

pub struct McpClientProxy {
    server_url: String,
    client: Option<Arc<McpClientInstance>>,
}

impl Clone for McpClientProxy {
    fn clone(&self) -> Self {
        Self {
            server_url: self.server_url.clone(),
            client: self.client.clone(),
        }
    }
}

impl McpClientProxy {
    pub fn new(server_url: &str) -> Self {
        McpClientProxy {
            server_url: server_url.to_string(),
            client: None,
        }
    }

    pub async fn init(&mut self) -> Result<()> {
        // Create the base transport
        let transport = SseTransport::new(&self.server_url, HashMap::new());

        // Start transport
        let handle = transport.start().await?;

        // Create the service with timeout middleware
        let service = McpService::with_timeout(handle, Duration::from_secs(300));

        // Create client
        let mut client = McpClient::new(service);
        debug!("Client created\n");

        // Initialize
        let server_info = client
            .initialize(
                ClientInfo {
                    name: "mcp-proxy-server-rust".into(),
                    version: env!("CARGO_PKG_VERSION").into(),
                },
                ClientCapabilities::default(),
            )
            .await?;
        info!("Connected to server: {server_info:?}\n");

        // Sleep for 100ms to allow the server to start - surprisingly this is required!
        tokio::time::sleep(Duration::from_millis(500)).await;

        // List tools
        let tools = client.list_tools(None).await?;
        debug!("Available tools: {tools:?}\n");

        // List resources
        let resources = client.list_resources(None).await?;
        debug!("Available resources: {resources:?}\n");

        // List prompts
        let prompts = client.list_prompts(None).await?;
        debug!("Available prompts: {prompts:?}\n");

        self.client = Some(Arc::new(client));
        Ok(())
    }

    pub fn get_client(&self) -> Option<Arc<McpClientInstance>> {
        self.client.clone()
    }
}
