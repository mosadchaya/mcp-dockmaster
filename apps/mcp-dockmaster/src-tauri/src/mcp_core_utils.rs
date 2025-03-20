use std::{fs, path::PathBuf};

use log::{error, info};
use mcp_core::{
    core::{mcp_core::MCPCore, mcp_core_proxy_ext::McpCoreProxyExt},
    utils::process::kill_all_processes_by_name,
};
use tauri::{utils::platform, Manager};

pub struct MCPCoreOptions {
    #[allow(dead_code)]
    pub database_path: PathBuf,
    pub proxy_server_sidecar_path: PathBuf,
}

/// Get the proxy server sidecar path
/// For linux, as we are distributing an AppImage file, we need to copy the binary to the app data directory
/// so other apps like Claude or Cursor can find it and execute it.
///
/// # Arguments
///
/// * `app_data_path` - The app data path
///
/// # Returns
async fn get_proxy_server_sidecar_path(app_data_path: PathBuf) -> Result<PathBuf, String> {
    let proxy_server_sidecar_name = "mcp-proxy-server";
    let proxy_server_sidecar_path = match std::env::consts::OS {
        "windows" => platform::current_exe()
            .map_err(|_| "failed to get current exe")?
            .parent()
            .unwrap()
            .join(proxy_server_sidecar_name)
            .with_extension("exe"),
        "linux" => {
            if !app_data_path.exists() {
                fs::create_dir_all(&app_data_path).map_err(|e| {
                    format!(
                        "failed to create app data directory {}: {}",
                        app_data_path.display(),
                        e
                    )
                })?;
            }
            let proxy_server_sidecar_path = platform::current_exe()
                .map_err(|_| "failed to get current exe")?
                .parent()
                .unwrap()
                .join(proxy_server_sidecar_name);
            let source_path = proxy_server_sidecar_path.clone();
            let destination_path = app_data_path.join(proxy_server_sidecar_name);

            let mcp_proxy_server_version_file = app_data_path.join("mcp-proxy-server.version");
            let mcp_proxy_server_version = match fs::read_to_string(&mcp_proxy_server_version_file)
            {
                Ok(version) => version,
                Err(_) => "failed reading mcp proxy server version".to_string(),
            };

            if destination_path.exists() && mcp_proxy_server_version == env!("CARGO_PKG_VERSION") {
                return Ok(destination_path);
            }

            kill_all_processes_by_name(destination_path.file_name().unwrap().to_str().unwrap())
                .await;
            if let Err(e) = fs::write(&mcp_proxy_server_version_file, env!("CARGO_PKG_VERSION")) {
                let error_message = format!(
                    "failed to write mcp proxy server version to {}: {}",
                    mcp_proxy_server_version_file.display(),
                    e
                );
                return Err(error_message);
            }
            if let Err(e) = fs::copy(&source_path, &destination_path) {
                let error_message = format!(
                    "failed to copy proxy server sidecar binary from {} to {}: {}",
                    proxy_server_sidecar_path.display(),
                    destination_path.display(),
                    e
                );
                return Err(error_message);
            }
            destination_path
        }
        "macos" => platform::current_exe()
            .map_err(|_| "failed to get current exe")?
            .parent()
            .unwrap()
            .join(proxy_server_sidecar_name),
        _ => return Err("failed to get proxy server sidecar path".to_string()),
    };
    Ok(proxy_server_sidecar_path)
}

/// Initialize the MCP core
///
/// # Arguments
///
/// * `app_handle` - The tauri app handle
///
/// # Returns
///
/// * `()` - The MCP core is initialized and added to the app state
pub async fn init_mcp_core(app_handle: &tauri::AppHandle) -> Result<(), String> {
    let proxy_server_sidecar_path = get_proxy_server_sidecar_path(
        app_handle
            .path()
            .app_data_dir()
            .map_err(|_| "failed to get app data dir")?,
    )
    .await?;

    app_handle.manage(proxy_server_sidecar_path.clone());

    info!(
        "proxy server sidecar path: {}",
        proxy_server_sidecar_path.display()
    );

    let database_path = app_handle
        .path()
        .app_data_dir()
        .map_err(|_| "failed to get app data dir")?
        .join("mcp_dockmaster.db");

    info!("database path: {}", database_path.display());

    let mcp_core = MCPCore::new(
        database_path.clone(),
        proxy_server_sidecar_path.to_path_buf(),
        app_handle.config().identifier.clone(),
    );
    app_handle.manage(MCPCoreOptions {
        database_path,
        proxy_server_sidecar_path,
    });
    app_handle.manage(mcp_core.clone());
    Ok(())
}

pub async fn uninit_mcp_core(app_handle: &tauri::AppHandle) {
    info!("uninit_mcp_core, getting handle");
    let app_handle_clone = app_handle.clone();
    let mcp_core = app_handle_clone.try_state::<MCPCore>();
    let mcp_core_options = app_handle_clone.try_state::<MCPCoreOptions>();

    // Kill all MCP Server processes
    if let Some(mcp_core) = mcp_core {
        info!("killing all MCP processes");
        let result = mcp_core.kill_all_processes().await;
        if let Err(e) = result {
            error!("failed to kill all MCP processes: {}", e);
        } else {
            info!("killing all MCP processes done");
        }
    }

    /*
        Kill the mcp proxy server process
        These processes are created by mcp clientes like cursor or claude
    */
    if let Some(mcp_core_options) = mcp_core_options {
        info!("killing mcp proxy server processes");
        let mcp_proxy_server_binary_path = mcp_core_options
            .proxy_server_sidecar_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();
        let _ = kill_all_processes_by_name(mcp_proxy_server_binary_path).await;
        info!("killing mcp proxy server processes done");
    }
}
