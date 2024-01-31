use std::{fs::File, io::Read};

use crate::{key_manager::KeysStats, error_type::Errors};

pub struct Date(String);

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
