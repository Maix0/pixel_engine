mod graphics;
mod handler;
/// Keyboard module
pub mod keyboard;
mod logic;
mod screen;

pub use graphics::{Color, Sprite};
pub use logic::Engine;
pub use screen::Screen;

/// Module that re-export Traits
pub mod traits {
    /// Basic Drawing Trait (basic shapes,...)
    pub use super::screen::ScreenTrait;
    /// Handlings Sprites
    pub use super::screen::SpriteTrait;
}
