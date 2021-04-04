extern crate pixel_engine as engine;
use engine::traits::*;

async fn init() {
    let game = engine::EngineWrapper::new("Circle".to_owned(), (51, 51, 10)).await;
    game.run(|game: &mut engine::Engine| {
        game.clear(engine::Color::WHITE);
        game.draw_circle((25, 25), 25, engine::Color::BLACK);
        game.fill_circle((25, 25), 12, engine::Color::BLUE);
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
