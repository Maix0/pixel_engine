extern crate pixel_engine as px;

use px::graphics::Color;
use px::traits::*;
use px::vector2::*;
fn main() {
    px::launch(game());
}

async fn game() {
    let mut wrapper = px::EngineWrapper::new("Decal Dungeon".to_string(), (1024, 1024, 2)).await;
    wrapper.clear(Color::BLACK);
    wrapper.run(|engine: &mut px::Engine| {
        engine.clear(Color::BLACK);
        Ok(true)
    });
}
