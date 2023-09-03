mod keystate_memory;

use self::keystate_memory::{KeystateMemory, LogKeyEvent};
use std::{
    collections::{HashMap, VecDeque},
    fmt,
};

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
    fn push_key_history(&mut self, key_code: &u16) {
        if self.keys_history.len() == MAX_KEYS_CHAIN {
            self.keys_history.pop_front();
        }
        self.keys_history.push_back(key_code.clone());
    }
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
