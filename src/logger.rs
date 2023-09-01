use crate::error_type::Errors;
use std::{fs::File, io::Write};

pub struct Logger {
    file: File,
}
impl Logger {
    pub fn new(path: &str) -> Result<Logger, Errors> {
        let file = match File::create(path) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Error creating file: {}", err);
                return Err(Errors::ErrorReadingFile);
            }
        };
        return Ok(Logger { file });
    }
    pub fn write_in_file(&self, text: &str) -> Result<(), Errors> {
        writeln!(&self.file, "{}", text)?;
        Ok(())
    }
}
