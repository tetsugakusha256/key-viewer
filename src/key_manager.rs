pub mod evdev_x11_tools;
pub mod key_types;
pub mod keystate_memory;

use self::key_types::{EvdevKeyCode, EvdevModMask};
use self::keystate_memory::{KeystateMemory, LogKeyEvent};
use evdev_x11_tools::EvdevX11Converter;
use std::{
    collections::{HashMap, VecDeque},
    fmt,
};

const MAX_KEYS_CHAIN: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
/// X11 agnostic, only works with evdev value
pub struct KeysManager {
    current_keys: KeystateMemory,
    /// <key_code,(mode_bitmask,count)>
    keys_pressed_stats: HashMap<EvdevKeyCode, HashMap<EvdevModMask, u32>>,
    keys_history: VecDeque<EvdevKeyCode>,
}
impl fmt::Display for KeysManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted_string = format!("{:#?}", &self.all_keys_stats_vec());
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
    pub fn receive_keyevent(&mut self, key_code: &EvdevKeyCode, key_value: &i32) {
        // Update the arrays
        let key_update_result = self.current_keys.receive_keyevent(&key_code, &key_value);
        // Update the statistics
        self.update_keycount_hashmap(&key_update_result);
    }
    pub fn get_keys_pressed_stats(&self) -> &HashMap<EvdevKeyCode, HashMap<EvdevModMask, u32>> {
        return &self.keys_pressed_stats;
    }
    pub fn set_keys_pressed_stats(
        &mut self,
        new_keys_pressed_stats: HashMap<EvdevKeyCode, HashMap<EvdevModMask, u32>>,
    ) -> () {
        self.keys_pressed_stats = new_keys_pressed_stats;
    }
    /// (key_code, number of clics, mod_mask)
    pub fn all_keys_stats_vec(&self) -> Option<Vec<(EvdevKeyCode, u32, EvdevModMask)>> {
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
    pub fn max_clicked_keys_all_layer(&self) -> Vec<(EvdevKeyCode, u32)> {
        let mut vec = Vec::new();
        for (key_code, mod_key_hashmap) in self.keys_pressed_stats.iter() {
            let mut total_clicks = 0;
            for (_mod_mask, clicks) in mod_key_hashmap.iter() {
                total_clicks += clicks;
            }
            vec.push((key_code.clone(), total_clicks.clone()));
        }
        vec.sort_by(|(_, clicks_0),(_, clicks_1)| clicks_1.cmp(clicks_0));
        vec
    }
    /// Get a sorted vec with key from the most clicked to the least clicked
    pub fn max_clicked_keys(&self, mod_mask: &EvdevModMask) -> Vec<(EvdevKeyCode, u32)> {
        let mut vec = Vec::new();
        for (key_code, mod_key_hashmap) in self.keys_pressed_stats.iter() {
            for (_mod_mask, clicks) in mod_key_hashmap.iter() {
                if mod_mask == _mod_mask {
                    vec.push((key_code.clone(), clicks.clone()));
                }
            }
        }
        vec.sort_by(|(_, clicks_0),(_, clicks_1)| clicks_1.cmp(clicks_0));
        vec
    }
    /// Get all clicks mod independent
    pub fn all_clicks(&self, key_code: &EvdevKeyCode) -> u32 {
        let mut total_clicks = 0;
        for (_key_code, mod_key_hashmap) in self.keys_pressed_stats.iter() {
            if key_code == _key_code {
                for (_mod_mask, clicks) in mod_key_hashmap.iter() {
                    total_clicks += clicks;
                }
            }
        }
        total_clicks
    }
    /// Get number of clicks of a key with a specific mod on (or no mod)
    pub fn clicks(&self, key_code: &EvdevKeyCode, mod_mask: &EvdevModMask) -> u32 {
        let mut total_clicks = 0;
        for (_key_code, mod_key_hashmap) in self.keys_pressed_stats.iter() {
            if key_code == _key_code {
                for (_mod_mask, clicks) in mod_key_hashmap.iter() {
                    if mod_mask == _mod_mask {
                        total_clicks += clicks;
                    }
                }
            }
        }
        total_clicks
    }

    /// map of all keys with clicks for a given mod_mask
    /// same_layer_rule: if true shift_l shift_r are traited as one
    pub fn hashmap_mod_keys(
        &self,
        mod_mask: &EvdevModMask,
        same_layer_rule: &bool,
    ) -> HashMap<EvdevKeyCode, u32> {
        let mut hashmap = HashMap::<EvdevKeyCode, u32>::new();
        for (key_code, mod_key_hashmap) in self.keys_pressed_stats.iter() {
            for (_mod_mask, clicks) in mod_key_hashmap.iter() {
                if *same_layer_rule
                    && EvdevX11Converter::mod_mask_to_layer(mod_mask)
                        == EvdevX11Converter::mod_mask_to_layer(_mod_mask)
                {
                    hashmap
                        .entry(*key_code)
                        .and_modify(|val| *val += clicks)
                        .or_insert(*clicks);
                } else if !*same_layer_rule && mod_mask == _mod_mask {
                    hashmap.insert(*key_code, *clicks);
                    break;
                }
            }
        }
        hashmap
    }

    fn push_key_history(&mut self, key_code: &EvdevKeyCode) {
        if self.keys_history.len() == MAX_KEYS_CHAIN {
            self.keys_history.pop_front();
        }
        self.keys_history.push_back(key_code.clone());
    }
    /// Single key stats with the given mod_mask
    fn key_stats(&self, key_code: &EvdevKeyCode, mod_mask: &EvdevModMask) -> u32 {
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
