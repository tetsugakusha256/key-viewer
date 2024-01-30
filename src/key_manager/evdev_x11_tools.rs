use crate::key_manager::EvdevKeyCode;
use xkbcommon::xkb;
use xkbcommon::xkb::Context;
use xkbcommon::xkb::Keymap;
use xkbcommon::xkb::KEYMAP_COMPILE_NO_FLAGS;

use super::key_types;
use super::key_types::EvdevModMask;

//TODO: add a layer to mod_mask converter and layer const and layer type
pub struct EvdevX11Converter {
    keymap: Keymap,
}
impl EvdevX11Converter {
    pub fn new(layout: &str) -> EvdevX11Converter {
        EvdevX11Converter {
            keymap: Keymap::new_from_names(
                &Context::new(xkb::CONTEXT_NO_FLAGS),
                "evdev",
                "pc105",
                layout,
                "",
                None,
                KEYMAP_COMPILE_NO_FLAGS,
            )
            .unwrap(),
        }
    }

    /// Return the x11 layer based on the EvdevModMask or 0
    pub fn mod_mask_to_layer(mod_mask: &EvdevModMask) -> u32 {
        let mod_mask = mod_mask.0;
        // Check the mod_mask for (shift, altgr ...)
        // layer 0=0, 1= 2 or 64, 2 = 16, 3=18 or 80, 4=32, 5=34 or 96
        // TODO: deal with iso5 layer that can be on any key even iso3
        match mod_mask {
            0 => 0,
            2 | 64 => 1,
            16 => 2,
            18 | 80 => 3,
            32 => 4,
            34 | 96 => 5,
            _ => 0,
        }
    }
    /// given an evdev keycode and a custom mod_mask
    /// return sym char or keyname if no sym char found
    fn get_x11_char(&self, keycode: &EvdevKeyCode, mod_mask: &EvdevModMask) -> String {
        let layer = EvdevX11Converter::mod_mask_to_layer(mod_mask);
        let x11_keycode = xkb::Keycode::new(keycode.0 as u32 + 8);
        let keysym = self.keymap.key_get_syms_by_level(x11_keycode, 0, layer);
        let keyname = self.keymap.key_get_name(x11_keycode);
        let mut text = String::new();
        if keysym.len() == 0 {
            text = "No keysym".to_string();
        } else {
            for sym in keysym.iter() {
                if let Some(keysym) = sym.key_char() {
                    text = format!("{}", keysym);
                } else {
                }
            }
        }
        if let Some(keyname) = keyname {
            if text.len() == 0 {
                text = format!("{}", keyname)
            };
        }
        text
    }
    // Return the char or key name of the key given a certain layer(mod_mask) for the current
    // layout()
    pub fn get_key_char(&self, keycode: &EvdevKeyCode, mod_mask: &EvdevModMask) -> String {
        let key_code_string = self.get_x11_char(&keycode, mod_mask);
        let name = key_types::evdev_keycode_to_name(&keycode);

        // Renaming some keys that x11 don't return correctly
        let _name = match key_code_string.as_str() {
            "No keysym" | "TAB" | "LALT" | "I151" | "LCTL" | "LWIN" | "RALT" | "PRSC" | "RCTL"
            | "PGUP" | "PGDN" | "HOME" | "END" | "INS" => name.clone(),
            _ => key_code_string,
        };
        // FIX: dead accent not showing
        let __name = match _name.as_str() {
            "LFSH" => "LShift".to_string(),
            "RTSH" => "RShift".to_string(),
            "AD01" => "`".to_string(),
            "AD05" => "´".to_string(),
            "AB01" => "^".to_string(),
            _ => _name,
        };
        let ___name = if __name.trim().len() == 0 {
            name
        } else if *keycode == EvdevKeyCode(1) {
            name
        } else if *keycode == EvdevKeyCode(14) {
            name
        } else if *keycode == EvdevKeyCode(111) {
            name
        } else {
            __name.clone()
        };

        ___name
    }
}
