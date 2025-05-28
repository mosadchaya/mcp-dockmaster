use anyhow::Result;
use rmcp::{
    RoleClient, ServiceExt,
    model::{ClientCapabilities, ClientInfo, Implementation, InitializeRequestParam},
    service::RunningService,
    transport::SseClientTransport,
};

pub async fn get_mcp_client(
    server_url: &str,
) -> Result<RunningService<RoleClient, InitializeRequestParam>> {
    tracing::info!(
        "Starting MCP client initialization for server URL: {}",
        server_url
    );

    let transport = SseClientTransport::start(server_url)
        .await
        .inspect_err(|e| {
            tracing::error!("Error starting transport: {:?}", e);
        })?;
    tracing::info!("Transport layer started successfully.");

    let client_info = ClientInfo {
        protocol_version: Default::default(),
        capabilities: ClientCapabilities::default(),
        client_info: Implementation {
            name: "dockmaster-mcp-proxy-server-client".into(),
            version: env!("CARGO_PKG_VERSION").into(),
        },
    };
    tracing::info!("ClientInfo constructed: {:?}", client_info);

    let client = client_info.serve(transport).await.inspect_err(|e| {
        tracing::error!("Client error during serve: {:?}", e);
    })?;
    tracing::info!("Client successfully served.");

    tracing::debug!("MCP client initialized with server URL: {}", server_url);
    tracing::debug!("MCP client peer info: {:?}", client.peer_info());
    tracing::debug!(
        "MCP client tools: {:?}",
        client.list_tools(None).await.inspect_err(|e| {
            tracing::error!("Error listing tools: {:?}", e);
        })?
    );
    tracing::debug!(
        "MCP client resources: {:?}",
        client.list_resources(None).await.inspect_err(|e| {
            tracing::error!("Error listing resources: {:?}", e);
        })?
    );
    tracing::debug!("MCP client prompts: {:?}", client.list_prompts(None).await?);

    tracing::info!("MCP client initialization completed successfully.");
    Ok(client)
}
