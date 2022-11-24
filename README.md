# pixel_engine

A reproduction of the oldPixelGameEngine(by OneLoneCoder) written in rust
This crate is split between 3 crates:

## pixel_engine_backend

This is provide wrapper around wgpu.
It handles the drawing of decals and the main screen

## pixel_engine_draw

This crate provide Traits to handle the Drawing.
You only need to implement one trait (The SmartDrawing trait) and the other trait are just supertrait, so you have them for free

## pixel_engine

This is the core of the projects, It is the main library, aimed to be used by the user.
This provide the Engine struct

# How to use

There are plenty of examples in the `examples` folder.
You only need to run `cargo run --bin=<NAME>`, or go to (https://maix.me)[https://maix.me/] to get a list of example code.

```rust
extern crate pixel_engine as px;
use px::traits::*;
fn main() {
    px::launch(async move { // the launch function is just a utility function to block on async, even on the web
        let game = px::EngineWrapper::new("Lines".to_owned(), (25, 25, 20));
        let mut start = (0, 0);
        let mut end = (5i32, 5i32);
        game.run(move |game: &mut engine::Engine| {
            // Drawing to the screen:
            game.clear([0, 0, 0].into());
            game.draw_line(
                start,
                end,
                [1.0, 1.0, 1.0].into(),
            );
            game.draw(start, [0, 255, 0].into());
            game.draw(end, [255, 0, 0].into());
            
            some_failible_function()?; // You can return errors, but it will crash the program and print the error message
            // Handling inputs
            game.get_key(px::inputs::Keycodes::Escape).any() {
                return Ok(false); // Returning Ok(false) is the only way to do a clean shutdown
            }
            Ok(true) // Continue to next frame
        });
    });
}
```

This is the stripped-down code of the `line` example.
There are some examples that aren't really useful (like the `input.rs`). They are here to make sure I don't break stuff.
