extern crate pixel_engine as engine;
use engine::traits::*;
fn main() {
    let game = engine::EngineWrapper::new("Circle".to_owned(), (51, 51, 10));
    game.run(|game: &mut engine::Engine| {
        game.clear(engine::Color::WHITE);
        game.draw_circle((25, 25), 25, engine::Color::BLACK);
        game.fill_circle((25, 25), 12, engine::Color::BLUE);
        Ok(true)
    });
}
