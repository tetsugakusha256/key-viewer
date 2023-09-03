use evdev::Key;
use std::{
    collections::{HashMap, VecDeque},
    fmt,
};

const MAX_KEYS: usize = 5;
const MAX_MOD_KEYS: usize = 5;
const MAX_KEYS_CHAIN: usize = 4;
// modkeys : shift_l 50 shift_r 62 ctrl_l 37 ctr_r 105 Superl 133 alt_l 64 iso3 108 iso5 107
const KEY_LEFTALT: u16 = Key::KEY_LEFTALT.code();
const KEY_LEFTCTRL: u16 = Key::KEY_LEFTCTRL.code();
const KEY_LEFTMETA: u16 = Key::KEY_LEFTMETA.code();
const KEY_LEFTSHIFT: u16 = Key::KEY_LEFTSHIFT.code();
const KEY_ISO3: u16 = Key::KEY_RIGHTALT.code();
const KEY_RIGHTCTRL: u16 = Key::KEY_RIGHTCTRL.code();
const KEY_ISO5: u16 = Key::KEY_PRINT.code();
const KEY_RIGHTSHIFT: u16 = Key::KEY_RIGHTSHIFT.code();

fn mod_to_bitmask(mod_key: &u16) -> u16 {
    match *mod_key {
        KEY_LEFTALT => 1,
        KEY_LEFTSHIFT => 2,
        KEY_LEFTMETA => 4,
        KEY_LEFTCTRL => 8,
        KEY_ISO3 => 16,
        KEY_ISO5 => 32,
        KEY_RIGHTSHIFT => 64,
        KEY_RIGHTCTRL => 128,
        _ => 0,
    }
}

#[allow(dead_code)]
pub struct KeysManager {
    current_keys: CurrentKeys,
    /// <key_code,(mode_bitmask,count)>
    keys_pressed_stats: HashMap<u16, HashMap<u16, u32>>,
    keys_history: VecDeque<u16>,
}
impl fmt::Display for KeysManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // writeln!(f, "Keypressed array :")?;
        // writeln!(f, "{}", &self.current_keys.pressed_keys)?;
        // writeln!(f, "Mod keys array :")?;
        // writeln!(f, "{}", &self.current_keys.pressed_mod_keys)?;
        writeln!(f, "Alt clicked : {}", &self.key_stats(&56, &1))?;
        writeln!(f, "G(d) clicked : {}", &self.key_stats(&34, &0))?;
        writeln!(f, "G(d) w/ alt : {}", &self.key_stats(&34, &1))?;
        writeln!(f, "End")
    }
}
impl KeysManager {
    pub fn new() -> KeysManager {
        KeysManager {
            current_keys: CurrentKeys::new(),
            keys_pressed_stats: HashMap::new(),
            keys_history: VecDeque::with_capacity(MAX_KEYS_CHAIN),
        }
    }
    /// Update the state with the new key event
    pub fn receive_keyevent(&mut self, key_code: &u16, key_value: &i32) {
        // Update the arrays
        let key_update_result = self.current_keys.receive_keyevent(&key_code, &key_value);
        // Update the statistics
        self.update_keycount_hashmap(&key_update_result);
    }
    fn push_key_history(&mut self, key_code: &u16) {
        if self.keys_history.len() == MAX_KEYS_CHAIN {
            self.keys_history.pop_front();
        }
        self.keys_history.push_back(key_code.clone());
    }
    fn key_stats(&self, key_code: &u16, mod_mask: &u16) -> u32{
        match self.keys_pressed_stats.get(key_code){
            Some(mod_key_hashmap) => match mod_key_hashmap.get(mod_mask){
                Some(pressed_amount) => pressed_amount.clone(),
                None => 0,
            },
            None => 0,
        }

    }
    fn update_keycount_hashmap(&mut self, key_update_result: &Option<LogKeyEvent>){
        match key_update_result {
            Some(x) => match x {
                LogKeyEvent::KeyPressed(key_code) => {
                    let mod_mask = self.current_keys.get_mod_keys_mask();
                    self.push_key_history(key_code);
                    self.keys_pressed_stats
                        .entry(*key_code)
                        .and_modify(|val| {
                            val.entry(mod_mask)
                                .and_modify(|_val| *_val = *_val + 1)
                                .or_insert(1);
                        })
                        .or_insert_with(|| {
                            let mut x = HashMap::new();
                            x.insert(mod_mask, 1);
                            x
                        });
                }
                LogKeyEvent::KeyReleased(_) => (),
                LogKeyEvent::KeyHold(_) => (),
                LogKeyEvent::KeyStayHold(_) => (),
            },
            None => (),
        }
    }
}

