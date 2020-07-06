# pixel_engine
A reproduction of the oldPixelGameEngine(by OneLoneCoder) written in rust 
This crate is split between 3 crates:

## pixel_engine_backend
This is provide wrapper around wgpu.
The end goal is to be able to change the backend with features

## pixel_engine_draw
This crate provide Traits to handle the Drawing.
You only need to implement one trait (The basic Drawing trait) and the Other trait are derived from the main trait

## pixel_engine
This is the core of the projets, It is the main library, aimed to be used by the user.
This provied and Engine struct

# How to use
There are plenty of examples in the `examples` folder.
You only need to run `cargo run --example=<NAME>` inside the `pixel_engine` crate to run them.

```rust
extern crate pixel_engine_core as engine;
use engine::traits::*;
fn main() {
    let game = engine::EngineWrapper::new("Lines".to_owned(), (25, 25, 20));
    let mut start = vec![0, 0];
    let mut end = vec![5u32, 5u32];
    game.run(move |game: &mut engine::Engine| {
        game.screen.clear([0, 0, 0].into());
        game.screen.draw_line(
            (start[0] as u32, start[1] as u32),
            (end[0] as u32, end[1] as u32),
            [1.0, 1.0, 1.0].into(),
        );
        game.screen
            .draw(start[0] as u32, start[1] as u32, [0, 255, 0].into());
        game.screen
            .draw(end[0] as u32, end[1] as u32, [255, 0, 0].into());
        Ok(true)
    });
}
```

This is the stripped-down code of the `line` example.
There are some examples that aren't really usefull (like the `input.rs`). They are here to make sure I don't break stuff. 
