extern crate evdev;

use std::io::{self, Read, Write};
use evdev::*;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();

    let mut stdin_lock = stdin.lock();
    let mut stdout_lock = stdout.lock();

    let mut event_buffer = [0u8; std::mem::size_of::<InputEvent>()];
    let mut buffer_offset = 0;

    loop {
        let mut byte = [0u8; 1];
        if stdin_lock.read_exact(&mut byte).is_err() {
            break;
        }

        event_buffer[buffer_offset] = byte[0];
        buffer_offset += 1;

        if buffer_offset == std::mem::size_of::<InputEvent>() {
            let event: InputEvent = unsafe { std::mem::transmute(event_buffer) };

            if event.event_type() == EventType::KEY && event.code() == Key::KEY_X.code() {
                let new_event = InputEvent::new(EventType::KEY, Key::KEY_Y.code(), event.value());
                let new_buffer: [u8; std::mem::size_of::<InputEvent>()] = unsafe {
                    std::mem::transmute(new_event)
                };
                stdout_lock.write_all(&new_buffer)?;
                stdout_lock.flush()?; // Flush the output immediately
            } else {
                stdout_lock.write_all(&event_buffer)?;
                stdout_lock.flush()?; // Flush the output immediately
            }

            buffer_offset = 0;
        }
    }

    Ok(())
}
