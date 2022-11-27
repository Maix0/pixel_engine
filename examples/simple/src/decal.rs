extern crate pixel_engine as px;
use px::traits::*;
use px::vector2::*;
use px::Color;

#[cfg(target_arch = "wasm32")]
extern crate wasm_bindgen;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
async fn init() {
    let mut game = px::EngineWrapper::new("Decal".to_owned(), (50, 50, 10)).await;
    let mut spr = px::graphics::Sprite::new_with_color(10, 10, [1.0, 0.0, 0.0, 1.0f32].into());
    let mut draw_type: u8 = 1;
    let mut sub_draw_type: u8 = 1;
    let mut sub_draw_max: u8 = 4;
    let mut angle = 0.0;
    for y in (0..spr.height()).step_by(3) {
        for x in (0..spr.width()).step_by(3) {
            spr.set_pixel(x, y, Color::WHITE);
            spr.set_pixel(x + 1, y, Color::BLUE);
            spr.set_pixel(x + 2, y, Color::MAGENTA);

            spr.set_pixel(x + 1, y + 1, Color::WHITE);
            spr.set_pixel(x + 2, y + 1, Color::BLUE);
            spr.set_pixel(x, y + 1, Color::MAGENTA);

            spr.set_pixel(x + 2, y + 2, Color::WHITE);
            spr.set_pixel(x, y + 2, Color::BLUE);
            spr.set_pixel(x + 1, y + 2, Color::MAGENTA)
        }
    }
    let decal = game.create_decal(&spr);

    let mut warp = 5_f32;
    game.run(move |game: &mut px::Engine| {
        if game.get_key(px::inputs::Keycodes::Escape).any() {
            return Ok(false);
        }
        // Draw Decal: game.draw_decal
        // Draw Decal: game.draw_partial_decal
        // Draw Decal: game.draw_decal_scaled
        // Draw Decal: game.draw_partial_decal_scaled
        if game.get_key(px::inputs::Keycodes::Key1).any() {
            draw_type = 1;
            sub_draw_max = 4;
        }
        // Draw Decal: game.draw_warped_decal
        // Draw Decal: game.draw_warped_partial_decal
        if game.get_key(px::inputs::Keycodes::Key2).any() {
            draw_type = 2;
            sub_draw_max = 2;
        }
        // Draw Decal: game.draw_rotated_decal
        // Draw Decal: game.draw_rotated_decal_scaled
        // Draw Decal: game.draw_partial_rotated_decal
        // Draw Decal: game.draw_partial_rotated_decal_scaled
        if game.get_key(px::inputs::Keycodes::Key3).any() {
            draw_type = 3;
            sub_draw_max = 4;
        }
        if game.get_key(px::inputs::Keycodes::A).any() {
            sub_draw_type = 1;
        }
        if game.get_key(px::inputs::Keycodes::Z).any() {
            sub_draw_type = 2;
        }
        if game.get_key(px::inputs::Keycodes::E).any() {
            sub_draw_type = 3;
        }
        if game.get_key(px::inputs::Keycodes::R).any() {
            sub_draw_type = 4;
        }

        if game.get_key(px::inputs::Keycodes::W).any() {
            angle += 2.0 * game.elapsed as f32;
        }
        if game.get_key(px::inputs::Keycodes::X).any() {
            angle -= 2.0 * game.elapsed as f32;
        }

        if angle > 2.0 * std::f32::consts::PI {
            angle = 0.0;
        } else if angle < 0.0 {
            angle = 2.0 * std::f32::consts::PI;
        }

        sub_draw_type = sub_draw_type.clamp(1, sub_draw_max);

        game.clear([0.5, 0.5, 0.5].into());
        for y in (0..game.size.1).step_by(3) {
            for x in (0..game.size.0).step_by(3) {
                game.draw((x as i32, y as i32), Color::BLACK);
                game.draw((x as i32 + 1, y as i32), Color::YELLOW);
                game.draw((x as i32 + 2, y as i32), Color::VERY_DARK_CYAN);

                game.draw((x as i32 + 1, y as i32 + 1), Color::BLACK);
                game.draw((x as i32 + 2, y as i32 + 1), Color::YELLOW);
                game.draw((x as i32, y as i32 + 1), Color::VERY_DARK_CYAN);

                game.draw((x as i32 + 2, y as i32 + 2), Color::BLACK);
                game.draw((x as i32, y as i32 + 2), Color::YELLOW);
                game.draw((x as i32 + 1, y as i32 + 2), Color::VERY_DARK_CYAN)
            }
        }
        game.draw_rect(
            (9, 9).into(),
            {
                let a: Vu2d = decal.size().into();
                let b: Vu2d = (2, 2).into();
                let Vu2d { x, y } = a + b;
                Vi2d {
                    x: x as i32,
                    y: y as i32,
                }
            },
            Color::RED,
        );
        use px::inputs::Keycodes;
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
            1 => match sub_draw_max {
                1 => game.draw_decal((10.0, 10.0), &decal),
                2 => game.draw_decal_scaled((10.0, 10.0), &decal, (1.5, 2.0)),
                3 => game.draw_partial_decal((10.0, 10.0), &decal, (3.0, 3.0), (3.0, 3.0)),
                4 => game.draw_partial_decal_scaled(
                    (10.0, 10.0),
                    &decal,
                    (3.0, 3.0),
                    (3.0, 3.0),
                    (1.5, 2.0),
                ),

                _ => unreachable!(),
            },
            2 => match sub_draw_type {
                1 => game.draw_warped_decal(
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
                2 => game.draw_warped_partial_decal(
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
                _ => unreachable!(),
            },
            3 => match sub_draw_type {
                1 => game.draw_rotated_decal((010., 10.0), &decal, angle, (0.0, 0.0)),
                2 => game.draw_rotated_decal_scaled(
                    (010., 10.0),
                    &decal,
                    angle,
                    (0.0, 0.0),
                    (1.5, 2.5),
                ),
                3 => game.draw_partial_rotated_decal(
                    (010., 10.0),
                    &decal,
                    angle,
                    (0.0, 0.0),
                    (1.0, 1.0),
                    (3.0, 3.0),
                ),
                4 => game.draw_partial_rotated_decal_scaled(
                    (010., 10.0),
                    &decal,
                    angle,
                    (0.0, 0.0),
                    (1.0, 1.0),
                    (3.0, 3.0),
                    (1.5, 2.0),
                ),
                _ => {}
            },
            5 => {}
            6 => game.draw_decal_tinted((010., 10.0), &decal, Color::YELLOW),
            _ => {}
        };
        game.draw_text((0, 0), 1, Color::RED, &format!("{}", draw_type));
        game.draw_text(
            (8, 0),
            1,
            Color::GREEN,
            &format!("{}/{}", sub_draw_type, sub_draw_max),
        );
        Ok(true) // return Ok(false) to stop nicely and Err(_) to stop & print the error
    });
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]

pub fn decal() {
    px::launch(init())
}
