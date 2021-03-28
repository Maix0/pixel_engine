extern crate pixel_engine as engine;
use engine::inputs::Keycodes as Keycode;
use engine::traits::*;
use engine::*;
use std::f64::consts::PI;
//use std::io::BufRead;
pub mod maps;
struct Player {
    angle: f64,
    x: f64,
    y: f64,
    fov: f64,
    speed: f64,
    depth: f64,
}

const MMF: i32 = 4; // Minimap factor

impl Player {
    fn new() -> Player {
        Player {
            angle: 0_f64,
            x: 2_f64,
            y: 2_f64,
            fov: 3.14159_f64 / 4.0_f64,
            depth: 16.0_f64,
            speed: 5.0_f64,
        }
    }
}

fn main() -> Result<(), String> {
    let fac = 4;
    let mut game = EngineWrapper::new("Pixel FPS".to_owned(), (120 * fac, 60 * fac, 10 / fac));
    // =======================
    let viewport = (game.get_inner().size.0, 7 * game.get_inner().size.1 / 8);
    let mut player = Player::new();
    let mut map = maps::WorldConstructor::load_file(String::from("maps/dev.map"))
        .unwrap()
        .to_world();
    map.load_all().unwrap();
    let mut current_tile: char = '#';

    // =======================
    game.run(move |game: &mut Engine| {
        game.screen.clear(Color::BLACK);
        // WRITE YOUR CODE HERE
        if game.get_key(Keycode::Q).held {
            // TURN TO THE LEFT
            player.angle -= (player.speed * 0.75_f64) * game.elapsed;
        }
        if game.get_key(Keycode::D).held {
            // TURN TO THE RIGHT
            player.angle += (player.speed * 0.75_f64) * game.elapsed;
        }
        if game.get_key(Keycode::Z).held {
            // MOVE FORWARD
            player.x += player.angle.sin() * player.speed * game.elapsed;
            player.y += player.angle.cos() * player.speed * game.elapsed;
            if map.get_2d(player.x as i64, player.y as i64) != Some('.') {
                player.x -= player.angle.sin() * player.speed * game.elapsed;
                player.y -= player.angle.cos() * player.speed * game.elapsed;
            }
        }
        if game.get_key(Keycode::S).held {
            // MOCE BACKWARD
            player.x -= player.angle.sin() * player.speed * game.elapsed;
            player.y -= player.angle.cos() * player.speed * game.elapsed;
            if map.get_2d(player.x as i64, player.y as i64) != Some('.') {
                player.x += player.angle.sin() * player.speed * game.elapsed;
                player.y += player.angle.cos() * player.speed * game.elapsed;
            }
        }
        if game.get_key(Keycode::A).held {
            // MOVE LEFT
            player.x -= player.angle.cos() * player.speed * game.elapsed;
            player.y += player.angle.sin() * player.speed * game.elapsed;
            if map.get_2d(player.x as i64, player.y as i64) != Some('.') {
                player.x += player.angle.cos() * player.speed * game.elapsed;
                player.y -= player.angle.sin() * player.speed * game.elapsed;
            }
        }
        if game.get_key(Keycode::E).held {
            // MOVE RIGHT
            player.x += player.angle.cos() * player.speed * game.elapsed;
            player.y -= player.angle.sin() * player.speed * game.elapsed;
            if map.get_2d(player.x as i64, player.y as i64) != Some('.') {
                player.x -= player.angle.cos() * player.speed * game.elapsed;
                player.y += player.angle.sin() * player.speed * game.elapsed;
            }
        }
        if game.get_key(Keycode::Escape).any() {
            return Ok(false);
        }

        for x in 0..=(viewport.0) {
            let ray_angle =
                (player.angle - player.fov / 2.0_f64) + (x as f64 / viewport.0 as f64) * player.fov;
            let mut wall_distance = 0_f64;
            let stepsize = 0.1_f64;
            let mut hit_wall = false;
            let eye_x = (&ray_angle).sin();
            let eye_y = (&ray_angle).cos();
            let mut sample_x = -0.1_f64;
            while !hit_wall && wall_distance < player.depth {
                wall_distance += stepsize;

                // CORDINATES OF CURRENT TESTED CELL AS i64
                let test_x = (player.x + eye_x * wall_distance).floor() as i64;
                let test_y = (player.y + eye_y * wall_distance).floor() as i64;

                if test_x < 0
                    || test_x >= map.map.w as i64
                    || test_y < 0
                    || test_y >= map.map.h as i64
                {
                    hit_wall = true;
                    wall_distance = player.depth;
                    sample_x = -1.0;
                } else {
                    if map.get_2d(test_x, test_y) != Some('.') {
                        hit_wall = true;
                        current_tile = map.get_2d(test_x, test_y).unwrap();
                        // MIDDLE OF WALL AS f64
                        let mid_x = test_x as f64 + 0.5_f64;
                        let mid_y = test_y as f64 + 0.5_f64;

                        let test_point_x = player.x + eye_x * wall_distance;
                        let test_point_y = player.y + eye_y * wall_distance;

                        let test_angle =
                            (test_point_y as f64 - mid_y).atan2(test_point_x as f64 - mid_x);

                        if test_angle >= -PI * 0.25_f64 && test_angle < PI * 0.25_f64 {
                            sample_x = test_point_y - (test_y as f64);
                        } else if test_angle >= PI * 0.25_f64 && test_angle < PI * 0.75_f64 {
                            sample_x = test_point_x - (test_x as f64);
                        } else if test_angle < -PI * 0.25_f64 && test_angle >= -PI * 0.75_f64 {
                            sample_x = test_point_x - (test_x as f64);
                        } else if test_angle >= PI * 0.75_f64 || test_angle < -PI * 0.75_f64 {
                            sample_x = test_point_y - (test_y as f64);
                        } else {
                            sample_x = -1.0_f64
                        }
                    }
                }
            }
            let ceiling =
                ((viewport.1 as f64 / 2.0) as f64 - viewport.1 as f64 / wall_distance) as i64;
            let floor = (viewport.1 as i64 - ceiling) as i64;

            for y in 0..=(viewport.1) {
                if y as i64 <= ceiling {
                    // CEILING
                    game.screen.draw(x as u32, y as u32, Color::BLACK);
                } else if y as i64 > ceiling && y as i64 <= floor {
                    // WALL
                    let sample_y =
                        ((y as f64) - (ceiling as f64)) / ((floor as f64) - (ceiling as f64));
                    let color = if let Some(tile) = map.tiles.get(&current_tile)
                    //== Some(tile)
                    {
                        tile.sprite.as_ref().unwrap().get_sample(sample_x, sample_y)
                    } else {
                        Color::GREEN
                    };
                    game.screen.draw(x as u32, y as u32, color);
                /*match wall.get_sample(sample_x, sample_y) {
                    engine::Color::WHITE => {
                        println!("WHITE PIXEL DRAWN TO SCREEN!");
                    }
                    _ => {}
                };*/
                } else {
                    // FLOOR
                    game.screen.draw(x as u32, y as u32, Color::DARK_GREEN);
                }
            }
        }
        for ny in 0..map.map.h {
            for nx in 0..map.map.w {
                match map.get_2d(nx as i64, ny as i64) {
                    Some('.') => {
                        game.screen.fill_rect(
                            (nx as u32 * MMF as u32) + MMF as u32,
                            (ny as u32 * MMF as u32) + MMF as u32,
                            MMF as u32,
                            MMF as u32,
                            Color::BLACK,
                        );
                    }
                    _ => {
                        game.screen.fill_rect(
                            (nx as u32 * MMF as u32) + MMF as u32,
                            (ny as u32 * MMF as u32) + MMF as u32,
                            MMF as u32,
                            MMF as u32,
                            Color::RED,
                        );
                    }
                }
                game.screen.fill_rect(
                    (player.x as u32 * MMF as u32) + MMF as u32,
                    (player.y as u32 * MMF as u32) + MMF as u32,
                    MMF as u32,
                    MMF as u32,
                    Color::GREEN,
                );
            }
        }
        game.screen.draw_text(
            0,
            game.size.1 - 18,
            2,
            [255, 255, 255].into(),
            &format!("{:.5}", game.elapsed),
        );
        return Ok(true);
    });
    Ok(())
}
