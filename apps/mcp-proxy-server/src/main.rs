use anyhow::Result;
use clap::Parser;
use rmcp::{ServiceExt, transport::stdio};
use server::mcp_proxy_client::get_mcp_client;
use tracing_subscriber::{self, EnvFilter};

pub mod server;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Use SSE transport with specified port
    #[arg(long)]
    see_target_address: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    /*
        In most cases we can use this tracing subscriber using the stderr as writer
        BUT
        there are some apps like IntelliJ products that flag an mcp server as errored
        when found any error in the logs.
    */
    // tracing_subscriber::fmt()
    //     .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
    //     .with_target(false)
    //     .with_thread_ids(true)
    //     .with_file(true)
    //     .with_line_number(true)
    //     .with_writer(std::io::stderr)
    //     .with_ansi(false)
    //     .init();

    tracing::info!("Starting MCP Dockmaster Proxy Server");

    /*
        This is important because some clients like IntelliJ products
        sometimes pass "" when there are no arguments, producing some issues.
        So if arguments are not parsed, we use default values.
    */
    let args = Args::try_parse().unwrap_or_else(|e| {
        tracing::warn!("Failed to parse arguments: {}. Using default values.", e);
        Args {
            see_target_address: None,
        }
    });

    let sse_address = args
        .see_target_address
        .unwrap_or("http://127.0.0.1:11011/sse".to_string());

    let mcp_proxy_client = loop {
        match get_mcp_client(&sse_address).await {
            Ok(client) => break client,
            Err(e) => {
                tracing::error!("Error getting MCP client: {:?}. Retrying...", e);
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
        }
    };

    tracing::info!("Creating stdio transport...");
    let transport = stdio();

    tracing::info!("Creating MCP server...");
    let mcp_proxy_server = server::mcp_proxy_server::McpProxyServer::new(mcp_proxy_client)
        .serve(transport)
        .await?;

    tracing::info!("Waiting for MCP server to exit...");
    mcp_proxy_server.waiting().await?;
    Ok(())
}
