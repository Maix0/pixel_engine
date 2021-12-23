extern crate pixel_engine as engine;
extern crate rand;

use engine::inputs::Keycodes::Escape;
use engine::traits::*;
async fn init() {
    let game = engine::EngineWrapper::new("Random".to_owned(), (256, 240, 5)).await;
    use rand::Rng;
    let mut rng = rand::thread_rng();
    game.run(move |game: &mut engine::Engine| {
        if game.get_key(Escape).any() {
            return Ok(false);
        }
        for x in 0..game.size.0 {
            for y in 0..game.size.1 {
                game.draw(
                    (x as i32, y as i32),
                    engine::Color::new(rng.gen(), rng.gen(), rng.gen()),
                )
            }
        }
        Ok(true)
    });
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        use std::panic;
        panic::set_hook(Box::new(pixel_engine::console_error_panic_hook::hook));
        pixel_engine::wasm_bindgen_futures::spawn_local(init());
    };
    #[cfg(not(target_arch = "wasm32"))]
    pixel_engine::futures::executor::block_on(init());
}
