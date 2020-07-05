//! A pixel base game engine

#![cfg_attr(target_os = "emscripten", allow(unused_mut))]
extern crate pixel_engine_backend as px_backend;
extern crate pixel_engine_draw as px_draw;
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