#[allow(dead_code)]
pub enum LogKeyEvent {
    KeyPressed(u16),
    KeyReleased(u16),
    KeyHold(u16),
    KeyStayHold(u16),
}
struct KeysList([(u16, i32); MAX_KEYS]);
impl fmt::Display for KeysList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "KeysPressed array :")?;
        for (index, (code, value)) in self.iter().enumerate() {
            writeln!(f, "Row : {index}\t Code : {} \t Value :{}", code, value)?
        }
        writeln!(f, "End")
    }
}
impl KeysList {
    fn iter(&self) -> std::slice::Iter<(u16, i32)> {
        self.0.iter()
    }
    fn iter_mut(&mut self) -> std::slice::IterMut<(u16, i32)> {
        self.0.iter_mut()
    }
}
pub struct CurrentKeys {
    pressed_keys: KeysList,
    pressed_mod_keys: KeysList,
}
#[allow(dead_code)]
impl CurrentKeys {
    pub fn new() -> CurrentKeys {
        let mut my_array: [(u16, i32); MAX_KEYS] = [(0, 0); MAX_KEYS]; // Initialize with default values
        let mut my_mod_array: [(u16, i32); MAX_MOD_KEYS] = [(0, 0); MAX_MOD_KEYS]; // Initialize with default values
        for i in 0..MAX_KEYS {
            my_array[i] = (0, 0);
            my_mod_array[i] = (0, 0);
        }
        return CurrentKeys {
            pressed_keys: (KeysList(my_mod_array)),
            pressed_mod_keys: (KeysList(my_array)),
        };
    }
    pub fn get_current_keys_pressed() {}
    pub fn clear(&mut self) {
        for e in self.pressed_keys.iter_mut() {
            *e = (0u16, 0i32);
        }
    }
    fn is_mod_key(key_code: &u16) -> bool {
        match *key_code {
            KEY_LEFTALT | KEY_LEFTCTRL | KEY_LEFTMETA | KEY_LEFTSHIFT | KEY_ISO3
            | KEY_RIGHTCTRL | KEY_ISO5 | KEY_RIGHTSHIFT => true,
            _ => false,
        }
    }
    pub fn get_mod_keys_mask(&self) -> u16 {
        let mut mask = 0;
        for (code, value) in self.pressed_mod_keys.iter() {
            if *value != 0 && *code != 0 {
                mask += mod_to_bitmask(code)
            }
        }
        mask
    }
    /// Update the state with the new key event
    pub fn receive_keyevent(&mut self, key_code: &u16, key_value: &i32) -> Option<LogKeyEvent> {
        // Update the arrays
        let key_update_result = if CurrentKeys::is_mod_key(key_code) {
            CurrentKeys::update_keystate(&mut self.pressed_mod_keys, &key_code, &key_value)
        } else {
            CurrentKeys::update_keystate(&mut self.pressed_keys, &key_code, &key_value)
        };
        key_update_result
    }
    fn update_keystate(
        key_list: &mut KeysList,
        key_code: &u16,
        key_value: &i32,
    ) -> Option<LogKeyEvent> {
        match key_value {
            0 | 1 | 2 => {
                // Check if we already have the key logged
                for (code, value) in key_list.iter_mut() {
                    if *code == *key_code {
                        return inner_update_keystate(value, code, &key_code, &key_value);
                    }
                }
                // if no same key already logged, add it at the first empty spot
                for (code, value) in key_list.iter_mut() {
                    if *value == 0 {
                        return inner_update_keystate(value, code, &key_code, &key_value);
                    }
                }
            }
            _ => return None,
        }
        fn inner_update_keystate(
            key_value: &mut i32,
            key_code: &mut u16,
            new_key_code: &u16,
            new_key_value: &i32,
        ) -> Option<LogKeyEvent> {
            let new_key_code = if *new_key_value == 0 {
                0
            } else {
                *new_key_code
            };
            match key_value {
                0 => {
                    *key_value = *new_key_value;
                    *key_code = new_key_code;
                    return Some(LogKeyEvent::KeyPressed(new_key_code));
                }
                1 => {
                    *key_value = *new_key_value;
                    *key_code = new_key_code;
                    return Some(LogKeyEvent::KeyReleased(new_key_code));
                }
                2 => {
                    if *key_value == *new_key_value {
                        return Some(LogKeyEvent::KeyStayHold(new_key_code));
                    }
                    *key_value = *new_key_value;
                    *key_code = new_key_code;
                    return Some(LogKeyEvent::KeyHold(new_key_code));
                }
                _ => {
                    return None;
                }
            }
        }
        return None;
    }
}
