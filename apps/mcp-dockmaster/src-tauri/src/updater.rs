use log::{error, info};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};
use tauri_plugin_updater::UpdaterExt;

pub async fn check_for_updates(app_handle: &tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    let updater = app_handle
        .updater_builder()
        .on_before_exit(|| {
            println!("app is about to exit on Windows!");
        })
        .build()?;

    if let Some(update) = updater.check().await? {
        info!("Update available: {}", update.version);

        let answer = app_handle
            .dialog()
            .message("Would you like to install it? This will restart the application to apply the update.")
            .title(format!("New Update Available v{}", update.version))
            .buttons(MessageDialogButtons::OkCancelCustom(
                "Download & Install".to_string(),
                "Later".to_string(),
            ))
            .blocking_show();

        if !answer {
            info!("new update available but user cancelled installation");
            return Ok(());
        }

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
                app_handle.restart();
            }
            Err(e) => {
                error!("Failed to install update: {}", e);
                if e.to_string().contains("InvalidSignature") {
                    error!("Update signature verification failed. This could mean the update package has been tampered with or the public key doesn't match.");
                }
            }
        }
    } else {
        app_handle
            .dialog()
            .message("You're running the latest version.")
            .title("No Updates Available")
            .blocking_show();
        info!("No update available");
    }
    Ok(())
}

#[tauri::command]
pub async fn check_for_updates_command(app_handle: tauri::AppHandle) -> Result<(), String> {
    check_for_updates(&app_handle)
        .await
        .map_err(|e| e.to_string())
}
