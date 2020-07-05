extern crate pixel_engine_gl as engine;

use engine::keyboard::Keycodes;
use engine::traits::*;
fn main() {
    let mut game = engine::Engine::new("Input".to_owned(), (50, 50, 10));
    game.run(|game: &mut engine::Engine| {
        if game.is_pressed(Keycodes::Space) {
            println!("[ PRESS ]")
        }
        if game.is_released(Keycodes::Space) {
            println!("[RELEASE]")
        }
        if game.is_held(Keycodes::Space) {
            println!("[ HELD  ]")
        }
        Ok(true)
    });
}
