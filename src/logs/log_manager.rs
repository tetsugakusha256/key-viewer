use std::{collections::HashMap, fs::File};

use crate::{
    error_type::Errors,
    key_manager::{
        evdev_x11_tools::EvdevX11Converter,
        key_types::{EvdevKeyCode, EvdevModMask},
        KeysStatsManager,
    },
};

use super::log_types::Date;

pub struct LogManager {
    keys_manager: KeysStatsManager,
    evdev_converter: EvdevX11Converter,
    // Store files as they are getting opened
    files: HashMap<Date, File>,
}
impl LogManager {
    pub fn new() -> Result<LogManager, Errors> {
        let keys_pressed = KeysStatsManager::new();
        Ok(LogManager {
            keys_manager: keys_pressed,
            evdev_converter: EvdevX11Converter::new("cuco"),
            files: HashMap::new(),
        })
    }
    // Load file of the given date
    fn load_file(&mut self, _date: &Date) -> &File{
        todo!()
    }
    fn set_keys_manager_date(&mut self, date: &Date) -> &File{
        let _file = self.load_file(date);
        todo!()
    }

    pub fn date_with_log_list(&self) -> Vec<String> {
        todo!()
    }
    pub fn all_clicks_all_time(&self, _key_code: &EvdevKeyCode) -> u32 {
        todo!()
    }
    pub fn clicks_all_time(&self, _key_code: &EvdevKeyCode, _mod_mask: &EvdevModMask) -> u32 {
        todo!()
    }
    pub fn keys_clicked_before_key(
        &self,
        second_key: &EvdevKeyCode,
        _date: &Date,
    ) -> Vec<(EvdevKeyCode, u32)> {
        self.keys_manager.keys_clicked_before_key(second_key)
    }
    pub fn keys_clicked_after_key(
        &self,
        first_key: &EvdevKeyCode,
        _date: &Date,
    ) -> Vec<(EvdevKeyCode, u32)> {
        self.keys_manager.keys_clicked_after_key(first_key)
    }
    pub fn max_clicked_keys_all_layer(&self, _date: &Date) -> Vec<(EvdevKeyCode, u32)> {
        self.keys_manager.max_clicked_keys_all_layer()
    }

    pub fn max_clicked_keys(
        &self,
        mod_mask: &EvdevModMask,
        _date: &Date,
    ) -> Vec<(EvdevKeyCode, u32)> {
        self.keys_manager.max_clicked_keys(mod_mask)
    }
    pub fn clicks(&self, key_code: &EvdevKeyCode, mod_mask: &EvdevModMask, _date: &Date) -> u32 {
        self.keys_manager.clicks(key_code, mod_mask)
    }

    pub fn all_clicks(&self, key_code: &EvdevKeyCode, _date: &Date) -> u32 {
        self.keys_manager.all_clicks(key_code)
    }
    /// String of all keys clicked on a given mod_mask
    pub fn nice_string_mask(&self, mod_mask: &EvdevModMask, _date: &Date) -> String {
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
}
