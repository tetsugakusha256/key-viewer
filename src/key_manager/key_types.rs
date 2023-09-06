use std::fmt;

use evdev::Key;
// modkeys : shift_l 50 shift_r 62 ctrl_l 37 ctr_r 105 Superl 133 alt_l 64 iso3 108 iso5 107
pub const KEY_LEFTALT: EvdevKeyCode = EvdevKeyCode(Key::KEY_LEFTALT.code());
pub const KEY_LEFTCTRL: EvdevKeyCode = EvdevKeyCode(Key::KEY_LEFTCTRL.code());
pub const KEY_LEFTMETA: EvdevKeyCode = EvdevKeyCode(Key::KEY_LEFTMETA.code());
pub const KEY_LEFTSHIFT: EvdevKeyCode = EvdevKeyCode(Key::KEY_LEFTSHIFT.code());
pub const KEY_RIGHTCTRL: EvdevKeyCode = EvdevKeyCode(Key::KEY_RIGHTCTRL.code());
pub const KEY_RIGHTSHIFT: EvdevKeyCode = EvdevKeyCode(Key::KEY_RIGHTSHIFT.code());
pub const KEY_ISO3: EvdevKeyCode = EvdevKeyCode(Key::KEY_RIGHTALT.code());
pub const KEY_ISO5: EvdevKeyCode = EvdevKeyCode(Key::KEY_PRINT.code());

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub struct EvdevKeyCode(pub u16);
impl From<u16> for EvdevKeyCode {
    fn from(val: u16) -> EvdevKeyCode {
        EvdevKeyCode(val)
    }
}
impl Into<u16> for EvdevKeyCode {
    fn into(self) -> u16 {
        self.0
    }
}
impl fmt::Display for EvdevKeyCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.0)
    }
}
// TODO: use custom type to improve type safety and comprehension
// and implement a function keycode + keymask -> char using layout info
// and a simpler version keycode -> to named key using layout
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct EvdevModMask(pub u16);
impl From<u16> for EvdevModMask {
    fn from(val: u16) -> EvdevModMask {
        EvdevModMask(val)
    }
}
impl Into<u16> for EvdevModMask {
    fn into(self) -> u16 {
        self.0
    }
}
impl fmt::Display for EvdevModMask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut text = String::from("");
    let mod_mask = self.0;
    if (mod_mask >> 0 & 1) == 1 {
        text = text + "Alt_l, ";
    }
    if (mod_mask >> 1 & 1) == 1 {
        text = text + "Shift_l, ";
    }
    if (mod_mask >> 2 & 1) == 1 {
        text = text + "Meta_l, ";
    }
    if (mod_mask >> 3 & 1) == 1 {
        text = text + "Ctrl_l, ";
    }
    if (mod_mask >> 4 & 1) == 1 {
        text = text + "ISO_3, ";
    }
    if (mod_mask >> 5 & 1) == 1 {
        text = text + "ISO_5, ";
    }
    if (mod_mask >> 6 & 1) == 1 {
        text = text + "Shift_r, ";
    }
    if (mod_mask >> 7 & 1) == 1 {
        text = text + "Ctrl_r, ";
    }
        writeln!(f, "{}", text)
    }
}
