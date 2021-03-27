extern crate pixel_engine as engine;
use engine::traits::*;
fn main() {
    let game = engine::EngineWrapper::new("Lines".to_owned(), (25, 25, 20));
    let mut start = vec![0, 0];
    let mut end = vec![5u32, 5u32];
    let mut toggle = false;
    game.run(move |game: &mut engine::Engine| {
        game.clear([0, 0, 0].into());
        use engine::inputs::Keycodes::{Down, Escape, Left, Right, Space, Up};
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
            (start[0] as u32, start[1] as u32),
            (end[0] as u32, end[1] as u32),
            [1.0, 1.0, 1.0].into(),
        );

        game.draw((start[0] as u32, start[1] as u32), [0, 255, 0].into());
        game.draw((end[0] as u32, end[1] as u32), [255, 0, 0].into());
        Ok(true)
    });
}
