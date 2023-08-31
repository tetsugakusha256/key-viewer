extern crate evdev;

use std::io::{self, Read, Write};
use evdev::*;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();

    let mut stdin_lock = stdin.lock();
    let mut stdout_lock = stdout.lock();

    let mut buffer = [0u8; std::mem::size_of::<InputEvent>()];

    while stdin_lock.read_exact(&mut buffer).is_ok() {
        let event: InputEvent = unsafe { std::mem::transmute(buffer) };

        if event.event_type() == EventType::KEY && event.code() == Key::KEY_X.code() {
                let new_event = InputEvent::new(EventType::KEY, Key::KEY_Y.code(), event.value()); 
            let new_buffer: [u8; std::mem::size_of::<InputEvent>()] = unsafe { std::mem::transmute(new_event) };
            stdout_lock.write_all(&new_buffer)?;
        } else {
            stdout_lock.write_all(&buffer)?;
        }
    }

    Ok(())
}
