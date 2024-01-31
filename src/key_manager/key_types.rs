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
// layer 0=0, 1= 2 or 64, 2 = 16, 3=18 or 80, 4=32, 5=34 or 96, 6=8 or 128
// Base
pub const LAYER_0: EvdevModMask = EvdevModMask(0);
// Shift
pub const LAYER_1: EvdevModMask = EvdevModMask(2);
// ISO_3
pub const LAYER_2: EvdevModMask = EvdevModMask(16);
// Shift ISO_3
pub const LAYER_3: EvdevModMask = EvdevModMask(18);
// ISO_5
pub const LAYER_4: EvdevModMask = EvdevModMask(32);
// Shift ISO_5
pub const LAYER_5: EvdevModMask = EvdevModMask(34);
// Ctrl
pub const LAYER_6: EvdevModMask = EvdevModMask(8);
// Ctrl + Shift
pub const LAYER_7: EvdevModMask = EvdevModMask(10);
// Alt
pub const LAYER_8: EvdevModMask = EvdevModMask(1);
// Alt + Shift
pub const LAYER_9: EvdevModMask = EvdevModMask(3);

#[derive(PartialEq, Clone, Copy)]
pub enum Layer {
    AllLayer,
    Layer0,
    Layer1,
    Layer2,
    Layer3,
    Layer4,
    Layer5,
    Layer6,
    Layer7,
    Layer8,
    Layer9,
}
impl Into<EvdevModMask> for &Layer {
    fn into(self) -> EvdevModMask {
        match self {
            Layer::AllLayer => LAYER_0,
            Layer::Layer0 => LAYER_0,
            Layer::Layer1 => LAYER_1,
            Layer::Layer2 => LAYER_2,
            Layer::Layer3 => LAYER_3,
            Layer::Layer4 => LAYER_4,
            Layer::Layer5 => LAYER_5,
            Layer::Layer6 => LAYER_6,
            Layer::Layer7 => LAYER_7,
            Layer::Layer8 => LAYER_8,
            Layer::Layer9 => LAYER_9,
        }
    }
}
impl Into<EvdevModMask> for Layer {
    fn into(self) -> EvdevModMask {
        match self {
            Layer::AllLayer => LAYER_0,
            Layer::Layer0 => LAYER_0,
            Layer::Layer1 => LAYER_1,
            Layer::Layer2 => LAYER_2,
            Layer::Layer3 => LAYER_3,
            Layer::Layer4 => LAYER_4,
            Layer::Layer5 => LAYER_5,
            Layer::Layer6 => LAYER_6,
            Layer::Layer7 => LAYER_7,
            Layer::Layer8 => LAYER_8,
            Layer::Layer9 => LAYER_9,
        }
    }
}

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
#[derive(Debug, Clone, Copy, Hash, Eq, serde::Deserialize, serde::Serialize)]
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
/// defining equal as both shift are the same layer, both ctrl same layer
impl PartialEq for EvdevModMask {
    fn eq(&self, other: &Self) -> bool {
        // Define your custom equality logic here
        match self.0 {
            0 => {
                if other.0 == 0 {
                    return true;
                } else {
                    return false;
                }
            }
            2 | 64 => {
                if other.0 == 2 || other.0 == 64 {
                    return true;
                } else {
                    return false;
                }
            }
            16 => {
                if other.0 == 16 {
                    return true;
                } else {
                    return false;
                }
            }
            18 | 80 => {
                if other.0 == 18 || other.0 == 80 {
                    return true;
                } else {
                    return false;
                }
            }
            32 => {
                if other.0 == 32 {
                    return true;
                } else {
                    return false;
                }
            }
            34 | 96 => {
                if other.0 == 34 || other.0 == 96 {
                    return true;
                } else {
                    return false;
                }
            }
            8 | 128 => {
                if other.0 == 34 || other.0 == 96 {
                    return true;
                } else {
                    return false;
                }
            }
            x => {
                if x == other.0 {
                    return true;
                } else {
                    return false;
                }
            }
        }
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
pub fn evdev_keycode_to_name(key_code: &EvdevKeyCode) -> String {
    //Colemak layout
    return match key_code.0 {
        1 => "Esc",
        2 => "1",
        3 => "2",
        4 => "3",
        5 => "4",
        6 => "5",
        7 => "6",
        8 => "7",
        9 => "8",
        10 => "9",
        11 => "0",
        12 => "-",
        13 => "=",
        14 => "Backspace",
        15 => "tab",
        16 => "q",
        17 => "w",
        18 => "f",
        19 => "p",
        20 => "g",
        21 => "j",
        22 => "l",
        23 => "u",
        24 => "y",
        25 => "=",
        26 => "ç",
        27 => "]",
        28 => "cr",
        29 => "Ctrl",
        30 => "a",
        31 => "r",
        32 => "s",
        33 => "t",
        34 => "d",
        35 => "h",
        36 => "n",
        37 => "e",
        38 => "i",
        39 => "o",
        40 => "'",
        41 => "`",
        42 => "LFSH",
        43 => "¦",
        44 => "z",
        45 => "x",
        46 => "c",
        47 => "v",
        48 => "b",
        49 => "k",
        50 => "m",
        51 => ",",
        52 => ".",
        53 => "/",
        54 => "RTSH",
        56 => "Alt",
        57 => "Space",
        59 => "F1",
        60 => "F2",
        61 => "F3",
        62 => "F4",
        63 => "F5",
        64 => "F6",
        65 => "F7",
        66 => "F8",
        67 => "F9",
        68 => "F10",
        86 => "<",
        87 => "F11",
        88 => "F12",
        97 => "Ctrl",
        99 => "PrScn",
        100 => "AltGr",
        102 => "Home",
        103 => "Up",
        104 => "PgUp",
        107 => "End",
        109 => "PgDn",
        110 => "Ins",
        111 => "Del",
        125 => "Mod4",
        143 => "Fn",
        _ => "Unknown key",
    }
    .to_string();
}
