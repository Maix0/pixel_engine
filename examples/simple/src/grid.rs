extern crate pixel_engine as px;

#[cfg(target_arch = "wasm32")]
extern crate wasm_bindgen;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use px::traits::*;
use px::*;
const STEP_BY: usize = 5;
async fn init() {
    let mut game = EngineWrapper::new("Grid".to_owned(), (150, 150, 1)).await;
    let size = game.size();
    game.clear(Color::BLACK);
    for x in (0..size.x).step_by(STEP_BY) {
        game.draw_line((x as i32, 0), (x as i32, size.y as i32 - 1), Color::GREEN);
    }
    for y in (0..size.y).step_by(STEP_BY) {
        game.draw_line((0, y as i32), (size.x as i32 - 1, y as i32), Color::CYAN);
    }
    game.run(|game: &mut pixel_engine::Engine| {
        Ok(game.get_key(pixel_engine::inputs::Keycodes::Escape).any())
    });
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]

pub fn grid() {
    px::launch(init())
}
