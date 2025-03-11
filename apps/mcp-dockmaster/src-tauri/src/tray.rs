use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, WebviewWindow, WebviewWindowBuilder,
};

use crate::updater::check_for_updates;

#[derive(Debug, Clone, Copy)]
pub enum Window {
    Main,
}

impl Window {
    pub fn as_str(&self) -> &'static str {
        match self {
            Window::Main => "main",
        }
    }
}

pub fn create_tray(app: &tauri::AppHandle) -> tauri::Result<()> {
    let quit_menu_item = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
    let show_menu_item = MenuItemBuilder::with_id("show", "Show").build(app)?;
    let check_for_updates_menu_item =
        MenuItemBuilder::with_id("check_for_updates", "Check for Updates").build(app)?;
    let menu = MenuBuilder::new(app)
        .items(&[
            &quit_menu_item,
            &show_menu_item,
            &check_for_updates_menu_item,
        ])
        .build()?;
    let is_template = cfg!(target_os = "macos");
    let icon = app.default_window_icon().unwrap().clone();
    let _ = TrayIconBuilder::with_id("tray")
        .icon(icon)
        .icon_as_template(is_template)
        .on_tray_icon_event(|_tray, event| {
            if cfg!(target_os = "windows") {
                if let TrayIconEvent::Click { button, .. } = event {
                    if button == MouseButton::Left {
                        // TODO: Show window
                    }
                }
            }
        })
        .show_menu_on_left_click(!cfg!(target_os = "windows"))
        .menu(&menu)
        .on_menu_event(move |tray, event| match event.id().as_ref() {
            "show" => {
                let _ = recreate_window(tray.app_handle().clone(), Window::Main, true);
            }
            "quit" => {
                tauri::async_runtime::spawn(async move {
                    std::process::exit(0);
                });
            }
            "check_for_updates" => {
                let app_handle_clone = tray.app_handle().clone();
                tauri::async_runtime::spawn(async move {
                    let _ = check_for_updates(&app_handle_clone).await;
                });
            }
            _ => (),
        })
        .build(app)?;
    Ok(())
}

pub fn recreate_window(
    app_handle: AppHandle,
    window_name: Window,
    focus: bool,
) -> tauri::Result<WebviewWindow> {
    let label = window_name.as_str();
    if let Some(window) = app_handle.get_webview_window(label) {
        if focus {
            if window.is_minimized().unwrap_or_default() {
                let _ = window.unminimize();
            }
            window.show().unwrap();
            let _ = window.set_focus();
        }
        Ok(window)
    } else {
        let window_config = app_handle
            .config()
            .app
            .windows
            .iter()
            .find(|w| w.label == label)
            .unwrap()
            .clone();

        match WebviewWindowBuilder::from_config(&app_handle, &window_config) {
            Ok(builder) => match builder.build() {
                Ok(window) => Ok(window),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }
}
