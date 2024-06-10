// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};
use tauri::{Manager, State};
use keystroke::start_keystroke_listener;
use tray::create_tray_menu;

mod keystroke;
mod tray;

#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}

#[derive(Clone)]
struct AppState {
    font_size: Arc<Mutex<u32>>,
    padding: Arc<Mutex<u32>>,
    selected_sound: Arc<Mutex<String>>,
}

#[tauri::command]
fn update_settings(font_size: u32, padding: u32, state: State<AppState>) {
    println!("{font_size} {padding}");
    *state.font_size.lock().unwrap() = font_size;
    *state.padding.lock().unwrap() = padding;
}

#[tauri::command]
fn set_selected_sound(sound: String, state: tauri::State<AppState>) {
    let mut selected_sound = state.selected_sound.lock().unwrap();
    *selected_sound = sound;
}

#[tauri::command]
fn get_selected_sound(state: tauri::State<'_, AppState>) -> String {
    let selected_sound = state.selected_sound.lock().unwrap();
    selected_sound.clone()
}

fn main() {
    let tauri_app = tauri::Builder::default();
    let tray = create_tray_menu();

    tauri_app
        .invoke_handler(tauri::generate_handler![
            update_settings,
            set_selected_sound,
            get_selected_sound
        ])
        .setup(|app| {
            start_keystroke_listener(app);
            Ok(())
        })
        .on_window_event(|event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
                let label = event.window().label();
                if label == "settings" {
                    event.window().hide().unwrap();
                    api.prevent_close();
                }
            }
        })
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            app.emit_all("single-instance", Payload { args: argv, cwd })
                .unwrap();
        }))
        .system_tray(tray)
        .on_system_tray_event(|app, event| {
            tray::on_tray_event(app, event);
        })
        .manage(AppState {
            font_size: Arc::new(Mutex::new(24)),
            padding: Arc::new(Mutex::new(10)),
            selected_sound: Arc::new(Mutex::new("key_1.mp3".to_string())),
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
