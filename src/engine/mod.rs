mod graphics;
mod handler;
/// User Input module
pub mod inputs;
mod logic;
mod screen;
/// Collection of trait used by the user
pub mod traits;

pub use graphics::{Color, Sprite};
pub use logic::Engine;
pub use screen::{PixelMode, Screen};
