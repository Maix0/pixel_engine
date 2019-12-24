extern crate pixel_engine_gl as engine;

fn main() -> Result<(), String> {
    let mut game = engine::logic::Engine::new("Text".to_owned(), (25, 25, 20), &game_logic);
    game.run()?;
    Ok(())
}
fn game_logic(game: &mut engine::logic::Engine) -> Result<(), String> {
    let running = true;
    let mut start = vec![0, 0];
    let mut end = vec![5u32, 5u32];
    let mut toggle = false;
    while game.new_frame() && running {
        /*for x in 0..game.size.0 {
            for y in 0..game.size.1 {
                game.screen.draw(x, y, [0.5, 0.5, 0.5].into())
            }
        }*/
        if game.is_pressed(A) {
            game.screen
                .screenshot(std::path::Path::new("./screenshot.png"));
        }
        //game.screen.clear([1, 1, 1].into());
        game.screen.clear([0, 0, 0].into());
        use engine::keyboard::Keycodes::{Down, Left, Right, Space, Up, A};
        // END POINT CONTROL

        if game.is_pressed(Space) {
            toggle = !toggle;
        }
        if toggle {
            if (game.is_pressed(Left) || game.is_held(Left)) && end[0] > 0 {
                end[0] -= 1;
            }
            if game.is_pressed(Right) || game.is_held(Right) {
                end[0] += 1;
            }
            if game.is_pressed(Down) || game.is_held(Down) {
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
            if game.is_pressed(Right) || game.is_held(Right) {
                start[0] += 1;
            }
            if game.is_pressed(Down) || game.is_held(Down) {
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
    }
    Ok(())
}
