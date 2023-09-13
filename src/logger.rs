use crate::{
    error_type::Errors,
    key_manager::{evdev_x11_tools::EvdevX11Converter, KeysManager},
    key_manager::{
        key_types::{EvdevKeyCode, EvdevModMask},
        KeysStats,
    },
};
use std::{
    fs::File,
    io::{Read, Write},
};

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct OnDiskData {
    pub keys_stats: KeysStats,
}
impl OnDiskData {
    pub fn new(keys_stats: KeysStats) -> OnDiskData {
        OnDiskData { keys_stats }
    }
    /// Load data from disk
    pub fn new_from_disk(file: &mut File) -> Result<OnDiskData, Errors> {
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let deserialized: OnDiskData = serde_json::from_str(&content)?;
        Ok(deserialized)
    }
}
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
        let mut file = File::open(&path)?;
        let keys_pressed = KeysManager::new();
        let res = OnDiskData::new_from_disk(&mut file);
        match res {
            Ok(on_disk_data) => {
                let mut logger = Logger {
                    file,
                    keys_manager: keys_pressed,
                    path: path.clone(),
                    evdev_converter: EvdevX11Converter::new("cuco"),
                };
                logger.keys_manager.keys_stats = on_disk_data.keys_stats;
                return Ok(logger);
            }
            Err(_) => return Logger::new(path),
        }
    }

    pub fn send_key(&mut self, code: &EvdevKeyCode, value: &i32) -> () {
        self.keys_manager.receive_keyevent(&code, &value);
    }

    pub fn log_key_data(&mut self) -> Result<(), Errors> {
        self.file = File::create(&self.path)?;
        self.save_to_disk()
    }

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
    pub fn keys_clicked_before_key(&self, second_key: &EvdevKeyCode) -> Vec<(EvdevKeyCode, u32)> {
        self.keys_manager.keys_clicked_before_key(second_key)
    }
    pub fn keys_clicked_after_key(&self, first_key: &EvdevKeyCode) -> Vec<(EvdevKeyCode, u32)> {
        self.keys_manager.keys_clicked_after_key(first_key)
    }
    pub fn max_clicked_keys_all_layer(&self) -> Vec<(EvdevKeyCode, u32)> {
        self.keys_manager.max_clicked_keys_all_layer()
    }

    pub fn max_clicked_keys(&self, mod_mask: &EvdevModMask) -> Vec<(EvdevKeyCode, u32)> {
        self.keys_manager.max_clicked_keys(mod_mask)
    }
    pub fn clicks(&self, key_code: &EvdevKeyCode, mod_mask: &EvdevModMask) -> u32 {
        self.keys_manager.clicks(key_code, mod_mask)
    }

    pub fn all_clicks(&self, key_code: &EvdevKeyCode) -> u32 {
        self.keys_manager.all_clicks(key_code)
    }
    /// String of all keys clicked on a given mod_mask
    pub fn nice_string_mask(&self, mod_mask: &EvdevModMask) -> String {
        let mut text = String::from("");
        text += &mod_mask.to_string();
        let keystats_vec = self.keys_manager.hashmap_mod_keys(mod_mask, &true);
        for (code, clicks) in keystats_vec {
            text = text
                + self.evdev_converter.get_x11_char(&code, &mod_mask).as_str()
                + " \t \t "
                + clicks.to_string().as_str()
                + "\n";
        }

        text
    }
    /// Save data to disk
    fn save_to_disk(&self) -> Result<(), Errors> {
        //TODO: how to remove clone here?
        let data_to_save = OnDiskData::new(self.keys_manager.keys_stats.clone());
        let serialized = serde_json::to_string(&data_to_save)?;
        let _ = self.write_in_log(&serialized);
        Ok(())
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
