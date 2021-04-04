extern crate pixel_engine as engine;
use engine::traits::*;
async fn init() {
    let mut game = engine::EngineWrapper::new("Text".to_owned(), (500, 500, 1)).await;
    game.draw_text((0, 0), 1, [255, 255, 255].into(), "BONJOUR".into());
    game.run(|_game| Ok(true));
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
