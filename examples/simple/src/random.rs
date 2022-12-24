extern crate pixel_engine as px;
extern crate rand;

#[cfg(target_arch = "wasm32")]
extern crate wasm_bindgen;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use px::inputs::Keycodes::Escape;
use px::traits::*;
async fn init() {
    let game = px::EngineWrapper::new("Random".to_owned(), (256, 240, 5)).await;
    use rand::Rng;
    let mut rng = rand::thread_rng();
    game.run(move |game: &mut px::Engine| {
        if game.get_key(Escape).any() {
            return Ok(false);
        }
        for x in 0..game.size().x {
            for y in 0..game.size().y {
                game.draw(
                    (x as i32, y as i32),
                    px::Color::new(rng.gen(), rng.gen(), rng.gen()),
                )
            }
        }
        Ok(true)
    });
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]

pub fn random() {
    px::launch(init())
}
