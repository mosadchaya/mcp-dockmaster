use tauri::{AppHandle, Manager, WebviewWindow, WebviewWindowBuilder};

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
