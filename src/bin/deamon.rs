extern crate evdev;

use std::time::Instant;

use chrono::Local;
use evdev::{Device, EventType, Key};
use key_capture::{
    config_manager::config_type::load_config, key_manager::key_types::EvdevKeyCode, logs::logger,
};

fn main() {
    let config = load_config();
    let path = "/dev/input/event16"; // Replace X with the appropriate event number
    let mut device = Device::open(path).expect("Failed to create device");

    let mut logger = logger::Logger::new().unwrap();

    let mut last_time = Instant::now();
    let delta_step = 10 * 60;

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
                    if last_time.elapsed().as_secs() > delta_step {
                        println!("Saving stat to disk");
                        let _ = logger.log_key_data();
                        last_time = Instant::now();
                    }
                    logger.send_key(&EvdevKeyCode(event.code()), &event.value());
                    if event.code() == Key::KEY_0.code() {}
                }
            }
        }
    }
}
