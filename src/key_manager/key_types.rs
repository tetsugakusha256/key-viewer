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
pub const LAYER_0: EvdevModMask = EvdevModMask(0);
pub const LAYER_1: EvdevModMask = EvdevModMask(2);
pub const LAYER_2: EvdevModMask = EvdevModMask(16);
pub const LAYER_3: EvdevModMask = EvdevModMask(18);
pub const LAYER_4: EvdevModMask = EvdevModMask(32);
pub const LAYER_5: EvdevModMask = EvdevModMask(34);

pub enum Layer {
    AllLayer,
    Layer0,
    Layer1,
    Layer2,
    Layer3,
    Layer4,
    Layer5,
}
impl Into<EvdevModMask> for &Layer{
    fn into(self) -> EvdevModMask {
        match self{
            Layer::AllLayer => LAYER_0,
            Layer::Layer0 => LAYER_0,
            Layer::Layer1 => LAYER_1,
            Layer::Layer2 => LAYER_2,
            Layer::Layer3 => LAYER_3,
            Layer::Layer4 => LAYER_4,
            Layer::Layer5 => LAYER_5,
        }
    }
}
impl Into<EvdevModMask> for Layer{
    fn into(self) -> EvdevModMask {
        match self{
            Layer::AllLayer => LAYER_0,
            Layer::Layer0 => LAYER_0,
            Layer::Layer1 => LAYER_1,
            Layer::Layer2 => LAYER_2,
            Layer::Layer3 => LAYER_3,
            Layer::Layer4 => LAYER_4,
            Layer::Layer5 => LAYER_5,
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
