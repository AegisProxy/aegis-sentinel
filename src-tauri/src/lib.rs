mod clipboard_monitor;

use clipboard_monitor::{ClipboardMonitor, SecurityEvent};
use tauri::{Manager, State};
use std::sync::Mutex;

struct AppState {
    clipboard_monitor: Mutex<ClipboardMonitor>,
    risk_level: Mutex<u32>,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_risk_level(state: State<AppState>) -> u32 {
    *state.risk_level.lock().unwrap()
}

#[tauri::command]
fn start_monitoring(state: State<AppState>, app_handle: tauri::AppHandle) {
    let monitor = state.clipboard_monitor.lock().unwrap();
    monitor.start(app_handle);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            let app_handle = app.handle().clone();
            let state = app.state::<AppState>();
            
            // Start clipboard monitoring automatically
            let monitor = state.clipboard_monitor.lock().unwrap();
            monitor.start(app_handle);
            
            Ok(())
        })
        .manage(AppState {
            clipboard_monitor: Mutex::new(ClipboardMonitor::new()),
            risk_level: Mutex::new(0),
        })
        .invoke_handler(tauri::generate_handler![greet, get_risk_level, start_monitoring])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
