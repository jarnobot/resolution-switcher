#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::load_config,
            commands::save_config,
            commands::get_current_resolution,
            commands::minimize_window,
            commands::close_window,
        ])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            // Set icon explicitly at runtime so Windows uses the full multi-res ICO
            // for the taskbar button and Alt+Tab thumbnail
            let icon_bytes = include_bytes!("../icons/icon.ico");
            if let Ok(icon) = tauri::image::Image::from_bytes(icon_bytes) {
                let _ = window.set_icon(icon);
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
