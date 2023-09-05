use crate::{
    error_type::Errors,
    key_manager::keystate_memory::mod_mask_to_string,
    key_manager::{evdev_x11_tools::EvdevX11Converter, KeysManager},
};
use std::{fs::File, io::Write, collections::HashMap};

/// Owns a keysmanager and a converter and manage the logging
#[allow(dead_code)]
pub struct Logger {
    file: File,
    keys_manager: KeysManager,
    path: String,
    evdev_converter: EvdevX11Converter,
}
#[allow(dead_code)]
impl Logger {
    pub fn new(path: String) -> Result<Logger, Errors> {
        let file = File::create(&path)?;
        let keys_pressed = KeysManager::new();
        Ok(Logger {
            file,
            keys_manager: keys_pressed,
            path,
            evdev_converter: EvdevX11Converter::new("cuco"),
        })
    }
    pub fn send_key(&mut self, code: &u16, value: &i32) -> () {
        self.keys_manager.receive_keyevent(&code, &value);
    }
    pub fn print_to_file(&mut self) -> Result<(), Errors> {
        self.file = File::create(&self.path)?;
        self.save_to_disk()?;
        self.write_in_log(&self.nice_string())
        // self.write_in_log(&self.keys_manager)
    }
    // TODO:
    // Well formatted string with all recorded key info
    // Maybe make a list of all keys to loop through
    // This get the hashmap and print everything in it
    // Need another function that get all key on layout and retrieve the clicks
    pub fn nice_string(&self) -> String {
        let mut text = String::from("");
        if let Some(keystats_vec) = self.keys_manager.keystats_vec() {
            for (code, clicks, map) in keystats_vec {
                text = text + mod_mask_to_string(&map).as_str();
                text = text
                    + self.evdev_converter.convert_keycode(&code, &0).as_str()
                    + "\t clicked : "
                    + clicks.to_string().as_str()
                    + "\n";
            }
        }
        text
    }
    // TODO:
    // Save data to disk
    fn save_to_disk(&self) -> Result<(), Errors> {
        let serialized = serde_json::to_string(&self.keys_manager.get_keys_pressed_stats())?;
        let _ = self.write_in_log(&serialized);
        Ok(())
    }
    // TODO:
    // Load data from disk
    fn load_from_disk(&self) -> Result<(), Errors> {
        let serialized = serde_json::to_string(&self.keys_manager.get_keys_pressed_stats())?;
        let deserialized: HashMap<u16, HashMap<u16, u32>> = serde_json::from_str(&serialized).unwrap();
        // Prints deserialized = Point { x: 1, y: 2 }
        println!("deserialized = {:?}", deserialized);
        Ok(())
    }

    fn write_in_log<T: std::fmt::Display>(&self, text: &T) -> Result<(), Errors> {
        writeln!(&self.file, "{}", text)?;
        Ok(())
    }
    fn write_in_file(&self, text: &str) -> Result<(), Errors> {
        writeln!(&self.file, "{}", text)?;
        Ok(())
    }
}
