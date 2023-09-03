use crate::{error_type::Errors, key_manager::{KeysManager}};
use std::{fs::File, io::Write};

#[allow(dead_code)]
pub struct Logger {
    file: File,
    keys_manager: KeysManager,
}
#[allow(dead_code)]
impl Logger {
    pub fn new(path: &str) -> Result<Logger, Errors> {
        let file = File::create(path)?;
        let keys_pressed = KeysManager::new();
        Ok(Logger { file, keys_manager: keys_pressed })
    }
    pub fn send_key(&mut self, code: &u16, value: &i32) -> () {
        // let _ = self.write_in_log(format!("Code: {code}\t\tValue: {value}").as_str());
        self.keys_manager.receive_keyevent(&code, &value);
        let _ = self.write_in_log(&format!("Code : {code}\t Value : {value}"));
        let _ = self.write_in_log(&self.keys_manager);
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
