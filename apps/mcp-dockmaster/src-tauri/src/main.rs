// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tokio::main]
async fn main() {
    // Log a message to confirm logger is working
    log::info!("starting app version: {}", env!("CARGO_PKG_VERSION"));

    mcp_dockmaster_lib::run().await;
}
