#![cfg_attr(target_os = "emscripten", allow(unused_mut))]
#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;
pub mod graphics;
pub mod handler;
pub mod keyboard;
pub mod logic;
pub mod screen;
