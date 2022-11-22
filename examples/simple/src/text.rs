extern crate pixel_engine as px;
use px::traits::*;

#[cfg(target_arch = "wasm32")]
extern crate wasm_bindgen;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

async fn init() {
    let mut game = px::EngineWrapper::new("Text".to_owned(), (500, 500, 1)).await;
    game.draw_text((0, 0), 1, [255, 255, 255].into(), "BONJOUR");
    game.run(|_game| Ok(true));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]

pub fn text() {
    px::launch(init())
}
