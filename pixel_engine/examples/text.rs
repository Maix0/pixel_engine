extern crate pixel_engine as engine;
use engine::traits::*;
fn main() {
    let mut game = engine::EngineWrapper::new("Text".to_owned(), (500, 500, 1));
    game.get_inner()
        .screen
        .draw_text(0, 0, 1, [255, 255, 255].into(), "BONJOUR".into());
    game.run(|_game| Ok(true));
}
