use std::sync::Arc;
use tauri::{Manager, RunEvent};
use tokio::sync::RwLock;
use log::info;
use crate::features::http_server::start_http_server;
use crate::features::mcp_proxy::{MCPState, register_tool, list_tools, list_all_server_tools, discover_tools, execute_tool, execute_proxy_tool, update_tool_status, update_tool_config, uninstall_tool, get_claude_config, get_claude_stdio_config, get_all_server_data, mcp_hello_world};
use tray::create_tray;

mod features;
mod tray;
mod commands {
    use std::process::Command;
    
    #[tauri::command]
    pub async fn greet(name: &str) -> Result<String, String> {
        Ok(format!("Hello, {}! You've been greeted from Rust!", name))
    }
    
    #[tauri::command]
    pub async fn check_node_installed() -> Result<bool, String> {
        match Command::new("node").arg("--version").output() {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
    
    #[tauri::command]
    pub async fn check_uv_installed() -> Result<bool, String> {
        match Command::new("uv").arg("--version").output() {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
    
    #[tauri::command]
    pub async fn check_docker_installed() -> Result<bool, String> {
        match Command::new("docker").arg("--version").output() {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

fn cleanup_mcp_processes(app_handle: &tauri::AppHandle) {
    if let Some(state) = app_handle.try_state::<MCPState>() {
        let tool_registry = state.tool_registry.clone();
        
        if let Ok(handle) = tokio::runtime::Handle::try_current() {
            handle.spawn(async move {
                let mut registry = tool_registry.write().await;
                registry.kill_all_processes().await;
            });
        }
    }
}

async fn init_mcp_services() -> MCPState {
    let mcp_state = MCPState::default();
    let http_state = Arc::new(RwLock::new(mcp_state.clone()));
    
    info!("Starting MCP HTTP server...");
    start_http_server(http_state).await;
    info!("MCP HTTP server started");
    
    mcp_state
}

#[cfg(target_os = "macos")]
fn handle_window_reopen(app_handle: &tauri::AppHandle) {
    let main_window_label = "main";
    
    if let Some(window) = app_handle.get_webview_window(main_window_label) {
        window.show().unwrap();
        window.center().unwrap();
        let _ = window.set_focus();
    } else {
        let main_window_config = app_handle
            .config()
            .app
            .windows
            .iter()
            .find(|w| w.label == main_window_label)
            .unwrap()
            .clone();
            
        if let Ok(builder) = tauri::WebviewWindowBuilder::from_config(app_handle, &main_window_config) {
            if let Err(e) = builder.build() {
                eprintln!("Failed to build main window: {}", e);
            }
        } else {
            eprintln!("Failed to create WebviewWindowBuilder from config");
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let mcp_state = init_mcp_services().await;

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(mcp_state)
        .setup(|app| {
            create_tray(app.handle())?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            commands::check_node_installed,
            commands::check_uv_installed,
            commands::check_docker_installed,
            register_tool,
            list_tools,
            list_all_server_tools,
            discover_tools,
            execute_tool,
            execute_proxy_tool,
            update_tool_status,
            update_tool_config,
            uninstall_tool,
            get_claude_config,
            get_claude_stdio_config,
            get_all_server_data,
            mcp_hello_world
        ])
        .build(tauri::generate_context!())
        .expect("Error while running Tauri application")
        .run(move |app_handle, event| match event {
            RunEvent::ExitRequested { api, .. } => {
                cleanup_mcp_processes(app_handle);
                api.prevent_exit();
            }
            RunEvent::Exit { .. } => {
                cleanup_mcp_processes(app_handle);
                std::process::exit(0);
            }
            #[cfg(target_os = "macos")]
            RunEvent::Reopen { .. } => handle_window_reopen(app_handle),
            _ => {}
        });
}
