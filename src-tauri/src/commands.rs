use resolution_core::{Config, get_current_resolution as get_res};
use serde_json::Value;
use tauri::WebviewWindow;

#[tauri::command]
pub fn load_config() -> Result<Config, String> {
    Config::load().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_config(config: Config) -> Result<(), String> {
    config.save().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_current_resolution() -> Result<Value, String> {
    let res = get_res().map_err(|e| e.to_string())?;
    Ok(serde_json::json!({ "w": res.w, "h": res.h }))
}

#[tauri::command]
pub fn minimize_window(window: WebviewWindow) -> Result<(), String> {
    window.minimize().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn close_window(window: WebviewWindow) -> Result<(), String> {
    window.close().map_err(|e| e.to_string())
}
