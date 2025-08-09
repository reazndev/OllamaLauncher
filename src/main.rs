use rdev::{listen, Event, EventType, Key};
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref ALT_DOWN: Mutex<bool> = Mutex::new(false);
}

fn main() -> Result<(), rdev::ListenError> {
    println!("Listening for Alt+Space..."); // remove later
    listen(callback)
}

fn callback(event: Event) {
    match event.event_type {
        EventType::KeyPress(Key::Alt) => {
            *ALT_DOWN.lock().unwrap() = true;
        }
        EventType::KeyRelease(Key::Alt) => {
            *ALT_DOWN.lock().unwrap() = false;
        }
        EventType::KeyPress(Key::Space) => {
            if *ALT_DOWN.lock().unwrap() {
                println!("Alt + Space detected!"); // remove later
                // TODO: trigger popup here

            }
        }
        _ => {}
    }
}
