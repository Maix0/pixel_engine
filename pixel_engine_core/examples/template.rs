extern crate pixel_engine_gl as engine;

use engine::traits::*;
fn main() {
    let mut game = engine::Engine::new("Template".to_owned(), (500, 500, 1));
    game.run(|_game: &mut engine::Engine| {
        // Your code here
        Ok(true) // return Ok(false) to stop nicely and Err(_) to stop & print the error
    });
}
