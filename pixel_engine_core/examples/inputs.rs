extern crate pixel_engine as engine;

use engine::inputs::Keycodes;
fn main() {
    let game = engine::EngineWrapper::new("Input".to_owned(), (50, 50, 10));
    game.run(|game: &mut engine::Engine| {
        if game.get_key(Keycodes::Space).pressed {
            println!("[ PRESS ]")
        }
        if game.get_key(Keycodes::Space).released {
            println!("[RELEASE]")
        }
        if game.get_key(Keycodes::Space).held {
            println!("[ HELD  ]")
        }
        Ok(true)
    });
}
