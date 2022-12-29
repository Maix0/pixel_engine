//! A pixel base game engine

#![warn(clippy::pedantic)]

extern crate pixel_engine_backend as px_backend;
extern crate pixel_engine_draw as px_draw;

#[cfg(not(target_arch = "wasm32"))]
pub extern crate futures;

#[cfg(target_arch = "wasm32")]
pub extern crate console_error_panic_hook;
#[cfg(target_arch = "wasm32")]
pub extern crate wasm_bindgen_futures;
#[deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    // unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]
#[allow(clippy::doc_markdown)]
/// Utility function that manage the engine, create the game instance and runs it
/// This will also work on WebAssembly
pub fn start<G: Game + 'static>(
    title: impl AsRef<str>,
    size: impl Into<px_draw::vector2::Vu2d>,
    scale: u32,
) {
    let title = title.as_ref().to_string();
    let size = size.into();
    launch(async move {
        let engine = EngineWrapper::new(title, (size.x, size.y, scale)).await;
        engine.run_init::<G>();
    });
}

mod engine;
pub use engine::*;
