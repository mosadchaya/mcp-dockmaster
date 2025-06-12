use crate::features::mcp_proxy::{
    analyze_github_repository, analyze_local_directory, check_database_exists_command, clear_database_command, 
    discover_tools, execute_proxy_tool, get_tools_visibility_state, import_server_from_url, 
    list_all_server_tools, list_servers, register_custom_server, register_server, 
    restart_server_command, set_tools_hidden, uninstall_server, update_server_config, 
    update_server_status,
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

    #[tauri::command]
    pub async fn check_uv_project(directory: String) -> Result<bool, String> {
        use std::path::Path;
        
        let dir_path = Path::new(&directory);
        
        // Check if uv.lock exists
        let uv_lock_path = dir_path.join("uv.lock");
        if uv_lock_path.exists() {
            return Ok(true);
        }
        
        // Check if pyproject.toml exists (indicates modern Python project)
        let pyproject_path = dir_path.join("pyproject.toml");
        if pyproject_path.exists() {
            // Check if the pyproject.toml contains uv-specific content
            match std::fs::read_to_string(&pyproject_path) {
                Ok(content) => {
                    let content_lower = content.to_lowercase();
                    // Look for uv-specific markers in pyproject.toml
                    if content_lower.contains("uv") || 
                       content_lower.contains("requires-python") ||
                       content_lower.contains("build-system") {
                        return Ok(true);
                    }
                }
                Err(_) => {
                    // If we can't read the file, assume it's a modern Python project
                    return Ok(true);
                }
            }
        }
        
        Ok(false)
    }

    #[tauri::command]
    pub async fn check_node_project(directory: String) -> Result<Option<String>, String> {
        use std::path::Path;
        
        let dir_path = Path::new(&directory);
        
        // Check if package.json exists
        let package_json_path = dir_path.join("package.json");
        if package_json_path.exists() {
            // Try to parse package.json to find scripts
            match std::fs::read_to_string(&package_json_path) {
                Ok(content) => {
                    match serde_json::from_str::<serde_json::Value>(&content) {
                        Ok(package_json) => {
                            // Check for common start scripts in order of preference
                            let script_preferences = ["start", "serve", "dev", "run"];
                            
                            if let Some(scripts) = package_json.get("scripts").and_then(|s| s.as_object()) {
                                for script_name in script_preferences {
                                    if scripts.contains_key(script_name) {
                                        return Ok(Some(format!("npm run {}", script_name)));
                                    }
                                }
                                
                                // If no preferred scripts found but scripts exist, use npm start as default
                                if !scripts.is_empty() {
                                    return Ok(Some("npm start".to_string()));
                                }
                            }
                            
                            // If no scripts, suggest node directly
                            return Ok(Some("node".to_string()));
                        }
                        Err(_) => {
                            // If we can't parse package.json, default to node
                            return Ok(Some("node".to_string()));
                        }
                    }
                }
                Err(_) => {
                    // If we can't read package.json, default to node
                    return Ok(Some("node".to_string()));
                }
            }
        }
        
        // Check for other Node.js indicators
        let node_indicators = ["package-lock.json", "yarn.lock", "pnpm-lock.yaml", "bun.lockb"];
        for indicator in node_indicators {
            if dir_path.join(indicator).exists() {
                return Ok(Some("node".to_string()));
            }
        }
        
        Ok(None)
    }

    #[tauri::command]
    pub async fn check_docker_project(directory: String) -> Result<Option<String>, String> {
        use std::path::Path;
        
        let dir_path = Path::new(&directory);
        
        // Check for docker-compose files first (more specific)
        let compose_files = [
            "docker-compose.yml", 
            "docker-compose.yaml", 
            "compose.yml", 
            "compose.yaml"
        ];
        
        for compose_file in compose_files {
            if dir_path.join(compose_file).exists() {
                return Ok(Some("docker-compose up".to_string()));
            }
        }
        
        // Check for Dockerfile
        let dockerfile_names = ["Dockerfile", "dockerfile"];
        for dockerfile in dockerfile_names {
            if dir_path.join(dockerfile).exists() {
                return Ok(Some("docker run".to_string()));
            }
        }
        
        Ok(None)
    }

    #[tauri::command]
    pub async fn read_text_file(path: String) -> Result<String, String> {
        use std::fs;
        fs::read_to_string(&path).map_err(|e| format!("Failed to read file {}: {}", path, e))
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
            commands::check_uv_project,
            commands::check_node_project,
            commands::check_docker_project,
            commands::read_text_file,
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
            analyze_github_repository,
            analyze_local_directory,
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
