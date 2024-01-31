use chrono::{Local, NaiveDate};

use crate::{
    error_type::Errors,
    key_manager::{evdev_x11_tools::EvdevX11Converter, KeysStatsManager},
    key_manager::{
        key_types::{EvdevKeyCode, EvdevModMask},
    },
};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
};

use super::log_types::OnDiskData;

/// Owns a keysmanager and a converter and manage the logging
/// Manages a single day at a time of logs, able to add key one by one
/// To be used for logging
#[allow(dead_code)]
pub struct Logger {
    file: Option<File>,
    path: String,
    date: String,
    keys_manager: KeysStatsManager,
    evdev_converter: EvdevX11Converter,
}
#[allow(dead_code)]
impl Logger {
    pub fn new() -> Result<Logger, Errors> {
        // Format the date and time as a string
        let date = Local::now().format("%Y-%m-%d").to_string();
        let path = "/home/anon/Documents/Code/Key_capture/deamon_logging/".to_string()
            + &date
            + "-log.txt";
        let mut keys_manager = KeysStatsManager::new();

        // Try to open the file
        Ok(Logger {
            file: None,
            date: date.to_string(),
            keys_manager,
            path,
            evdev_converter: EvdevX11Converter::new("cuco"),
        })
    }
    pub fn send_key(&mut self, code: &EvdevKeyCode, value: &i32) -> () {
        self.keys_manager.receive_keyevent(&code, &value);
    }

    pub fn log_key_data(&mut self) -> Result<(), Errors> {
        self.save_to_disk()
    }

    pub fn nice_string(&self) -> String {
        let mut text = String::from("");
        if let Some(keystats_vec) = self.keys_manager.all_keys_stats_vec() {
            for (code, clicks, map) in keystats_vec {
                text = text + &map.to_string();
                text = text
                    + self.evdev_converter.get_key_char(&code, &map).as_str()
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
                + self.evdev_converter.get_key_char(&code, &mod_mask).as_str()
                + " \t \t "
                + clicks.to_string().as_str()
                + "\n";
        }

        text
    }
    /// Save data to disk with detection of new day
    fn save_to_disk(&mut self) -> Result<(), Errors> {

        let data_to_save = OnDiskData::new(self.keys_manager.keys_stats.clone());
        let serialized = serde_json::to_string(&data_to_save)?;
        self.file = Some(File::create(&self.path)?);
        writeln!(self.file.as_ref().unwrap(), "{}", &serialized)?;

        let today_date = Local::now().format("%Y-%m-%d").to_string();
        // If the day changed, start a new file with new data for the next time
        if today_date != self.date {
            self.path = "/home/anon/Documents/Code/Key_capture/deamon_logging/".to_string()
                + &today_date
                + "-log.txt";
            self.date = today_date;
            self.keys_manager = KeysStatsManager::new();
        }
        Ok(())
    }
}
