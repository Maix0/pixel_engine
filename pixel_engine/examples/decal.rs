extern crate pixel_engine as engine;
use engine::traits::*;
use engine::vector2::*;
use engine::Color;
fn main() {
    let mut game = engine::EngineWrapper::new("Decal".to_owned(), (50, 50, 10));
    let mut spr = engine::graphics::Sprite::new_with_color(10, 10, [1.0, 0.0, 0.0, 1.0].into());
    let mut draw_type: u8 = 1;
    for y in (0..spr.height).step_by(3) {
        for x in (0..spr.width).step_by(3) {
            spr.set_pixel(x + 0, y, Color::WHITE);
            spr.set_pixel(x + 1, y, Color::BLUE);
            spr.set_pixel(x + 2, y, Color::MAGENTA);

            spr.set_pixel(x + 1, y + 1, Color::WHITE);
            spr.set_pixel(x + 2, y + 1, Color::BLUE);
            spr.set_pixel(x + 0, y + 1, Color::MAGENTA);

            spr.set_pixel(x + 2, y + 2, Color::WHITE);
            spr.set_pixel(x + 0, y + 2, Color::BLUE);
            spr.set_pixel(x + 1, y + 2, Color::MAGENTA)
        }
    }
    let decal = game.create_decal(&spr);

    let mut warp = 5_f32;
    game.run(move |game: &mut engine::Engine| {
        // Draw Decal: game.draw_decal
        if game.get_key(engine::inputs::Keycodes::Key1).any() {
            draw_type = 1;
        }
        // Draw Decal: game.draw_decal_scaled (1.5, 2.0)
        if game.get_key(engine::inputs::Keycodes::Key2).any() {
            draw_type = 2;
        }
        // Draw Decal: game.draw_warped_decal
        if game.get_key(engine::inputs::Keycodes::Key3).any() {
            draw_type = 3;
        }
        // Warped + Partial
        if game.get_key(engine::inputs::Keycodes::Key4).any() {
            draw_type = 4;
        }
        if game.get_key(engine::inputs::Keycodes::Key5).any() {
            draw_type = 5;
        }
        if game.get_key(engine::inputs::Keycodes::Key6).any() {
            draw_type = 6;
        }

        game.clear([0.5, 0.5, 0.5].into());
        for y in (0..game.size.1).step_by(3) {
            for x in (0..game.size.0).step_by(3) {
                game.draw((x + 0, y), Color::BLACK);
                game.draw((x + 1, y), Color::YELLOW);
                game.draw((x + 2, y), Color::VERY_DARK_CYAN);

                game.draw((x + 1, y + 1), Color::BLACK);
                game.draw((x + 2, y + 1), Color::YELLOW);
                game.draw((x + 0, y + 1), Color::VERY_DARK_CYAN);

                game.draw((x + 2, y + 2), Color::BLACK);
                game.draw((x + 0, y + 2), Color::YELLOW);
                game.draw((x + 1, y + 2), Color::VERY_DARK_CYAN)
            }
        }
        game.draw_rect(
            (9, 9).into(),
            {
                let a: Vu2d = decal.size().into();
                let b: Vu2d = (2, 2).into();
                a + b
            },
            Color::RED,
        );
        use engine::inputs::Keycodes;
        if game.get_key(Keycodes::Q).any() {
            warp += game.elapsed as f32 * 2.0;
            if warp > 11.0 {
                warp = 11.0;
            }
        }
        if game.get_key(Keycodes::D).any() {
            warp -= game.elapsed as f32 * 2.0;
            if warp < -5.0 {
                warp = -5.0;
            }
        }
        if game.get_key(Keycodes::S).any() {
            warp = 0.0;
        }

        match draw_type {
            1 => game.draw_decal((10.0, 10.0), &decal),
            2 => game.draw_decal_scaled((10.0, 10.0), &decal, (1.5, 2.0)),
            3 => game.draw_warped_decal(
                [
                    (10.0, 10.0),
                    (10.0 - warp, 10.0 + decal.size().1 as f32),
                    (
                        10.0 + decal.size().0 as f32 + warp,
                        10.0 + decal.size().0 as f32,
                    ),
                    (10.0 + decal.size().0 as f32, 10.0),
                ],
                &decal,
            ),
            4 => game.draw_warped_partial_decal(
                [
                    (10.0, 10.0),
                    (10.0 - warp, 10.0 + decal.size().1 as f32),
                    (
                        10.0 + decal.size().0 as f32 + warp,
                        10.0 + decal.size().0 as f32,
                    ),
                    (10.0 + decal.size().0 as f32, 10.0),
                ],
                (1.0, 1.0),
                (3.0, 3.0),
                &decal,
            ),
            _ => {}
        };
        game.draw_text((0, 0), 1, Color::RED, &format!("{:?}", draw_type));
        Ok(true) // return Ok(false) to stop nicely and Err(_) to stop & print the error
    });
}
