extern crate pixel_engine as px;
use px::traits::*;

#[cfg(target_arch = "wasm32")]
extern crate wasm_bindgen;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
async fn init() {
    let mut game = px::EngineWrapper::new("Colors".to_owned(), (500, 500, 1)).await;

    for x in 0..game.size.0 {
        for y in 0..game.size.1 {
            let red: f32 = if x < (game.size.0 - 1) / 2 {
                1f32 - x as f32 / ((game.size.0 - 1) / 2) as f32
            } else {
                0f32
            };
            let green: f32 = 1f32
                - ((x as f32 - ((game.size.0) / 2) as f32).abs() / ((game.size.0) / 2) as f32)
                    as f32;
            let blue: f32 = if x > (game.size.0 - 1) / 2 {
                x as f32 / ((game.size.0 - 1) / 2) as f32
            } else {
                0f32
            };
            game.draw((x as i32, y as i32), [red, green, blue].into());
        }
    }
    game.run(|_| Ok(true));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]

pub fn colors() {
    px::launch(init())
}
