pub use glutin::VirtualKeyCode as Keycodes;
#[derive(Eq, PartialEq, Debug, Hash, Clone, Copy)]
/// Represent a Key
pub struct Key {
    /// The keys modifiers
    /// [SHIFT / CTRL / ALT / WIN]
    pub modifier: [bool; 4],
    /// The keycode
    pub key: Keycodes,
}
impl Key {
    /// Return the key's text if it exist
    pub fn get_str_option(&self) -> Option<String> {
        if self.get_str() != "" {
            Some(self.get_str())
        } else {
            None
        }
    }
    /// Return the key's text if it exist, return blank string if not
    pub fn get_str(&self) -> String {
        use Keycodes::*;
        (match self.key {
            A => "a",
            B => "b",
            C => "c",
            D => "d",
            E => "e",
            F => "f",
            G => "g",
            H => "h",
            I => "i",
            J => "j",
            K => "k",
            L => "l",
            M => "m",
            N => "n",
            O => "o",
            P => "p",
            Q => "q",
            R => "r",
            S => "s",
            T => "t",
            U => "u",
            V => "v",
            W => "w",
            X => "x",
            Y => "y",
            Z => "z",
            Key1 | Numpad1 => "1",
            Key2 | Numpad2 => "2",
            Key3 | Numpad3 => "3",
            Key4 | Numpad4 => "4",
            Key5 | Numpad5 => "5",
            Key6 | Numpad6 => "6",
            Key7 | Numpad7 => "7",
            Key8 | Numpad8 => "8",
            Key9 | Numpad9 => "9",
            Key0 | Numpad0 => "0",
            Space => " ",
            _ => "",
        })
        .to_owned()
    }
}

impl From<glutin::KeyboardInput> for Key {
    fn from(key: glutin::KeyboardInput) -> Self {
        Key {
            modifier: [
                key.modifiers.shift,
                key.modifiers.ctrl,
                key.modifiers.alt,
                key.modifiers.logo,
            ],
            key: key.virtual_keycode.unwrap(),
        }
    }
}

/// Trait to handle Keysets
pub trait KeySet {
    /// Return true if the set has the key
    fn has(&self, key: Keycodes) -> bool;
}
impl KeySet for std::collections::HashSet<Key> {
    fn has(&self, key: Keycodes) -> bool {
        for k in self {
            if k.key == key {
                return true;
            }
        }
        false
    }
}
