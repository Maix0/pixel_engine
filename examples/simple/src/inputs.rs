extern crate pixel_engine as px;

#[cfg(target_arch = "wasm32")]
extern crate wasm_bindgen;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use px::inputs::Keycodes;
async fn init() {
    let game = px::EngineWrapper::new("Input".to_owned(), (50, 50, 10)).await;
    game.run(|game: &mut px::Engine| {
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

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]

pub fn inputs() {
    px::launch(init())
}