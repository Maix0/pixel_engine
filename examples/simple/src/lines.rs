extern crate pixel_engine as px;
use px::traits::*;

#[cfg(target_arch = "wasm32")]
extern crate wasm_bindgen;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
async fn init() {
    let game = px::EngineWrapper::new("Lines".to_owned(), (25, 25, 20)).await;
    let mut start = vec![0, 0];
    let mut end = vec![5u32, 5u32];
    let mut toggle = false;
    game.run(move |game: &mut px::Engine| {
        game.clear([0, 0, 0].into());
        use px::inputs::Keycodes::{Down, Escape, Left, Right, Space, Up};
        // END POINT CONTROL
        if game.get_key(Space).pressed {
            toggle = !toggle;
        }
        if game.get_key(Escape).pressed {
            return Ok(false);
        }
        if toggle {
            if game.get_key(Left).any() && end[0] > 0 {
                end[0] -= 1;
            }
            if game.get_key(Right).any() && end[0] < game.size.0 - 1 {
                end[0] += 1;
            }
            if game.get_key(Down).any() && end[1] < game.size.1 - 1 {
                end[1] += 1;
            }
            if game.get_key(Up).any() && end[1] > 0 {
                end[1] -= 1;
            }
        } else {
            // START POINT CONTROL
            if game.get_key(Left).any() && start[0] > 0 {
                start[0] -= 1;
            }
            if game.get_key(Right).any() && start[0] < game.size.0 - 1 {
                start[0] += 1;
            }
            if game.get_key(Down).any() && start[1] < game.size.1 - 1 {
                start[1] += 1;
            }
            if game.get_key(Up).any() && start[1] > 0 {
                start[1] -= 1;
            }
        }
        game.draw_line(
            (start[0] as i32, start[1] as i32),
            (end[0] as i32, end[1] as i32),
            [1.0, 1.0, 1.0].into(),
        );

        game.draw((start[0] as i32, start[1] as i32), [0, 255, 0].into());
        game.draw((end[0] as i32, end[1] as i32), [255, 0, 0].into());
        Ok(true)
    });
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]

pub fn lines() {
    px::launch(init())
}
