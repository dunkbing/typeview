use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};

use crate::AppState;

pub fn create_tray_menu() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let toggle_key = CustomMenuItem::new("toggle-key".to_string(), "Toggle Key");
    let toggle_sound = CustomMenuItem::new("toggle-sound".to_string(), "Toggle Sound");
    let settings = CustomMenuItem::new("settings".to_string(), "Settings");

    let tray_menu = SystemTrayMenu::new()
        .add_item(toggle_key)
        .add_item(toggle_sound)
        .add_item(settings)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    SystemTray::new().with_menu(tray_menu)
}

pub fn on_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => app.exit(0),
            "toggle-key" => toggle_key(app),
            "toggle-sound" => toggle_sound(app),
            "settings" => show_settings_window(app),
            _ => {}
        },
        _ => {}
    }
}

fn toggle_key(app: &AppHandle) {
    let window = app.get_window("main").unwrap();
    if window.is_visible().unwrap() {
        window.hide().unwrap();
    } else {
        window.show().unwrap();
    }
}

fn toggle_sound(app: &AppHandle) {
    let state = app.state::<AppState>();
    let mut sound_enabled = state.sound_enabled.lock().unwrap();
    *sound_enabled = !*sound_enabled;
}

fn show_settings_window(app: &AppHandle) {
    let window = app.get_window("settings");
    match window {
        Some(win) => {
            if win.is_visible().unwrap() {
                win.hide().unwrap();
            } else {
                win.set_focus().unwrap();
                win.show().unwrap();
            }
        }
        None => println!("Settings window not available"),
    }
}
