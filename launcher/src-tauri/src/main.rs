#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use rdev::{listen, Event, EventType, Key};
use std::sync::Mutex;
use lazy_static::lazy_static;
use tauri::Manager;

lazy_static! {
    static ref ALT_DOWN: Mutex<bool> = Mutex::new(false);
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();

            std::thread::spawn(move || {
                listen(move |event| {
                    match event.event_type {
                        EventType::KeyPress(Key::Alt) => {
                            *ALT_DOWN.lock().unwrap() = true;
                        }
                        EventType::KeyRelease(Key::Alt) => {
                            *ALT_DOWN.lock().unwrap() = false;
                        }
                        EventType::KeyPress(Key::Space) => {
                            if *ALT_DOWN.lock().unwrap() {
                                let _ = app_handle.emit_all("show-launcher", ());
                            }
                        }
                        _ => {}
                    }
                }).unwrap();
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}
