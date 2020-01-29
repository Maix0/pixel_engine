//! A pixel base game engine

#![cfg_attr(target_os = "emscripten", allow(unused_mut))]
#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate memblock;
/*#[deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]*/
/*
pub mod graphics;
pub mod handler;
pub mod keyboard;
pub mod logic;
pub mod screen;
*/
mod engine;
pub use engine::*;
