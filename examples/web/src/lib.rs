
extern crate fps_pixel;
extern crate pixel_engine;
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    fps_pixel::main();
}
