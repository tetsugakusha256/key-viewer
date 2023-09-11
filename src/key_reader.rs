use crate::{
    error_type::Errors,
    key_manager::key_types::{EvdevKeyCode, EvdevModMask},
    key_manager::{
        evdev_x11_tools::EvdevX11Converter, keystate_memory::KeystateMemory, KeysManager,
    },
};
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
};

/// Owns a keysmanager and a converter and manage the logging
#[allow(dead_code)]
pub struct KeyReader {
    current_keys: KeystateMemory,
    evdev_converter: EvdevX11Converter,
}
#[allow(dead_code)]
impl KeyReader {
    pub fn new(path: String) -> Result<KeyReader, Errors> {
        Ok(KeyReader {
            current_keys: KeystateMemory::new(),
            evdev_converter: EvdevX11Converter::new("cuco"),
        })
    }
    pub fn send_key(&mut self, code: &EvdevKeyCode, value: &i32) -> () {
        self.current_keys.receive_keyevent(&code, &value);
    }
    //TODO: setup async listening to the keys and sending them to the KeystateMemory
}
