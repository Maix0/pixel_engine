Pixel Engine In-Game Console Extension

This crate is an extension to the pixel engine.
It provides an in-game console with the ability to receive commands (in form of a string)
It also provides a simple way to print message into this console through log's API
(macros are also included in the crate)

This extension is designed to be fully usable with future extension

```rust
extern crate pixel_engine;
extern crate pixel_engine_console;

struct Game;
impl pixel_engine::Game for Game {
    fn create(engine: &mut Engine) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self)
    }

    fn update(&mut self, engine: &mut Engine) -> Result<bool, Box<dyn std::error::Error>> {
        // Opens the console
        engine.open_console(
            Keycodes::Escape, // The key used to close the console
            false,            // Does the update function will be called when the console is opened
        );

        cinfo!("Hello console !"); // Print a info message into the console
    }
}

impl ConsoleGame for Game {
    fn receive_console_input(&mut self, engine: &mut Engine, input: String) {
        // process the input
        // for example the shlex crate allow you to split the input into arguments like a shell
    }
}

fn main() {
    pixel_engine::start::<Game>("Console Example", (500, 500), 2);
}
```
