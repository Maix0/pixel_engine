extern crate pixel_engine as px;
use std::collections::HashSet;

#[cfg(target_arch = "wasm32")]
extern crate wasm_bindgen;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use px::traits::*;
async fn init() {
    let game = px::EngineWrapper::new("Mouse".to_owned(), (50, 50, 10)).await;
    let mut clicks: HashSet<px::vector2::Vu2d> = HashSet::new();
    let mut old_pos: px::vector2::Vu2d = (0, 0).into();
    game.run(move |game: &mut px::Engine| {
        if game.get_key(px::inputs::Keycodes::Escape).any() {
            return Ok(false);
        }
        game.clear([0, 0, 0].into());
        game.draw_line((25, 0), (25, 49), px::Color::GREEN);
        game.draw_line((0, 25), (49, 25), px::Color::GREEN);
        let mouse_pos = game.get_mouse_location();
        game.draw(mouse_pos.cast_i32(), [255, 0, 0].into());
        if game.get_mouse_btn(px::inputs::MouseBtn::Left).any() {
            if mouse_pos != old_pos {
                if clicks.contains(&mouse_pos) {
                    clicks.remove(&mouse_pos);
                } else {
                    clicks.insert(mouse_pos);
                }
            }
            old_pos = mouse_pos;
        }
        for pos in &clicks {
            game.draw(pos.cast_i32(), [0, 0, 255].into());
        }
        Ok(true)
    });
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]

pub fn mouse() {
    px::launch(init())
}
