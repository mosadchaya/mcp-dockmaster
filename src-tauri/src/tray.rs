use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, WebviewWindow, WebviewWindowBuilder
};

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

    let menu = MenuBuilder::new(app)
        .items(&[&quit_menu_item, &show_menu_item])
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
            _ => (),
        })
        .build(app)?;
    Ok(())
}

pub fn recreate_window(app_handle: AppHandle, window_name: Window, focus: bool) -> tauri::Result<WebviewWindow> {
    let label = window_name.as_str();
    if let Some(window) = app_handle.get_webview_window(label) {
        // log::info!("window {} found, bringing to front", label);
        if focus {
            // log::info!("focusing window {}", label);
            if window.is_minimized().unwrap_or_default() {
                let _ = window.unminimize();
            }
            window.show().unwrap();
            // window.center().unwrap();
            let _ = window.set_focus();
        }
        Ok(window)
    } else {
        // log::info!("window {} not found, recreating...", label);
        let window_config = app_handle
            .config()
            .app
            .windows
            .iter()
            .find(|w| w.label == label)
            .unwrap()
            .clone();
        // if window_config.label == Window::Coordinator.as_str() {
        //     window_config.background_throttling = Some(BackgroundThrottlingPolicy::Disabled);
        // }
        match WebviewWindowBuilder::from_config(&app_handle, &window_config) {
            Ok(builder) => match builder.build() {
                Ok(window) => {
                    // log::info!("window {} created", label);
                    Ok(window)
                }
                Err(e) => {
                    // log::error!("failed to recreate window: {}", e);
                    Err(e)
                },
            },
            Err(e) => {
                // log::error!("failed to recreate window from config: {}", e);
                Err(e)
            }
        }
    }
}