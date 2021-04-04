extern crate pixel_engine as engine;

use engine::inputs::Keycodes;
async fn init() {
    let game = engine::EngineWrapper::new("Input".to_owned(), (50, 50, 10)).await;
    game.run(|game: &mut engine::Engine| {
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
