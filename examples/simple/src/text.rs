extern crate pixel_engine as px;
use px::traits::*;

#[cfg(target_arch = "wasm32")]
extern crate wasm_bindgen;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

struct Game {
    textsheet: px::Sprite,
}

impl px::Game for Game {
    fn create(engine: &mut px::Engine) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            textsheet: engine.get_textsheet().clone(),
        })
    }

    fn update(&mut self, game: &mut px::Engine) -> Result<bool, Box<dyn std::error::Error>> {
        game.clear(px::Color::BLACK);
        game.draw_sprite((0, 8), 2, &self.textsheet, (false, false));
        game.draw_text((0, 0), 1, [255, 255, 255].into(), "BONJOUR");
        game.start_input();
        let buffer = game.get_input_buffer().to_owned();
        let cursor = game.get_input_cursor();
        game.fill_rect(
            (8 * cursor as i32 * 2, 250i32),
            (2 * 2, 8 * 2),
            px::Color::RED,
        );
        game.draw_text((0, 250), 2, px::Color::WHITE, &buffer);
        Ok(true)
    }

    fn receive_input(&mut self, _engine: &mut px::Engine, input: String) {
        println!("{input}");
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn text() {
    px::start::<Game>("Text", (500, 500), 2);
}
