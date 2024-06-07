// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use keystroke::start_keystroke_listener;
use std::sync::Mutex;
use tauri::{Manager, State, WindowBuilder, WindowUrl};
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

mod keystroke;

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
fn update_settings(font_size: u32, padding: u32, state: State<AppState>) {
    println!("{font_size} {padding}");
    *state.font_size.lock().unwrap() = font_size;
    *state.padding.lock().unwrap() = padding;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, update_settings])
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            window.set_decorations(false).unwrap();
            #[cfg(target_os = "macos")]
            apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
                .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

            #[cfg(target_os = "windows")]
            apply_blur(&window, Some((18, 18, 18, 125)))
                .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

            start_keystroke_listener(app);

            let settings_window =
                WindowBuilder::new(app, "settings", WindowUrl::App("settings.html".into()))
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

async fn display_keystrokes(rx: std::sync::mpsc::Receiver<String>) {
    while let Ok(key) = rx.recv() {
        println!("Key pressed: {}", key);
    }
}
