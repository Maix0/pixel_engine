extern crate pixel_engine as engine;

#[allow(unused_imports)]
use engine::traits::*;
fn main() {
    let game = engine::EngineWrapper::new("Template".to_owned(), (500, 500, 1));
    game.run(|_game: &mut engine::Engine| {
        // Your code here
        Ok(true) // return Ok(false) to stop nicely and Err(_) to stop & print the error
    });
}
