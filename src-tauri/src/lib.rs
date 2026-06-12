mod cache;
mod commands;
mod models;
mod scheduler;
mod vendors;

use std::sync::Mutex;

use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager,
};

use models::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            let config = commands::config::load_config_file();
            let cached_usage = cache::load_cache().unwrap_or_default();

            let state = AppState {
                cached_usage,
                config,
                last_update: None,
            };
            app.manage(Mutex::new(state));

            let show_item = MenuItem::with_id(app, "show", "Show Window", true, None::<&str>)?;
            let refresh_item = MenuItem::with_id(app, "refresh", "Refresh", true, None::<&str>)?;
            let separator = PredefinedMenuItem::separator(app)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_item, &refresh_item, &separator, &quit_item])?;

            TrayIconBuilder::with_id("main-tray")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "refresh" => {
                        let app_handle = app.clone();
                        tauri::async_runtime::spawn(async move {
                            let _ = scheduler::do_refresh(&app_handle).await;
                        });
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .build(app)?;

            let app_handle = app.handle().clone();
            let interval_secs = {
                let state = app.state::<Mutex<AppState>>();
                let s = state.lock().map_err(|e| e.to_string())?;
                s.config.refresh_interval_secs
            };

            {
                let state = app.state::<Mutex<AppState>>();
                let s = state.lock().map_err(|e| e.to_string())?;
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.emit("usage-updated", &s.cached_usage);
                }
            }

            scheduler::start_scheduler(app_handle, interval_secs);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::usage::get_usage,
            commands::usage::refresh_usage,
            commands::usage::get_last_update_time,
            commands::config::get_config,
            commands::config::save_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
