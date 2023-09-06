use crate::{
    error_type::Errors,
    key_manager::{evdev_x11_tools::EvdevX11Converter, KeysManager},
    key_manager::{
        key_types::{EvdevKeyCode, EvdevModMask},
    },
};
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
};

/// Owns a keysmanager and a converter and manage the logging
#[allow(dead_code)]
pub struct Logger {
    file: File,
    path: String,
    keys_manager: KeysManager,
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
    pub fn new_from_file(path: String) -> Result<Logger, Errors> {
        println!("before");
        let mut file = File::open(&path)?;
        let mut keys_pressed = KeysManager::new();
        let new_x = Logger::load_from_disk(&mut file)?;
        keys_pressed.set_keys_pressed_stats(new_x);
        Ok(Logger {
            file,
            keys_manager: keys_pressed,
            path,
            evdev_converter: EvdevX11Converter::new("cuco"),
        })
    }
    pub fn send_key(&mut self, code: &EvdevKeyCode, value: &i32) -> () {
        self.keys_manager.receive_keyevent(&code, &value);
    }
    pub fn print_to_file(&mut self) -> Result<(), Errors> {
        self.file = File::create(&self.path)?;
        self.save_to_disk()
        // self.write_in_log(&self.nice_string())
    }
    // TODO:
    // Well formatted string with all recorded key info
    // Maybe make a list of all keys to loop through
    // This get the hashmap and print everything in it
    // Need another function that get all key on layout and retrieve the clicks
    pub fn nice_string(&self) -> String {
        let mut text = String::from("");
        if let Some(keystats_vec) = self.keys_manager.all_keys_stats_vec() {
            for (code, clicks, map) in keystats_vec {
                text = text + &map.to_string();
                text = text
                    + self.evdev_converter.get_x11_char(&code, &map).as_str()
                    + "\t clicked : "
                    + clicks.to_string().as_str()
                    + "\n";
            }
        }
        text
    }

    /// String of all keys clicked on a given mod_mask
    pub fn nice_string_mask(&self, mod_mask: &EvdevModMask) -> String {
        let mut text = String::from("");
        text += &mod_mask.to_string();
        let keystats_vec = self.keys_manager.hashmap_mod_keys(mod_mask);
        for (code, clicks) in keystats_vec {
            text = text
                + self.evdev_converter.get_x11_char(&code, &mod_mask).as_str()
                + "\t clicked : "
                + clicks.to_string().as_str()
                + "\n";
        }

        text
    }
    /// Save data to disk
    fn save_to_disk(&self) -> Result<(), Errors> {
        let serialized = serde_json::to_string(&self.keys_manager.get_keys_pressed_stats())?;
        let _ = self.write_in_log(&serialized);
        Ok(())
    }
    /// Load data from disk
    fn load_from_disk(
        file: &mut File,
    ) -> Result<HashMap<EvdevKeyCode, HashMap<EvdevModMask, u32>>, Errors> {
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let deserialized: HashMap<EvdevKeyCode, HashMap<EvdevModMask, u32>> =
            serde_json::from_str(&content)?;
        Ok(deserialized)
    }

    fn write_in_log<T: std::fmt::Display>(&self, text: &T) -> Result<(), Errors> {
        writeln!(&self.file, "{}", text)?;
        Ok(())
    }
    // fn write_in_file(&self, text: &str) -> Result<(), Errors> {
    //     writeln!(&self.file, "{}", text)?;
    //     Ok(())
    // }
}
