pub trait TextInputSource {
    fn get_events(&self) -> Option<Event>;
}

#[derive(Clone, Copy, Debug)]
pub enum Event {
    CursorLeft,
    CursorRight,
    CursorMove { index: usize },
    Char(char),
}

struct TextInput;
