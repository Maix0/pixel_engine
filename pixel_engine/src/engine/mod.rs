pub use px_draw::graphics;
pub use px_draw::vector2;
/// A collection of traits used to draw things
pub mod traits;

mod decals;
/// User Input module
pub mod inputs;
mod logic;
mod screen;
pub use graphics::{Color, PixelMode, Sprite};
pub use logic::{Engine, EngineWrapper};

/// Takes a future and run it in the context of the engine
/// This is usefull when targeting wasm32 because we can't use the futures' `block_on` method
/// and we need to use javascript's promise type
pub fn launch<F: 'static + std::future::Future<Output = ()>>(f: F) {
    #[cfg(target_arch = "wasm32")]
    {
        use std::panic;
        panic::set_hook(Box::new(crate::console_error_panic_hook::hook));
        crate::wasm_bindgen_futures::spawn_local(f);
    };
    #[cfg(not(target_arch = "wasm32"))]
    crate::futures::executor::block_on(f);
}
