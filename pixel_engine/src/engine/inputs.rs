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
    #[must_use]
    pub const fn new(pressed: bool, held: bool, released: bool) -> Self {
        Input {
            pressed,
            held,
            released,
        }
    }
    /// Create an [`Input`] where all field are set to false
    #[must_use]
    pub const fn default() -> Self {
        Input {
            pressed: false,
            held: false,
            released: false,
        }
    }
    /// Return true if any of the field is true, false otherwise
    #[must_use]
    pub fn any(self) -> bool {
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

impl From<winit::event::KeyboardInput> for Key {
    fn from(key: winit::event::KeyboardInput) -> Self {
        Self {
            key: key.virtual_keycode.unwrap(),
        }
    }
}
