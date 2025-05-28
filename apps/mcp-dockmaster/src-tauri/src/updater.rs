use log::{error, info};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};
use tauri_plugin_updater::UpdaterExt;

use crate::app_uninit;

pub async fn check_for_updates(
    app_handle: &tauri::AppHandle,
    show_dialog_when_no_update_available: bool,
) {
    let app_handle_clone = app_handle.clone();
    let updater = app_handle
        .updater_builder()
        .on_before_exit(move || {
            let app_handle_clone = app_handle_clone.clone();
            tauri::async_runtime::spawn(async move {
                println!("app is about to exit on Windows!");
                let _ = app_uninit(&app_handle_clone).await;
            });
        })
        .build()
        .unwrap();

    if let Some(update) = updater.check().await.unwrap_or_else(|e| {
        error!("failed to check for updates: {e}");
        None
    }) {
        info!("update available: {}", update.version);
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
        info!("user answer to the update prompt: {answer}");
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
                        info!("downloaded {downloaded} from {content_length:?}");
                    },
                    || {
                        info!("download finished, preparing to install...");
                    },
                )
                .await
            {
                Ok(_) => {
                    info!("update installed successfully");

                    info!("uninitializing app...");
                    app_uninit(&app_handle_clone).await;

                    info!("restarting app...");
                    app_handle_clone.restart();
                }
                Err(e) => {
                    error!("failed to install update: {e}");
                    if e.to_string().contains("InvalidSignature") {
                        error!("update signature verification failed. This could mean the update package has been tampered with or the public key doesn't match.");
                    }
                }
            }
        });
        });
    } else if show_dialog_when_no_update_available {
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
    check_for_updates(&app_handle, true).await
}
