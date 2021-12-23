extern crate pixel_engine as px;
use px::traits::*;

#[cfg(target_arch = "wasm32")]
extern crate wasm_bindgen;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
async fn init() {
    let game = px::EngineWrapper::new("Circle".to_owned(), (51, 51, 10)).await;
    game.run(|game: &mut px::Engine| {
        game.clear(px::Color::WHITE);
        game.draw_circle((25, 25), 25, px::Color::BLACK);
        game.fill_circle((25, 25), 12, px::Color::BLUE);
        Ok(true)
    });
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn circle() {
    px::launch(init())
}
