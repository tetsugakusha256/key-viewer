use crate::{ui_manager::{handler::MappableAction, types::physical_layout_type::{PhysicalLayout, PhysicalRow, KeySize}}, key_manager::key_types::EvdevKeyCode};
use std::fs;
use toml::Value;

pub struct Config {
    // Physical representation of the keyboard
    physical_layout: PhysicalLayout,
    // Keybindings
    keybindings: Vec<(MappableAction, String)>,
    // Typing/keyboard layout as in ($setxkbmap -query)
    typing_layout: String,
    stats_folder: String,
}
fn is_valid_keybinding() -> bool {
    false
}
pub fn load_config() -> Config {
    // Read the contents of your TOML file into a string
    let toml_str = fs::read_to_string("config.toml").expect("Error reading the file");

    // Parse the TOML string into a `Value` using the `toml` crate
    let toml_value: Value = toml::from_str(&toml_str).expect("Error parsing TOML");

    println!("{:?}", toml_value.get("keybindings"));
    let physical_layout = PhysicalLayout {
            rows: vec![PhysicalRow {
                keys: vec![(EvdevKeyCode(1), KeySize(2))],
            }],
        };
    let keybindings = vec![(MappableAction::from("NextTab".to_string()), "h".to_string())];
    let typing_layout = "/usr/share/x11/xkb/symbols/cuco".to_string();
    let stats_folder = "~/.local/share/keylogger".to_string();
    // Now `toml_value` contains your TOML data and you can access it as needed
    println!("{:?}", toml_value);
    Config {
        physical_layout,
        keybindings,
        typing_layout,
        stats_folder,
    }
}
