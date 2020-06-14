extern crate pixel_engine_gl as engine;
extern crate rand;

use engine::keyboard::Keycodes::{Down, Escape, Left, Right, Space, Up};
fn main() -> Result<(), String> {
    let mut game = engine::Engine::new("Random".to_owned(), (256, 240, 2));
    use rand::Rng;
    let mut rng = rand::thread_rng();
    game.run(|game: &mut engine::Engine| {
        if game.get_key(Escape).is_some() {
            return Ok(false);
        }
        for x in 0..game.size.0 {
            for y in 0..game.size.1 {
                game.screen
                    .draw(x, y, engine::Color::new(rng.gen(), rng.gen(), rng.gen()))
            }
        }
        Ok(true)
    });
    Ok(())
}
