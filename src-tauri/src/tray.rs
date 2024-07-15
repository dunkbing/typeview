use tauri::{
    image::Image,
    menu::{CheckMenuItemBuilder, MenuBuilder, MenuItemBuilder},
    tray::TrayIconBuilder,
    AppHandle, Manager,
};
use tauri_plugin_store::StoreBuilder;

use crate::AppState;

pub fn build(app: &AppHandle) {
    let tray_menu = MenuBuilder::new(app)
        .items(&[
            &MenuItemBuilder::with_id("quit", "Quit")
                .build(app)
                .expect(""),
            &CheckMenuItemBuilder::with_id("toggle-key", "Toggle Key")
                .checked(get_tray_setting(app, "toggle-key".to_string()))
                .build(app)
                .expect(""),
            &CheckMenuItemBuilder::with_id("toggle-sound", "Toggle Sound")
                .checked(get_tray_setting(app, "toggle-sound".to_string()))
                .build(app)
                .expect(""),
            &MenuItemBuilder::with_id("settings", "Settings")
                .build(app)
                .expect(""),
        ])
        .build()
        .expect("Failed to build tray menu");

    let tray = TrayIconBuilder::with_id("tray")
        .icon(Image::from_bytes(include_bytes!("../icons/icon.png")).expect(""))
        .menu(&tray_menu)
        .on_menu_event(move |app, event| match event.id().as_ref() {
            "quit" => {
                app.exit(0);
            }
            "toggle-key" => {
                update_tray_setting(app, "toggle-key".to_string());
                toggle_key(&app);
            }
            "toggle-sound" => {
                update_tray_setting(app, "toggle-sound".to_string());
                toggle_sound(&app);
            }
            "settings" => {
                show_settings_window(&app);
            }
            _ => (),
        });
    tray.build(app).expect("Failed to build tray");
}

pub fn get_tray_setting(app: &AppHandle, key: String) -> bool {
    let mut store = StoreBuilder::new("app_data.bin").build(app.clone());
    store.load().unwrap_or_default();

    let setting_value = store
        .get(key.clone())
        .unwrap_or(&serde_json::Value::Bool(true))
        .as_bool()
        .unwrap();

    setting_value
}

fn update_tray_setting(app: &AppHandle, key: String) {
    let mut store = StoreBuilder::new("app_data.bin").build(app.clone());
    store.load().unwrap_or_default();

    // Get current value or true if not found
    let setting_value = get_tray_setting(app, key.clone());

    match setting_value {
        true => {
            store.insert(key.clone(), false.into()).unwrap();
        }
        false => {
            store.insert(key.clone(), true.into()).unwrap();
        }
    }
    store.save().expect("Failed to save store");

    // log updated value
    let updated_value = get_tray_setting(app, key.clone());

    println!("{}: {}", key, updated_value);
}

fn toggle_key(app: &AppHandle) {
    let window = app.get_webview_window("main").unwrap();
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
    let window = app.get_webview_window("settings");
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
