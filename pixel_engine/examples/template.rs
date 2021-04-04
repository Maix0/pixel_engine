extern crate pixel_engine as engine;

#[allow(unused_imports)]
use engine::traits::*;
async fn init() {
    let game = engine::EngineWrapper::new("Template".to_owned(), (500, 500, 1)).await;

    game.run(|_game: &mut engine::Engine| {
        // Your code here
        Ok(true) // return Ok(false) to stop nicely and Err(_) to stop & print the error
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
