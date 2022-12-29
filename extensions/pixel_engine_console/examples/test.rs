
use pixel_engine_console::{self, ConsoleEngine};
#[macro_use]
extern crate log;

use pixel_engine::inputs::Keycodes;
use pixel_engine::traits::*;

struct Game;

impl pixel_engine::Game for Game {
    fn create(engine: &mut pixel_engine::Engine) -> Result<Self, Box<dyn std::error::Error>> {
        engine.set_ignore_passthrough_chars(true);
        Ok(Self)
    }

    fn update(
        &mut self,
        engine: &mut pixel_engine::Engine,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        engine.add_input_passthrough(
            [
                Keycodes::Key1,
                Keycodes::Key2,
                Keycodes::Key3,
                Keycodes::Key4,
                Keycodes::Key5,
            ]
            .into_iter(),
        );

        if engine.get_key(Keycodes::Key1).pressed {
            error!(target:"console", "This is an error message");
        }
        if engine.get_key(Keycodes::Key2).pressed {
            warn!(target:"console", "This is an warn message");
        }
        if engine.get_key(Keycodes::Key3).pressed {
            info!(target:"console", "This is an info message");
        }
        if engine.get_key(Keycodes::Key4).pressed {
            debug!(target:"console", "This is an debug message");
        }
        if engine.get_key(Keycodes::Key5).pressed {
            trace!(target:"console", "This is an trace message");
        }

        if engine.get_key(Keycodes::Space).pressed {
            engine.open_console(Keycodes::Escape, true);
        }
        if engine.get_key(Keycodes::Escape).pressed && !engine.is_console_opened() {
            return Ok(false);
        }
        Ok(true)
    }

    fn receive_input(&mut self, _engine: &mut pixel_engine::Engine, input: String) {
        debug!(target: "console", "recieved_input: {input:?}");
    }
}

impl pixel_engine_console::ConsoleGame for Game {
    fn receive_console_input(&mut self, _engine: &mut pixel_engine::Engine, input: String) {
        debug!(target:"console", "console command: {input:?}");
    }

    fn create_console_game(
        engine: &mut pixel_engine::Engine,
    ) -> Result<(Self, pixel_engine_console::PixelConsoleOptions), Box<dyn std::error::Error>> {
        use pixel_engine::Game as _;
        Ok((Game::create(engine)?, Default::default()))
    }
}

async fn game() {
    let mut engine =
        pixel_engine::EngineWrapper::new("Console Test".to_string(), (500, 500, 2)).await;
    engine.clear(pixel_engine::Color::WHITE);
    engine.run_init::<pixel_engine_console::GameWrapper<Game>>();
}

fn main() {
    pixel_engine::launch(game());
}
