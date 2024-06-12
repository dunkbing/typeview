use rdev::{listen, EventType, Key};
use rodio::{Decoder, OutputStream, Sink};
use std::io::BufReader;
use std::path::PathBuf;
use std::thread;
use std::{fs::File, sync::Arc};
use tauri::Manager;

use crate::AppState;

fn play_sound(sound_path: String) {
    thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        let file = File::open(sound_path).unwrap();
        let source = Decoder::new(BufReader::new(file)).unwrap();

        sink.append(source);
        sink.sleep_until_end();
    });
}

pub fn start_keystroke_listener(app: &tauri::App) {
    let window = app.get_window("main").unwrap();
    let state = app.state::<AppState>().inner().clone();

    let window = Arc::new(window);
    let state = Arc::new(state);
    let sounds_dir = app
        .path_resolver()
        .resolve_resource("sounds/")
        .map(PathBuf::into_os_string)
        .and_then(|s| s.into_string().ok())
        .unwrap();

    tauri::async_runtime::spawn(async move {
        let window_clone = Arc::clone(&window);
        let state_clone = Arc::clone(&state);

        if let Err(error) = listen(move |event| match event.event_type {
            EventType::KeyPress(key) => {
                if let Some(key_str) = get_key_string(key) {
                    window_clone.emit_to("main", "KeyPress", key_str).unwrap();

                    let sound_name = state_clone.sound.lock().unwrap().clone();
                    let sound_path = format!("{}/{}", sounds_dir, sound_name);

                    if state_clone.sound_enabled.lock().unwrap().clone() {
                        play_sound(sound_path);
                    }
                }
            }
            EventType::KeyRelease(key) => {
                if let Some(key_str) = get_key_string(key) {
                    window_clone.emit_to("main", "KeyRelease", key_str).unwrap();
                }
            }
            _ => (),
        }) {
            println!("Error: {:?}", error);
        }
    });
}

fn get_key_string(key: Key) -> Option<String> {
    match key {
        Key::Alt => Some("Alt".to_string()),
        Key::AltGr => Some("AltGr".to_string()),
        Key::Backspace => Some("⌫".to_string()),
        Key::CapsLock => Some("⇬".to_string()),
        Key::ControlLeft => Some("^".to_string()),
        Key::ControlRight => Some("^".to_string()),
        Key::Delete => Some("⌦".to_string()),
        Key::DownArrow => Some("↓".to_string()),
        Key::End => Some("End".to_string()),
        Key::Escape => Some("⎋".to_string()),
        Key::F1 => Some("F1".to_string()),
        Key::F2 => Some("F2".to_string()),
        Key::F3 => Some("F3".to_string()),
        Key::F4 => Some("F4".to_string()),
        Key::F5 => Some("F5".to_string()),
        Key::F6 => Some("F6".to_string()),
        Key::F7 => Some("F7".to_string()),
        Key::F8 => Some("F8".to_string()),
        Key::F9 => Some("F9".to_string()),
        Key::F10 => Some("F10".to_string()),
        Key::F11 => Some("F11".to_string()),
        Key::F12 => Some("F12".to_string()),
        Key::Home => Some("Home".to_string()),
        Key::LeftArrow => Some("←".to_string()),
        Key::MetaLeft => Some("⌘".to_string()),
        Key::MetaRight => Some("⌘".to_string()),
        Key::PageDown => Some("PgDn".to_string()),
        Key::PageUp => Some("PgUp".to_string()),
        Key::Return => Some("⏎".to_string()),
        Key::RightArrow => Some("→".to_string()),
        Key::ShiftLeft => Some("⇧".to_string()),
        Key::ShiftRight => Some("⇧".to_string()),
        Key::Space => Some("␣".to_string()),
        Key::Tab => Some("↹".to_string()),
        Key::UpArrow => Some("↑".to_string()),
        Key::PrintScreen => Some("PrtSc".to_string()),
        Key::ScrollLock => Some("ScrollLock".to_string()),
        Key::Pause => Some("⎉".to_string()),
        Key::NumLock => Some("NumLock".to_string()),
        Key::BackQuote => Some("`".to_string()),
        Key::Num1 => Some("1".to_string()),
        Key::Num2 => Some("2".to_string()),
        Key::Num3 => Some("3".to_string()),
        Key::Num4 => Some("4".to_string()),
        Key::Num5 => Some("5".to_string()),
        Key::Num6 => Some("6".to_string()),
        Key::Num7 => Some("7".to_string()),
        Key::Num8 => Some("8".to_string()),
        Key::Num9 => Some("9".to_string()),
        Key::Num0 => Some("0".to_string()),
        Key::Minus => Some("-".to_string()),
        Key::Equal => Some("=".to_string()),
        Key::KeyQ => Some("q".to_string()),
        Key::KeyW => Some("w".to_string()),
        Key::KeyE => Some("e".to_string()),
        Key::KeyR => Some("r".to_string()),
        Key::KeyT => Some("t".to_string()),
        Key::KeyY => Some("y".to_string()),
        Key::KeyU => Some("u".to_string()),
        Key::KeyI => Some("i".to_string()),
        Key::KeyO => Some("o".to_string()),
        Key::KeyP => Some("p".to_string()),
        Key::LeftBracket => Some("[".to_string()),
        Key::RightBracket => Some("]".to_string()),
        Key::KeyA => Some("a".to_string()),
        Key::KeyS => Some("s".to_string()),
        Key::KeyD => Some("d".to_string()),
        Key::KeyF => Some("f".to_string()),
        Key::KeyG => Some("g".to_string()),
        Key::KeyH => Some("h".to_string()),
        Key::KeyJ => Some("j".to_string()),
        Key::KeyK => Some("k".to_string()),
        Key::KeyL => Some("l".to_string()),
        Key::SemiColon => Some(";".to_string()),
        Key::Quote => Some("'".to_string()),
        Key::KeyZ => Some("z".to_string()),
        Key::KeyX => Some("x".to_string()),
        Key::KeyC => Some("c".to_string()),
        Key::KeyV => Some("v".to_string()),
        Key::KeyB => Some("b".to_string()),
        Key::KeyN => Some("n".to_string()),
        Key::KeyM => Some("m".to_string()),
        Key::Comma => Some(",".to_string()),
        Key::Dot => Some(".".to_string()),
        Key::Slash => Some("/".to_string()),
        Key::Insert => Some("ins".to_string()),
        Key::KpReturn => Some("⏎".to_string()),
        Key::KpMinus => Some("−".to_string()),
        Key::KpPlus => Some("+".to_string()),
        Key::KpMultiply => Some("×".to_string()),
        Key::KpDivide => Some("÷".to_string()),
        Key::Kp0 => Some("0".to_string()),
        Key::Kp1 => Some("1".to_string()),
        Key::Kp2 => Some("2".to_string()),
        Key::Kp3 => Some("3".to_string()),
        Key::Kp4 => Some("4".to_string()),
        Key::Kp5 => Some("5".to_string()),
        Key::Kp6 => Some("6".to_string()),
        Key::Kp7 => Some("7".to_string()),
        Key::Kp8 => Some("8".to_string()),
        Key::Kp9 => Some("9".to_string()),
        Key::Function => Some("Fn".to_string()),
        Key::VolumeUp => Some("Vol+".to_string()),
        Key::VolumeDown => Some("Vol-".to_string()),
        Key::Unknown(_) => None,
        _ => None,
    }
}
