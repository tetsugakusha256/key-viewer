use std::fs::File;

use crate::{
    error_type::Errors,
    key_manager::{key_types::EvdevKeyCode, KeysStats},
    key_manager::{
        evdev_x11_tools::EvdevX11Converter, keystate_memory::KeystateMemory,
    }, logger::OnDiskData,
};

/// Owns a keysmanager and a converter and manage the logging
#[allow(dead_code)]
pub struct KeyReader {
    current_keys: KeystateMemory,
    pub keys_stats: KeysStats,
    evdev_converter: EvdevX11Converter,
}
#[allow(dead_code)]
impl KeyReader {
    pub fn new() -> Result<KeyReader, Errors> {
        Ok(KeyReader {
            current_keys: KeystateMemory::new(),
            keys_stats: KeysStats::new(),
            evdev_converter: EvdevX11Converter::new("cuco"),
        })
    }
    pub fn new_from_file(path: String) -> Result<KeyReader, Errors> {
        let mut file = File::open(&path)?;
        let keys_stats = OnDiskData::new_from_disk(&mut file)?.keys_stats;
        Ok(KeyReader {
            current_keys: KeystateMemory::new(),
            keys_stats,
            evdev_converter: EvdevX11Converter::new("cuco"),
        })
    }
    pub fn send_key(&mut self, code: &EvdevKeyCode, value: &i32) -> () {
        self.current_keys.receive_keyevent(&code, &value);
    }
}
