#![allow(dead_code)]

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Side {
    Left,
    Right,
}
#[derive(Clone, Debug)]
pub struct SwapBuffer<T> {
    left: Vec<T>,
    right: Vec<T>,
    primary: Side,
}

impl<T> SwapBuffer<T> {
    pub fn new() -> Self {
        Self {
            left: Vec::new(),
            right: Vec::new(),
            primary: Side::Left,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            left: Vec::with_capacity(capacity),
            right: Vec::with_capacity(capacity),
            primary: Side::Left,
        }
    }

    pub fn switch(&mut self) {
        self.primary = match self.primary {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }
}

impl<T> std::ops::Deref for SwapBuffer<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        match self.primary {
            Side::Left => &self.left,
            Side::Right => &self.right,
        }
    }
}
impl<T> std::ops::DerefMut for SwapBuffer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self.primary {
            Side::Left => &mut self.left,
            Side::Right => &mut self.right,
        }
    }
}
