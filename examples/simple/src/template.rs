extern crate pixel_engine as px;

#[cfg(target_arch = "wasm32")]
extern crate wasm_bindgen;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[allow(unused_imports)]
use px::traits::*;
async fn init() {
    let game = px::EngineWrapper::new("Template".to_owned(), (500, 500, 1)).await;

    game.run(|_game: &mut px::Engine| {
        // Your code here
        Ok(true) // return Ok(false) to stop nicely and Err(_) to stop & print the error
    });
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]

pub fn template() {
    px::launch(init())
}
