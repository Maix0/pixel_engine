extern crate pixel_engine as engine;

use engine::traits::*;
use engine::*;
const STEP_BY: usize = 5;
async fn init() {
    let mut game = EngineWrapper::new("Grid".to_owned(), (150, 150, 1)).await;
    let size = game.size;
    game.clear(Color::BLACK);
    for x in (0..game.size.0).step_by(STEP_BY) {
        game.draw_line((x as i32, 0), (x as i32, size.1 as i32 - 1), Color::GREEN);
    }
    for y in (0..game.size.1).step_by(STEP_BY) {
        game.draw_line((0, y as i32), (size.0 as i32 - 1, y as i32), Color::CYAN);
    }
    game.run(|_| Ok(true));
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
