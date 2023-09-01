extern crate evdev;

use evdev::*;
use std::io::{self, Read, Write};
mod error_type;
mod logger;

fn main() -> Result<(), error_type::Errors> {
    let stdin = io::stdin();
    let stdout = io::stdout();

    let mut stdin_lock = stdin.lock();
    let mut stdout_lock = stdout.lock();

    let mut event_buffer = [0u8; std::mem::size_of::<InputEvent>()];
    let mut buffer_offset = 0;
    let logger =
        logger::Logger::new("/home/anon/Documents/Code/RustLearning/key_capture/output.txt")
            .unwrap();

    loop {
        let mut byte = [0u8; 1];
        if stdin_lock.read_exact(&mut byte).is_err() {
            break;
        }

        event_buffer[buffer_offset] = byte[0];
        buffer_offset += 1;

        if buffer_offset == std::mem::size_of::<InputEvent>() {
            let event: InputEvent = unsafe { std::mem::transmute(event_buffer) };
            let _ = logger.write_in_file("test");

            if event.event_type() == EventType::KEY {
                let new_event = InputEvent::new(EventType::KEY, event.code(), event.value());
                let new_buffer: [u8; std::mem::size_of::<InputEvent>()] =
                    unsafe { std::mem::transmute(new_event) };
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

#[cfg(test)]
mod tests {
    #[test]
    fn empty() {
        assert_eq!(2, 2);
    }
    #[test]
    fn empty_input() {
        assert_eq!(1, 1);
    }
}
