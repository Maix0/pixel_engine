extern crate pixel_engine as engine;

use engine::traits::*;
use engine::*;
const STEP_BY: usize = 5;
fn main() {
    let mut game = EngineWrapper::new("Grid".to_owned(), (150, 150, 1));
    let size = game.size;
    game.clear(Color::BLACK);
    for x in (0..game.size.0).step_by(STEP_BY) {
        game.draw_line((x, 0), (x, size.1 - 1), Color::GREEN);
    }
    for y in (0..game.size.1).step_by(STEP_BY) {
        game.draw_line((0, y), (size.0 - 1, y), Color::CYAN);
    }
    game.run(|_| Ok(true));
}
