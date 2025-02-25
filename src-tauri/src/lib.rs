// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn check_node_installed() -> bool {
    match std::process::Command::new("node").arg("--version").output() {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[tauri::command]
fn check_uv_installed() -> bool {
    match std::process::Command::new("uv").arg("--version").output() {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[tauri::command]
fn check_docker_installed() -> bool {
    match std::process::Command::new("docker").arg("--version").output() {
        Ok(_) => true,
        Err(_) => false,
    }
}

use tauri::{Manager, RunEvent};
use tray::create_tray;
mod tray;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            create_tray(app.handle())?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            check_node_installed,
            check_uv_installed,
            check_docker_installed
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(move |app_handle, event| match event {
            RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            RunEvent::Exit { .. } => {
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
