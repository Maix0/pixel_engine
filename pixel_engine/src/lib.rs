//! A pixel base game engine

#![cfg_attr(target_os = "emscripten", allow(unused_mut))]
extern crate pixel_engine_backend as px_backend;
extern crate pixel_engine_draw as px_draw;

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
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]
mod engine;
pub use engine::*;
