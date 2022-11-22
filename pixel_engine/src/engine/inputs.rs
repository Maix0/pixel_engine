use px_backend::winit;
#[derive(Debug, Clone, Copy)]
/// A Simple Struct that Represent an Input
pub struct Input {
    /// Is the input pressed on that frame
    pub pressed: bool,
    /// Is the input held on that frame
    pub held: bool,
    /// Is the input released on that frame
    pub released: bool,
}

impl Input {
    /// Create a new [`Input`] with the given values
    #[must_use] pub const fn new(pressed: bool, held: bool, released: bool) -> Self {
        Input {
            pressed,
            held,
            released,
        }
    }
    /// Create an [`Input`] where all field are set to false
    #[must_use] pub const fn default() -> Self {
        Input {
            pressed: false,
            held: false,
            released: false,
        }
    }
    /// Return true if any of the field is true, false otherwise
    #[must_use] pub fn any(self) -> bool {
        self.pressed || self.held || self.released
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Mouse {
    /// State of each mouse button
    /// 0 => None
    /// 1 => Pressed
    /// 2 => Held
    /// 3 => Released
    pub(crate) buttons: [Input; 3],
    pub(crate) pos: (u32, u32),
    pub(crate) wheel: MouseWheel,
}

impl Mouse {
    pub const fn new() -> Self {
        Mouse {
            buttons: [Input::default(), Input::default(), Input::default()],
            pos: (0, 0),
            wheel: MouseWheel::None,
        }
    }
}
/// Represent a Mouse Button
#[derive(Debug, Clone, Copy)]
pub enum MouseBtn {
    /// The left click
    Left,
    /// The right click
    Right,
    /// The left middle click (scroll wheel click)
    Middle,
}

/// Represent a scroll wheel Direction
#[derive(Debug, Clone, Copy)]
pub enum MouseWheel {
    /// No Scroll
    None,
    /// Scrolling Up
    Up,
    /// Scrolling Down
    Down,
    /// Scrolling Right
    Right,
    /// Scrolling Left
    Left,
}

pub use winit::event::VirtualKeyCode as Keycodes;
#[derive(Eq, PartialEq, Debug, Hash, Clone, Copy)]
/// Represent a Key
pub struct Key {
    /// The keycode
    pub key: Keycodes,
}
impl Key {
    /// Return the key's text if it exist
    #[must_use] pub fn get_str_option(self) -> Option<String> {
        if self.get_str() != "" {
            Some(self.get_str())
        } else {
            None
        }
    }
    /// Return the key's text if it exist, return blank string if not
    #[must_use] pub fn get_str(self) -> String {
        use Keycodes::{A, B, C, D, E, F, G, H, I, J, K, Key0, Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9, L, M, N, Numpad0, Numpad1, Numpad2, Numpad3, Numpad4, Numpad5, Numpad6, Numpad7, Numpad8, Numpad9, O, P, Q, R, S, Space, T, U, V, W, X, Y, Z};
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

impl From<winit::event::KeyboardInput> for Key {
    fn from(key: winit::event::KeyboardInput) -> Self {
        Self {
            key: key.virtual_keycode.unwrap(),
        }
    }
}

/// Trait to handle Keysets
pub(crate) trait KeySet {
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
