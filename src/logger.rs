use crate::{error_type::Errors, key_manager::KeysManager};
use std::{fs::File, io::Write};

#[allow(dead_code)]
pub struct Logger {
    file: File,
    keys_manager: KeysManager,
    path: String,
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
        })
    }
    pub fn send_key(&mut self, code: &u16, value: &i32) -> () {
        self.keys_manager.receive_keyevent(&code, &value);
    }
    pub fn print_to_file(&mut self) -> Result<(), Errors> {
        self.file = File::create(&self.path)?;
        self.write_in_log(&self.keys_manager)
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
