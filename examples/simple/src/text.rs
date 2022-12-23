extern crate pixel_engine as px;
use px::traits::*;

#[cfg(target_arch = "wasm32")]
extern crate wasm_bindgen;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

async fn init() {
    let game = px::EngineWrapper::new("Text".to_owned(), (500, 500, 2)).await;
    let textsheet = game.get_textsheet().clone();
    game.run(move |game| {
        game.clear(px::Color::BLACK);
        game.draw_sprite((0, 8), 2, &textsheet, (false, false));
        game.draw_text((0, 0), 1, [255, 255, 255].into(), "BONJOUR");
        if let Some(input) = game.try_get_finished_input() {
            println!("{input}");
        } else {
            game.start_input();
            let buffer = game.get_input_buffer().to_owned();
            let cursor = game.get_input_cursor();
            game.fill_rect(
                (8 * cursor as i32 * 2, 250i32),
                (2 * 2, 8 * 2),
                px::Color::RED,
            );
            game.draw_text((0, 250), 2, px::Color::WHITE, &buffer);
        }
        Ok(true)
    });
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]

pub fn text() {
    px::launch(init())
}
