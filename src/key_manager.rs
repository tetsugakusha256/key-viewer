pub mod keystate_memory;
pub mod evdev_x11_tools;

use self::keystate_memory::{KeystateMemory, LogKeyEvent};
use std::{
    collections::{HashMap, VecDeque},
    fmt,
};

// TODO: use custom type to improve type safety and comprehension
// and implement a function keycode + keymask -> char using layout info
// and a simpler version keycode -> to named key using layout
#[allow(dead_code)]
struct KeyCode(u16);
#[allow(dead_code)]
struct ModMask(u16);
const MAX_KEYS_CHAIN: usize = 4;

#[allow(dead_code)]
pub struct KeysManager {
    current_keys: KeystateMemory,
    /// <key_code,(mode_bitmask,count)>
    keys_pressed_stats: HashMap<u16, HashMap<u16, u32>>,
    keys_history: VecDeque<u16>,
}
impl fmt::Display for KeysManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted_string = format!("{:#?}", &self.keystats_vec() );
        writeln!(f, "Stats: {}", formatted_string)
    }
}
#[allow(dead_code)]
impl KeysManager {
    pub fn new() -> KeysManager {
        KeysManager {
            current_keys: KeystateMemory::new(),
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
    pub fn get_keys_pressed_stats(&self) -> &HashMap<u16, HashMap<u16, u32>>{
        return &self.keys_pressed_stats;
    }
    /// (key_code, number of clics, mod_mask)
    pub fn keystats_vec(&self) -> Option<Vec<(u16, u32, u16)>> {
        let mut keystate_list = Vec::new();
        for (key_code, mod_key_hashmap) in self.keys_pressed_stats.iter() {
            for (mod_mask, clicks) in mod_key_hashmap.iter() {
                keystate_list.push((key_code.clone(), clicks.clone(), mod_mask.clone()));
            }
        }
        if keystate_list.is_empty() {
            None
        } else {
            Some(keystate_list)
        }
    }
    // TODO:
    // Get total clicks of a key with and without taking mod into account
    
    // TODO:
    // Sort by number of clics
    
    
    fn push_key_history(&mut self, key_code: &u16) {
        if self.keys_history.len() == MAX_KEYS_CHAIN {
            self.keys_history.pop_front();
        }
        self.keys_history.push_back(key_code.clone());
    }
    /// Single key stats with the given mod_mask
    fn key_stats(&self, key_code: &u16, mod_mask: &u16) -> u32 {
        match self.keys_pressed_stats.get(key_code) {
            Some(mod_key_hashmap) => match mod_key_hashmap.get(mod_mask) {
                Some(pressed_amount) => pressed_amount.clone(),
                None => 0,
            },
            None => 0,
        }
    }
    fn update_keycount_hashmap(&mut self, key_update_result: &Option<LogKeyEvent>) {
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
