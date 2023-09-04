use xkbcommon::xkb;
use xkbcommon::xkb::Context;
use xkbcommon::xkb::Keymap;
use xkbcommon::xkb::KEYMAP_COMPILE_NO_FLAGS;

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
    fn mod_mask_to_layer(mod_mask:&u16) -> u32{
        *mod_mask as u32
    }
    /// given an evdev keycode and a custom mod_mask
    /// return sym char or keyname if no sym char found
    pub fn convert_keycode(&self, keycode: &u16, mod_mask:&u16) -> String {
        let layer = EvdevX11Converter::mod_mask_to_layer(mod_mask);
        let x11_keycode = xkb::Keycode::new(*keycode as u32 + 8);
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
}
