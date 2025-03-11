use log::{error, info};
use mcp_core::core::{mcp_core::MCPCore, mcp_core_proxy_ext::McpCoreProxyExt};
use tauri::Manager;
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};
use tauri_plugin_updater::UpdaterExt;

fn uninit_mcp_core(app_handle: &tauri::AppHandle) {
    let mcp_core = app_handle.state::<MCPCore>();
    if let Ok(handle) = tokio::runtime::Handle::try_current() {
        let mcp_core = mcp_core.inner().clone();
        handle.spawn(async move {
            let result = mcp_core.kill_all_processes().await;
            if let Err(e) = result {
                error!("failed to kill all MCP processes: {}", e);
            }
        });
    }
}

pub async fn check_for_updates(app_handle: &tauri::AppHandle) {
    let app_handle_clone = app_handle.clone();
    let updater = app_handle
        .updater_builder()
        .on_before_exit(move || {
            println!("app is about to exit on Windows!");
            uninit_mcp_core(&app_handle_clone);
        })
        .build()
        .unwrap();

    if let Some(update) = updater.check().await.unwrap_or_else(|e| {
        error!("Failed to check for updates: {}", e);
        None
    }) {
        info!("Update available: {}", update.version);
        let app_handle_clone = app_handle.clone();
        let dialog = app_handle
            .dialog()
            .message("Would you like to install it? This will restart the application to apply the update.")
            .buttons(MessageDialogButtons::OkCancelCustom(
                "Install".to_string(),
                "Later".to_string(),
            ))
            .title(format!("New Update Available v{}", update.version));
        dialog.show(move |answer| {
        info!("user answer to the update prompt: {}", answer);
        if !answer {
            info!("new update available but user cancelled installation");
            return;
        }

        tauri::async_runtime::spawn(async move {
            let mut downloaded = 0;
            match update
                .download_and_install(
                    |chunk_length, content_length| {
                        downloaded += chunk_length;
                        info!("Downloaded {downloaded} from {content_length:?}");
                    },
                    || {
                        info!("Download finished, preparing to install...");
                    },
                )
                .await
            {
                Ok(_) => {
                    info!("Update installed successfully, restarting...");
                    app_handle_clone.restart();
                }
                Err(e) => {
                    error!("Failed to install update: {}", e);
                    if e.to_string().contains("InvalidSignature") {
                        error!("Update signature verification failed. This could mean the update package has been tampered with or the public key doesn't match.");
                    }
                }
            }
        });
        });
    } else {
        app_handle
            .dialog()
            .message("You're running the latest version.")
            .title("No Updates Available")
            .show(|_| {});
        info!("no update available");
    }
}

#[tauri::command]
pub async fn check_for_updates_command(app_handle: tauri::AppHandle) {
    check_for_updates(&app_handle).await
}
