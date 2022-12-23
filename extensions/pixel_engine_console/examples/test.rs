use pixel_engine;
use pixel_engine_console;
#[macro_use]
extern crate log;

use pixel_engine::inputs::Keycodes;
use pixel_engine::traits::*;
async fn game() {
    let mut engine =
        pixel_engine::EngineWrapper::new("Console Test".to_string(), (500, 500, 2)).await;
    let console =
        pixel_engine_console::PixelConsole::new_with_options(&mut engine, Default::default());

    engine.clear(pixel_engine::Color::BLUE);
    engine.run(
        move |engine: &mut pixel_engine::Engine| -> Result<bool, _> {
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

            if engine.get_key(Keycodes::Space).any() {
                console.render(engine);
            }
            if engine.get_key(Keycodes::Escape).any() {
                return Ok(false);
            }
            Ok(true)
        },
    )
}

fn main() {
    pixel_engine::launch(game());
}
