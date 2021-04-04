extern crate pixel_engine as engine;

use engine::traits::*;
async fn init() {
    let game = engine::EngineWrapper::new("Mode 7".to_owned(), (650 * 2, 350 * 2, 1)).await;
    let track_spr =
        engine::graphics::Sprite::load_from_file(&std::path::Path::new("spr/mariocircuit-1.png"))
            .unwrap();
    let track_layout_spr = engine::graphics::Sprite::load_from_file(&std::path::Path::new(
        "spr/layout-mariocircuit-1.png",
    ))
    .unwrap();
    let mut world = (1.02_f64, 1.02_f64);
    let mut world_a = 0.1_f64;
    let mut near = 0.005_f64;
    let mut far = 0.03_f64;
    let mut fov_half = std::f64::consts::FRAC_PI_4;
    let mut speed = 0.2;
    fn get_ground(pos: (f64, f64), t: &engine::Sprite) -> engine::Color {
        t.get_sample(pos.0 - 1.0, pos.1 - 1.0)
    }
    game.run(move |game: &mut engine::Engine| {
        use engine::inputs::Keycodes;
        if game.get_key(Keycodes::A).held {
            near += 0.1 * game.elapsed;
        };
        if game.get_key(Keycodes::Q).held {
            near -= 0.1 * game.elapsed;
        };
        if game.get_key(Keycodes::W).held {
            near = 0.005;
        };

        if game.get_key(Keycodes::Z).held {
            far += 0.1 * game.elapsed;
        };
        if game.get_key(Keycodes::S).held {
            far -= 0.1 * game.elapsed;
        };
        if game.get_key(Keycodes::X).held {
            far = 0.03;
        };

        if game.get_key(Keycodes::E).held {
            fov_half += 0.1 * game.elapsed;
        };
        if game.get_key(Keycodes::D).held {
            fov_half -= 0.1 * game.elapsed;
        };
        if game.get_key(Keycodes::C).held {
            fov_half = std::f64::consts::FRAC_PI_4;
        };

        let far1 = (
            world.0 + (world_a - fov_half).cos() * far,
            world.1 + (world_a - fov_half).sin() * far,
        );

        let near1 = (
            world.0 + (world_a - fov_half).cos() * near,
            world.1 + (world_a - fov_half).sin() * near,
        );

        let far2 = (
            world.0 + (world_a + fov_half).cos() * far,
            world.1 + (world_a + fov_half).sin() * far,
        );

        let near2 = (
            world.0 + (world_a + fov_half).cos() * near,
            world.1 + (world_a + fov_half).sin() * near,
        );

        //for y in 0..=((game.size.1 as f32 * 0.9) as _) {
        for y in 0..game.size.1 {
            let sample_depth = y as f64 / (game.size.1 as f64 / 2.0);

            let start = (
                (far1.0 - near1.0) / (sample_depth) + near1.0,
                (far1.1 - near1.1) / (sample_depth) + near1.1,
            );
            let end = (
                (far2.0 - near2.0) / (sample_depth) + near2.0,
                (far2.1 - near2.1) / (sample_depth) + near2.1,
            );

            for x in 0..game.size.0 {
                let sample_width = x as f64 / game.size.0 as f64;
                let sample = (
                    ((end.0 - start.0) * sample_width + start.0) - 1.0,
                    ((end.1 - start.1) * sample_width + start.1) - 1.0,
                );

                if sample.0 > 0.0 && sample.0 < 1.0 && sample.1 > 0.0 && sample.1 < 1.0 {
                    game.draw(
                        (x as i32, y as i32),
                        track_spr.get_sample(sample.0, sample.1),
                    );
                } else {
                    game.draw((x as i32, y as i32), [0, 0, 0].into());
                }
            }
        }
        if game.get_key(Keycodes::Left).held {
            world_a -= 1.0 * game.elapsed;
        }
        if game.get_key(Keycodes::Right).held {
            world_a += 1.0 * game.elapsed;
        }
        if game.get_key(Keycodes::Up).held {
            speed += if get_ground(world, &track_layout_spr) == engine::Color::WHITE {
                0.01
            } else {
                0.005
            };
            if speed
                > if get_ground(world, &track_layout_spr) == engine::Color::WHITE {
                    0.2
                } else {
                    0.1
                }
            {
                speed = if get_ground(world, &track_layout_spr) == engine::Color::WHITE {
                    0.2
                } else {
                    0.1
                };
            };
        }
        if game.get_key(Keycodes::Down).held {
            speed -= if get_ground(world, &track_layout_spr) == engine::Color::WHITE {
                0.005
            } else {
                0.01
            };
            if speed
                < -if get_ground(world, &track_layout_spr) == engine::Color::WHITE {
                    0.2
                } else {
                    0.1
                }
            {
                speed = -if get_ground(world, &track_layout_spr) == engine::Color::WHITE {
                    0.2
                } else {
                    0.1
                };
            };
        }

        if !(game.get_key(Keycodes::Up).held || game.get_key(Keycodes::Down).held) {
            speed += if get_ground(world, &track_layout_spr) == engine::Color::WHITE {
                0.01
            } else {
                0.01
            } * if speed < 0.0 { 1.0 } else { -1.0 };
            if speed > -0.001 || speed < 0.001 {
                speed = 0.0;
            }
        }
        world.0 += world_a.cos() * speed * game.elapsed;
        world.1 += world_a.sin() * speed * game.elapsed;
        if world.0 < 1.0
            || world.0 > 2.0
            || world.1 < 1.0
            || world.1 > 2.0
            || get_ground(world, &track_layout_spr) == engine::Color::BLACK
        {
            world.0 -= world_a.cos() * speed * game.elapsed;
            world.1 -= world_a.sin() * speed * game.elapsed;
            speed = 0.0;
        }
        /*
        game.screen.fill_rect(
            0,
            (game.size.1 as f32 * 0.9) as u32 + 1,
            9 * 8,
            18,
            engine::Color::BLACK,
        );
        game.screen.draw_text(
            0,
            (game.size.1 as f32 * 0.9) as _,
            1,
            engine::Color::WHITE,
            &format!("x: {:>4.2}", world.0),
        );
        game.screen.draw_text(
            0,
            (game.size.1 as f32 * 0.9) as u32 + 8,
            1,
            engine::Color::WHITE,
            &format!("y: {:4>.2}", world.1),
        );
        */
        Ok(true)
    });
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        use std::panic;
        panic::set_hook(Box::new(pixel_engine::console_error_panic_hook::hook));
        pixel_engine::wasm_bindgen_futures::spawn_local(init());
    };
    #[cfg(not(target_arch = "wasm32"))]
    pixel_engine::futures::executor::block_on(init());
}
