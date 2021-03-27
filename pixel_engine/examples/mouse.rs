extern crate pixel_engine as engine;
use std::collections::HashSet;

use engine::traits::*;
fn main() {
    let game = engine::EngineWrapper::new("Mouse".to_owned(), (50, 50, 10));
    let mut clicks: HashSet<(u32, u32)> = HashSet::new();
    let mut old_pos = (0, 0);
    game.run(move |game: &mut engine::Engine| {
        if game.get_key(engine::inputs::Keycodes::Escape).any() {
            return Ok(false);
        }
        game.clear([0, 0, 0].into());
        game.draw_line((25, 0), (25, 49), engine::Color::GREEN);
        game.draw_line((0, 25), (49, 25), engine::Color::GREEN);
        let mouse_pos = game.get_mouse_location();
        game.draw((mouse_pos.0, mouse_pos.1), [255, 0, 0].into());
        if game.get_mouse_btn(engine::inputs::MouseBtn::Left).any() {
            if old_pos.0 != mouse_pos.0 || old_pos.1 != mouse_pos.1 {
                if clicks.contains(&mouse_pos) {
                    clicks.remove(&mouse_pos);
                } else {
                    clicks.insert(mouse_pos);
                }
            }
            old_pos = mouse_pos;
        }
        for (x, y) in &clicks {
            game.draw((*x, *y), [0, 0, 255].into());
        }
        Ok(true)
    });
}
