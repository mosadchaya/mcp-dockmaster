use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    Manager,
};

use crate::{
    app_uninit,
    updater::check_for_updates,
    windows::{recreate_window, Window},
};

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
    let icon = if cfg!(target_os = "macos") {
        tauri::image::Image::from_bytes(include_bytes!("../icons/tray-icon-macos.png"))?
    } else {
        app.default_window_icon().unwrap().clone()
    };
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
                let app_handle = tray.app_handle().clone();
                tauri::async_runtime::spawn(async move {
                    app_uninit(&app_handle).await;
                    std::process::exit(0);
                });
            }
            "check_for_updates" => {
                let app_handle_clone = tray.app_handle().clone();
                tauri::async_runtime::spawn(async move {
                    let _ = check_for_updates(&app_handle_clone, true).await;
                });
            }
            _ => (),
        })
        .build(app)?;
    Ok(())
}
