extern crate evdev;

use evdev::{Device, EventType, Key};
use key_capture::{key_manager::key_types::EvdevKeyCode, logger};

fn main() {
    let path = "/dev/input/event16"; // Replace X with the appropriate event number
    let mut device = Device::open(path).expect("Failed to create device");

    let mut logger = logger::Logger::new(
        "/home/anon/Documents/Code/RustLearning/key_capture/output_deamon.txt".to_string(),
    )
    .unwrap();

    loop {
        if let Ok(events) = device.fetch_events() {
            for event in events {
                if event.event_type() == EventType::KEY {
                    match event.value() {
                        1 => {
                            println!("Press");
                            println!("event : {}", event.code())
                        }
                        _ => {}
                    }
                    if event.code() == Key::KEY_END.code() {
                        let _ = logger.log_key_data();
                    }
                    logger.send_key(&EvdevKeyCode(event.code()), &event.value());
                    if event.code() == Key::KEY_0.code() {}
                }
            }
        }
    }
}
