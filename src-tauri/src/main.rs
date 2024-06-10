// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dirs::config_dir;
use keystroke::start_keystroke_listener;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::{Manager, State};
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
    sound: Arc<Mutex<String>>,
    sound_enabled: Arc<Mutex<bool>>,
}

#[derive(Serialize, Deserialize)]
struct Config {
    font_size: u32,
    padding: u32,
    sound: String,
}

#[tauri::command]
fn update_settings(font_size: u32, padding: u32, sound: String, state: State<AppState>) {
    println!("{font_size} {padding} {sound}");
    *state.font_size.lock().unwrap() = font_size;
    *state.padding.lock().unwrap() = padding;
    *state.sound.lock().unwrap() = sound.clone();
    save_state(&Config {
        font_size,
        padding,
        sound,
    })
    .expect("Failed to save settings");
}

#[tauri::command]
fn get_state(state: tauri::State<'_, AppState>) -> Config {
    Config {
        font_size: *state.font_size.lock().unwrap(),
        padding: *state.padding.lock().unwrap(),
        sound: state.sound.lock().unwrap().clone(),
    }
}

fn get_config_path() -> PathBuf {
    let mut path = config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("typeview");
    fs::create_dir_all(&path).expect("Failed to create config directory");
    path.push("typeview.json");
    path
}

fn load_state() -> io::Result<AppState> {
    let config_path = get_config_path();
    let config_path_str = config_path.display().to_string();
    println!("{config_path_str}");
    if config_path.exists() {
        let mut file = File::open(config_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Config = serde_json::from_str(&contents)?;
        Ok(AppState {
            font_size: Arc::new(Mutex::new(config.font_size)),
            padding: Arc::new(Mutex::new(config.padding)),
            sound: Arc::new(Mutex::new(config.sound)),
            sound_enabled: Arc::new(Mutex::new(true)),
        })
    } else {
        let config = Config {
            font_size: 24,
            padding: 10,
            sound: "key_1.mp3".to_string(),
        };
        let state = AppState {
            font_size: Arc::new(Mutex::new(config.font_size)),
            padding: Arc::new(Mutex::new(config.padding)),
            sound: Arc::new(Mutex::new(config.sound.clone())),
            sound_enabled: Arc::new(Mutex::new(true)),
        };
        save_state(&config).expect("Failed to save settings");
        Ok(state)
    }
}

fn save_state(config: &Config) -> io::Result<()> {
    let config_path = get_config_path();
    let json = serde_json::to_string(&config)?;
    let mut file = File::create(config_path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

fn main() {
    let tauri_app = tauri::Builder::default();
    let tray = create_tray_menu();

    let initial_state = load_state().expect("Failed to load initial state");

    tauri_app
        .invoke_handler(tauri::generate_handler![update_settings, get_state])
        .setup(|app| {
            let setting_window = app.get_window("settings").unwrap();
            setting_window.hide().unwrap();
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
        .manage(initial_state)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
