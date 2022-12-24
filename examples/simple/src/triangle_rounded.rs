extern crate pixel_engine as px;
use px::traits::*;

#[cfg(target_arch = "wasm32")]
extern crate wasm_bindgen;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
async fn init() {
    let mut game = px::EngineWrapper::new("Triangle".to_string(), (550, 550, 1)).await;
    fn dist(p1: (i32, i32), p2: (i32, i32)) -> f64 {
        (((p2.0 - p1.0).pow(2) + (p2.1 - p1.1).pow(2)) as f64).sqrt()
    }
    let base = 0.95;
    let offset = (game.size().y as f32 * 0.05) as i32;
    let green = (
        (game.size().x as f32 * (1f32 - base)) as i32,
        ((game.size().y as f32 * base) - 100f32 * base) as i32 + offset,
    );
    let blue = (
        (game.size().x as f32 * base) as i32,
        ((game.size().y as f32 * base) - 100f32 * base) as i32 + offset,
    );
    let red: (i32, i32) = (
        (game.size().x as f32 * 0.50f32) as i32,
        ((game.size().y as f32 * base) - 100f32 * base) as i32
            - (dist(green, blue).powi(2) - (dist(green, blue) / 2f64).powi(2)).sqrt() as i32
            + offset,
    );
    let max_dist = dist(red, blue);
    for x in 0..(game.size().x) {
        for y in 0..(game.size().y) {
            let point = (x as i32, y as i32);
            if dist(point, red) < max_dist
                && dist(point, blue) < max_dist
                && dist(point, green) < max_dist
            {
                game.draw(
                    (x as i32, y as i32),
                    px::Color::new(
                        ((1f64 - dist((x as i32, y as i32), red) / dist(red, blue)) * 255f64) as u8,
                        ((1f64 - dist((x as i32, y as i32), green) / dist(red, blue)) * 255f64)
                            as u8,
                        ((1f64 - dist((x as i32, y as i32), blue) / dist(red, blue)) * 255f64)
                            as u8,
                    ),
                )
            } else {
                game.draw((x as i32, y as i32), px::Color::BLACK)
            }
        }
    }
    game.run(move |game: &mut px::Engine| {
        return Ok(game.get_key(px::inputs::Keycodes::Escape).any());
    });
}
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn triangle_rounded() {
    px::launch(init())
}
