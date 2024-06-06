// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use tauri::{Manager, WindowBuilder, WindowUrl, State};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

struct AppState {
    font_size: Mutex<u32>,
    padding: Mutex<u32>,
}

#[tauri::command]
fn update_settings(state: State<AppState>, font_size: u32, padding: u32) {
    *state.font_size.lock().unwrap() = font_size;
    *state.padding.lock().unwrap() = padding;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            window.set_decorations(false).unwrap();
            tauri::async_runtime::spawn(show_random_character(window));

            let settings_window = WindowBuilder::new(app, "settings", WindowUrl::App("settings.html".into()))
                .title("Settings")
                .inner_size(400.0, 300.0)
                .build()
                .unwrap();

            Ok(())
        })
        .manage(AppState {
            font_size: Mutex::new(24),
            padding: Mutex::new(10),
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
