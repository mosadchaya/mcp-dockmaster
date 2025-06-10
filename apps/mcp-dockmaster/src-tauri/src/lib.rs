use crate::features::mcp_proxy::{
    check_database_exists_command, clear_database_command, discover_tools, execute_proxy_tool,
    get_tools_visibility_state, import_server_from_url, list_all_server_tools, list_servers,
    register_custom_server, register_server, restart_server_command, set_tools_hidden, 
    uninstall_server, update_server_config, update_server_status,
};
use commands::{get_app_identifier, get_mcp_proxy_server_binary_path};
use features::mcp_proxy::{
    check_claude_installed, check_cursor_installed, get_claude_config, get_cursor_config,
    get_generic_config, install_claude, install_cursor, is_process_running, restart_process,
};
use log::{error, info};
use mcp_core::core::mcp_core::MCPCore;
use mcp_core_utils::{init_mcp_core, uninit_mcp_core};
use tauri::{Emitter, Manager, RunEvent};
use tray::create_tray;
use updater::{check_for_updates, check_for_updates_command};
use windows::{recreate_window, Window};

mod features;
mod mcp_core_utils;
mod tray;
mod updater;
mod windows;

mod commands {
    use std::sync::atomic::{AtomicBool, Ordering};

    use mcp_core::core::{mcp_core::MCPCore, mcp_core_runtimes_ext::McpCoreRuntimesExt};
    use tauri::State;

    use crate::mcp_core_utils::MCPCoreOptions;

    // Global flag to track initialization status
    pub static INITIALIZATION_COMPLETE: AtomicBool = AtomicBool::new(false);

    #[tauri::command]
    pub async fn check_node_installed() -> Result<bool, String> {
        MCPCore::is_nodejs_installed().await
    }

    #[tauri::command]
    pub async fn check_uv_installed() -> Result<bool, String> {
        MCPCore::is_uv_installed().await
    }

    #[tauri::command]
    pub async fn check_docker_installed() -> Result<bool, String> {
        MCPCore::is_docker_installed().await
    }

    #[tauri::command]
    pub async fn check_initialization_complete() -> Result<bool, String> {
        Ok(INITIALIZATION_COMPLETE.load(Ordering::Relaxed))
    }

    #[tauri::command]
    pub async fn get_mcp_proxy_server_binary_path(
        mcp_core_options: State<'_, MCPCoreOptions>,
    ) -> Result<String, String> {
        Ok(mcp_core_options
            .proxy_server_sidecar_path
            .to_string_lossy()
            .to_string())
    }

    #[tauri::command]
    pub async fn get_app_identifier(app_handle: tauri::AppHandle) -> Result<String, String> {
        Ok(app_handle.config().identifier.clone())
    }
}

fn init_services(app_handle: tauri::AppHandle) {
    tokio::spawn(async move {
        let mcp_core = app_handle.state::<MCPCore>();
        let result = mcp_core.init().await;
        if let Err(e) = result {
            error!("Failed to initialize MCP services: {e:?}");
        }

        // Set the initialization complete flag
        commands::INITIALIZATION_COMPLETE.store(true, std::sync::atomic::Ordering::Relaxed);

        info!("Background initialization of MCP services completed");

        // Emit an event to notify the frontend that initialization is complete
        if let Err(e) = app_handle.emit("mcp-initialization-complete", ()) {
            error!("Failed to emit initialization complete event: {e}");
        } else {
            info!("Emitted initialization complete event");
        }
    });
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

        if let Ok(builder) =
            tauri::WebviewWindowBuilder::from_config(app_handle, &main_window_config)
        {
            if let Err(e) = builder.build() {
                eprintln!("Failed to build main window: {}", e);
            }
        } else {
            eprintln!("Failed to create WebviewWindowBuilder from config");
        }
    }
}

pub async fn app_init(app_handle: &tauri::AppHandle) -> Result<(), String> {
    create_tray(app_handle).map_err(|e| e.to_string())?;

    // Check for updates
    let _ = check_for_updates(app_handle, false).await;

    // Initialize the MCP core
    init_mcp_core(app_handle).await?;

    let app_handle_clone = app_handle.clone();
    recreate_window(app_handle_clone, Window::Main, true).map_err(|e| e.to_string())?;

    // Start background initialization after the UI has started
    let app_handle = app_handle.clone();
    init_services(app_handle);
    Ok(())
}

pub async fn app_uninit(app_handle: &tauri::AppHandle) {
    uninit_mcp_core(app_handle).await;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {}))
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            // Check for updates in the background when the app is opened
            let app_handle_clone = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                app_init(&app_handle_clone).await.unwrap_or_else(|e| {
                    error!("Failed to initialize app: {e}");
                    app_handle_clone.exit(1);
                });
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::check_node_installed,
            commands::check_uv_installed,
            commands::check_docker_installed,
            commands::check_initialization_complete,
            register_server,
            register_custom_server,
            list_servers,
            list_all_server_tools,
            discover_tools,
            execute_proxy_tool,
            update_server_status,
            update_server_config,
            restart_server_command,
            uninstall_server,
            check_database_exists_command,
            clear_database_command,
            check_claude_installed,
            check_cursor_installed,
            install_claude,
            install_cursor,
            get_claude_config,
            get_cursor_config,
            get_generic_config,
            import_server_from_url,
            restart_process,
            is_process_running,
            check_for_updates_command,
            set_tools_hidden,
            get_tools_visibility_state,
            get_mcp_proxy_server_binary_path,
            get_app_identifier,
        ])
        .build(tauri::generate_context!())
        .expect("Error while running Tauri application")
        .run(move |app_handle, event| match event {
            RunEvent::ExitRequested { api, .. } => {
                /*
                    First, prevent exit to handle cleanup
                    This is ignored when using AppHandle#method.restart.
                */
                info!("[RunEvent:ExitRequested] preventing exit to handle cleanup");
                api.prevent_exit();
            }
            RunEvent::Exit => {
                // Cleanup processes
                let app_handle = app_handle.clone();
                tauri::async_runtime::spawn(async move {
                    info!("[RunEvent:Exit] uninit app");
                    app_uninit(&app_handle).await;
                    info!("[RunEvent::Exit] exiting app");
                    std::process::exit(0);
                });
            }
            #[cfg(target_os = "macos")]
            RunEvent::Reopen { .. } => handle_window_reopen(app_handle),
            _ => {}
        });
}
