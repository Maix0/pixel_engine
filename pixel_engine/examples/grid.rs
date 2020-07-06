extern crate pixel_engine as engine;

use engine::traits::*;
use engine::*;
const STEP_BY: usize = 5;
fn main() {
    let mut game = EngineWrapper::new("Grid".to_owned(), (150, 150, 1));
    let game_inner = game.get_inner();
    game_inner.screen.clear(Color::BLACK);
    for x in (0..game_inner.size.0).step_by(STEP_BY) {
        game_inner
            .screen
            .draw_line((x, 0), (x, game_inner.size.1 - 1), Color::GREEN);
    }
    for y in (0..game_inner.size.1).step_by(STEP_BY) {
        game_inner
            .screen
            .draw_line((0, y), (game_inner.size.0 - 1, y), Color::CYAN);
    }
    game.run(|_| Ok(true));
}
