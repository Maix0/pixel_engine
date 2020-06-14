extern crate pixel_engine_gl as engine;

fn main() {
    let mut game = engine::Engine::new("Text".to_owned(), (25, 25, 20));
    let mut start = vec![0, 0];
    let mut end = vec![5u32, 5u32];
    let mut toggle = false;
    game.run(&mut |game: &mut engine::Engine| {
        game.screen.clear([0, 0, 0].into());
        use engine::keyboard::Keycodes::{Down, Left, Right, Space, Up};
        // END POINT CONTROL
        if game.is_pressed(Space) {
            toggle = !toggle;
        }
        if toggle {
            if (game.is_pressed(Left) || game.is_held(Left)) && end[0] > 0 {
                end[0] -= 1;
            }
            if (game.is_pressed(Right) || game.is_held(Right)) && end[0] < game.size.0 - 1 {
                end[0] += 1;
            }
            if (game.is_pressed(Down) || game.is_held(Down)) && end[1] < game.size.1 - 1 {
                end[1] += 1;
            }
            if (game.is_pressed(Up) || game.is_held(Up)) && end[1] > 0 {
                end[1] -= 1;
            }
        } else {
            // START POINT CONTROL
            if (game.is_pressed(Left) || game.is_held(Left)) && start[0] > 0 {
                start[0] -= 1;
            }
            if (game.is_pressed(Right) || game.is_held(Right)) && start[0] < game.size.0 - 1 {
                start[0] += 1;
            }
            if (game.is_pressed(Down) || game.is_held(Down)) && start[1] < game.size.1 - 1 {
                start[1] += 1;
            }
            if (game.is_pressed(Up) || game.is_held(Up)) && start[1] > 0 {
                start[1] -= 1;
            }
        }
        game.screen.draw_line(
            (start[0] as u32, start[1] as u32),
            (end[0] as u32, end[1] as u32),
            [1.0, 1.0, 1.0].into(),
        );

        game.screen
            .draw(start[0] as u32, start[1] as u32, [0, 255, 0].into());
        game.screen
            .draw(end[0] as u32, end[1] as u32, [255, 0, 0].into());
        Ok(true)
    });
}
