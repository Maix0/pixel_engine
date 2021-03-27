extern crate pixel_engine as engine;
extern crate rand;

use engine::inputs::Keycodes::Escape;
use engine::traits::*;
fn main() {
    let game = engine::EngineWrapper::new("Random".to_owned(), (256, 240, 5));
    use rand::Rng;
    let mut rng = rand::thread_rng();
    game.run(move |game: &mut engine::Engine| {
        if game.get_key(Escape).any() {
            return Ok(false);
        }
        for x in 0..game.size.0 {
            for y in 0..game.size.1 {
                game.draw((x, y), engine::Color::new(rng.gen(), rng.gen(), rng.gen()))
            }
        }
        Ok(true)
    });
}
