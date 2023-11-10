use std::fmt;

use super::key_types::*;

const MAX_KEYS: usize = 5;
const MAX_MOD_KEYS: usize = 5;
#[allow(dead_code)]
pub enum LogKeyEvent {
    KeyPressed(EvdevKeyCode),
    KeyReleased(EvdevKeyCode),
    KeyHold(EvdevKeyCode),
    KeyStayHold(EvdevKeyCode),
}
#[derive(Debug)]
struct KeysList([(EvdevKeyCode, i32); MAX_KEYS]);
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
    fn iter(&self) -> std::slice::Iter<(EvdevKeyCode, i32)> {
        self.0.iter()
    }
    fn iter_mut(&mut self) -> std::slice::IterMut<(EvdevKeyCode, i32)> {
        self.0.iter_mut()
    }
}
#[derive(Debug)]
pub struct KeystateMemory {
    pressed_keys: KeysList,
    pressed_mod_keys: KeysList,
}
#[allow(dead_code)]
impl KeystateMemory {
    // TODO: manage different max keys
    pub fn new() -> KeystateMemory {
        let mut my_array: [(EvdevKeyCode, i32); MAX_KEYS] = [(EvdevKeyCode(0), 0); MAX_KEYS]; // Initialize with default values
        let mut my_mod_array: [(EvdevKeyCode, i32); MAX_MOD_KEYS] =
            [(EvdevKeyCode(0), 0); MAX_MOD_KEYS]; // Initialize with default values
        for i in 0..MAX_KEYS {
            my_array[i] = (EvdevKeyCode(0), 0);
        }
        for i in 0..MAX_MOD_KEYS {
            my_mod_array[i] = (EvdevKeyCode(0), 0);
        }
        return KeystateMemory {
            pressed_keys: (KeysList(my_array)),
            pressed_mod_keys: (KeysList(my_mod_array)),
        };
    }
    pub fn get_current_keys_pressed() {}
    pub fn clear(&mut self) {
        for e in self.pressed_keys.iter_mut() {
            *e = (EvdevKeyCode(0), 0i32);
        }
    }
    pub fn is_key_pressed(&self, key_code: &EvdevKeyCode) -> bool{
        self.pressed_keys.iter().any(|e| e.0 == *key_code)
    }
    fn is_mod_key(key_code: &EvdevKeyCode) -> bool {
        match *key_code {
            KEY_LEFTALT | KEY_LEFTCTRL | KEY_LEFTMETA | KEY_LEFTSHIFT | KEY_ISO3
            | KEY_RIGHTCTRL | KEY_ISO5 | KEY_RIGHTSHIFT => true,
            _ => false,
        }
    }
    pub fn get_mod_keys_mask(&self) -> EvdevModMask {
        let mut mask = 0;
        for (code, value) in self.pressed_mod_keys.iter() {
            if *value != 0 && *code != EvdevKeyCode(0) {
                mask += KeystateMemory::mod_to_mod_mask(code).0
            }
        }
        EvdevModMask(mask)
    }
    /// Update the state with the new key event
    pub fn receive_keyevent(
        &mut self,
        key_code: &EvdevKeyCode,
        key_value: &i32,
    ) -> Option<LogKeyEvent> {
        // Update the arrays
        let key_update_result = if KeystateMemory::is_mod_key(key_code) {
            KeystateMemory::update_keystate(&mut self.pressed_mod_keys, &key_code, &key_value)
        } else {
            KeystateMemory::update_keystate(&mut self.pressed_keys, &key_code, &key_value)
        };
        key_update_result
    }
    fn mod_to_mod_mask(mod_key: &EvdevKeyCode) -> EvdevModMask {
        EvdevModMask::from(match *mod_key {
            KEY_LEFTALT => 1,
            KEY_LEFTSHIFT => 2,
            KEY_LEFTMETA => 4,
            KEY_LEFTCTRL => 8,
            KEY_ISO3 => 16,
            KEY_ISO5 => 32,
            KEY_RIGHTSHIFT => 64,
            KEY_RIGHTCTRL => 128,
            _ => 0,
        })
        // layer 0=0, 1= 2 or 64, 2 = 16, 3=18 or 80, 4=32, 5=34 or 96
    }
    fn update_keystate(
        key_list: &mut KeysList,
        key_code: &EvdevKeyCode,
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
            key_code: &mut EvdevKeyCode,
            new_key_code: &EvdevKeyCode,
            new_key_value: &i32,
        ) -> Option<LogKeyEvent> {
            let new_key_code = if *new_key_value == 0 {
                EvdevKeyCode(0)
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mask_to_mod() {
        let a = &EvdevModMask(0);
        let b = &EvdevModMask(1);
        let c = &EvdevModMask(3);
        let d = &EvdevModMask(7);
        //TODO: use contain
        assert_eq!(a.to_string(), "".to_string());
        assert_eq!(b.to_string(), "Alt_l, ".to_string());
        assert_eq!(c.to_string(), "Alt_l, Shift_l, ".to_string());
        assert!(d.to_string().contains("Alt_l"));
        assert!(d.to_string().contains("Shift_l"));
        assert!(d.to_string().contains("Meta_l"));
    }
}
