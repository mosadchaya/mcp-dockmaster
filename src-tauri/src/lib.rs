// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn greet(name: &str) -> Result<String, String> {
    Ok(format!("Hello, {}! You've been greeted from Rust!", name))
}

#[tauri::command]
async fn check_node_installed() -> Result<bool, String> {
    match std::process::Command::new("node").arg("--version").output() {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

#[tauri::command]
async fn check_uv_installed() -> Result<bool, String> {
    match std::process::Command::new("uv").arg("--version").output() {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

#[tauri::command]
async fn check_docker_installed() -> Result<bool, String> {
    match std::process::Command::new("docker").arg("--version").output() {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

use tauri::{Manager, RunEvent};
use tray::create_tray;
mod tray;
// Add MCP module
pub mod mcp_proxy;

// Ensure we clean up processes when the application exits
fn cleanup_mcp_processes(app_handle: &tauri::AppHandle) {
    if let Some(state) = app_handle.try_state::<mcp_proxy::MCPState>() {
        // Use a blocking task to clean up processes
        let tool_registry = state.tool_registry.clone();
        std::thread::spawn(move || {
            let runtime = tokio::runtime::Runtime::new().unwrap();
            runtime.block_on(async {
                let mut registry = tool_registry.write().await;
                registry.kill_all_processes();
            });
        });
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // Add MCP state to the app
        .manage(mcp_proxy::MCPState::default())
        .setup(|app| {
            create_tray(app.handle())?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            check_node_installed,
            check_uv_installed,
            check_docker_installed,
            // Register MCP commands
            mcp_proxy::register_tool,
            mcp_proxy::list_tools,
            mcp_proxy::list_all_server_tools,
            mcp_proxy::discover_tools,
            mcp_proxy::execute_tool,
            mcp_proxy::execute_proxy_tool,
            mcp_proxy::update_tool_status,
            mcp_proxy::uninstall_tool,
            mcp_proxy::get_claude_config,
            mcp_proxy::get_all_server_data,
            mcp_proxy::mcp_hello_world
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(move |app_handle, event| match event {
            RunEvent::ExitRequested { api, .. } => {
                // Clean up processes before exiting
                cleanup_mcp_processes(app_handle);
                api.prevent_exit();
            }
            RunEvent::Exit { .. } => {
                // Clean up processes before exiting
                cleanup_mcp_processes(app_handle);
                std::process::exit(0);
            }
            #[cfg(target_os = "macos")]
            RunEvent::Reopen { .. } => {
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
                    match tauri::WebviewWindowBuilder::from_config(app_handle, &main_window_config)
                    {
                        Ok(builder) => {
                            if let Err(e) = builder.build() {
                                println!("failed to build main window: {}", e);
                            }
                        }
                        Err(e) => {
                            println!("failed to create WebviewWindowBuilder from config: {}", e);
                        }
                    }
                }
            }
            _ => {}
        });
}
