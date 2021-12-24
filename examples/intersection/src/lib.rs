use pixel_engine::{traits::*, Color};
use pixel_engine::{
    vector2::{Vf2d, Vi2d, Vu2d},
    EngineWrapper,
};
extern crate pixel_engine;

#[cfg(target_arch = "wasm32")]
extern crate wasm_bindgen;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

const MAX_DISTANCE: f32 = 100.0;

pub async fn init() {
    let game = EngineWrapper::new("Intersection".to_string(), (512, 512, 2)).await;
    let mut player = Vf2d { x: 0.0, y: 0.0 };
    let map_size = Vi2d { x: 32, y: 30 };
    let cell_size = Vi2d { x: 16, y: 16 };
    let mut map = vec![0u8; (map_size.x * map_size.y) as usize];

    game.run(move |game| {
        let mouse = Vu2d::from(game.get_mouse_location()).cast_f32();
        let mousecell = mouse / cell_size.cast_f32();
        let cell = mousecell.cast_i32();

        if game
            .get_mouse_btn(pixel_engine::inputs::MouseBtn::Left)
            .held
        {
            map[cell.y as usize * map_size.x as usize + cell.x as usize] = 1;
        }

        if game.get_key(pixel_engine::inputs::Keycodes::Z).held {
            player.y -= 25.0 * game.elapsed as f32;
        }
        if game.get_key(pixel_engine::inputs::Keycodes::S).held {
            player.y += 25.0 * game.elapsed as f32;
        }
        if game.get_key(pixel_engine::inputs::Keycodes::Q).held {
            player.x -= 25.0 * game.elapsed as f32;
        }
        if game.get_key(pixel_engine::inputs::Keycodes::D).held {
            player.x += 25.0 * game.elapsed as f32;
        }

        player.x = player.x.clamp(0.0, map_size.x as f32);
        player.y = player.y.clamp(0.0, map_size.y as f32);

        let ray_start = player;
        let ray_dir = (mousecell - player).norm();
        let ray_unit_step_size = Vf2d {
            x: (1.0 + (ray_dir.y / ray_dir.x) * (ray_dir.y / ray_dir.x)).sqrt(),
            y: (1.0 + (ray_dir.x / ray_dir.y) * (ray_dir.x / ray_dir.y)).sqrt(),
        };
        let mut map_check = ray_start.cast_i32();
        let mut ray_length1d = Vf2d { x: 0.0, y: 0.0 };
        let mut step = Vi2d { x: 0, y: 0 };

        if ray_dir.x < 0.0 {
            step.x = -1;
            ray_length1d.x = (ray_start.x - (map_check.x as f32)) * ray_unit_step_size.x;
        } else {
            step.x = 1;
            ray_length1d.x = ((map_check.x as f32 + 1.0) - ray_start.x) * ray_unit_step_size.x;
        }

        if ray_dir.y < 0.0 {
            step.y = -1;
            ray_length1d.y = (ray_start.y - (map_check.y as f32)) * ray_unit_step_size.y;
        } else {
            step.y = 1;
            ray_length1d.y = ((map_check.y as f32 + 1.0) - ray_start.y) * ray_unit_step_size.y;
        }

        let mut tile_found = false;
        let mut distance = 0.0;
        while !tile_found && distance < MAX_DISTANCE {
            if ray_length1d.x < ray_length1d.y {
                map_check.x += step.x;
                distance = ray_length1d.x;
                ray_length1d.x += ray_unit_step_size.x;
            } else {
                map_check.y += step.y;
                distance = ray_length1d.y;
                ray_length1d.y += ray_unit_step_size.y;
            }
            // if (vMapCheck.x >= 0 && vMapCheck.x < vMapSize.x && vMapCheck.y >= 0 && vMapCheck.y < vMapSize.y)
            if (0..map_size.x).contains(&map_check.x) && (0..map_size.y).contains(&map_check.y) {
                if map[(map_check.y * map_size.x + map_check.x) as usize] == 1 {
                    tile_found = true;
                }
            }
        }

        let mut intersection: Option<Vf2d> = None;
        if tile_found {
            intersection = Some(ray_start + ray_dir * distance);
        }

        game.clear(0.into());

        for y in 0..map_size.y {
            for x in 0..map_size.x {
                let cur_cell = map[(y * map_size.x + x) as usize];
                if cur_cell == 1 {
                    game.fill_rect(Vi2d::from((x, y)) * cell_size, cell_size, Color::BLUE);
                }
                game.draw_rect(Vi2d::from((x, y)) * cell_size, cell_size, Color::DARK_GREY);
            }
        }

        if game
            .get_mouse_btn(pixel_engine::inputs::MouseBtn::Right)
            .held
        {
            game.draw_line_dotted(
                (player * cell_size.cast_f32()).cast_i32(),
                mouse.cast_i32(),
                Color::WHITE,
                0xF0F0F0F0,
            );

            if let Some(inter) = &intersection {
                game.draw_circle((*inter * cell_size.cast_f32()).cast_i32(), 4, Color::YELLOW);
            }
        }

        game.fill_circle((player * cell_size.cast_f32()).cast_i32(), 4, Color::RED);
        game.fill_circle(mouse.cast_i32(), 4, Color::GREEN);
        game.draw_text(
            (16, 480),
            1,
            Color::GREEN,
            &format!("X: {}", {
                let inter = intersection.unwrap_or(Vf2d { x: 0.0, y: 0.0 }).cast_u32();
                (player.x as i32 - inter.x as i32).abs()
            }),
        );
        game.draw_line(
            (128, 490),
            (
                128 + {
                    let inter = intersection.unwrap_or(Vf2d { x: 0.0, y: 0.0 }).cast_u32();
                    (player.x as i32 - inter.x as i32).abs()
                },
                490,
            ),
            Color::GREEN,
        );

        game.draw_text(
            (16, 500),
            1,
            Color::RED,
            &format!("X: {}", {
                let inter = intersection.unwrap_or(Vf2d { x: 0.0, y: 0.0 }).cast_u32();
                (player.y as i32 - inter.y as i32).abs()
            }),
        );

        game.draw_line(
            (128, 500),
            (
                128 + {
                    let inter = intersection.unwrap_or(Vf2d { x: 0.0, y: 0.0 }).cast_u32();
                    (player.y as i32 - inter.y as i32).abs()
                },
                500,
            ),
            Color::RED,
        );
        Ok(true)
    });
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn main() {
    pixel_engine::launch(init());
}
