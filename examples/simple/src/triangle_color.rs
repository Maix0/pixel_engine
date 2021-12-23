extern crate pixel_engine as px;
use px::traits::*;

#[cfg(target_arch = "wasm32")]
extern crate wasm_bindgen;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
async fn init() {
    let mut game = px::EngineWrapper::new("Triangle".to_string(), (500, 500, 1)).await;
    fn dist(p1: (i32, i32), p2: (i32, i32)) -> f64 {
        return (((p2.0 - p1.0).pow(2) + (p2.1 - p1.1).pow(2)) as f64).sqrt();
    }
    let base = 0.95f32;
    let green = (
        (game.size.0 as f32 * (1f32 - base)) as i32,
        (game.size.1 as f32 * base) as i32,
    );
    let blue = (
        (game.size.0 as f32 * base) as i32,
        (game.size.1 as f32 * base) as i32,
    );
    let red: (i32, i32) = (
        (game.size.0 as f32 * 0.50f32) as i32,
        (game.size.1 as f32 * base) as i32
            - (dist(green, blue).powi(2) - (dist(green, blue) / 2f64).powi(2)).sqrt() as i32,
    );

    let lines = (
        (
            // 1 = red & 2 = green
            green.1 - red.1,                                       // a
            red.0 - green.0,                                       // b
            (green.1 - red.1) * red.0 + (red.0 - green.0) * red.1, // c
        ),
        (
            // 1 = green & 2 = blue
            blue.1 - green.1,
            green.0 - blue.0,
            (blue.1 - green.1) * green.0 + (green.0 - blue.0) * green.1,
        ),
        (
            // 1 = blue & 2 = red
            red.1 - blue.1,
            blue.0 - red.0,
            (red.1 - blue.1) * blue.0 + (blue.0 - red.0) * blue.1,
        ),
    );
    for x in 0..(game.size.0) {
        for y in 0..(game.size.1) {
            if (lines.0).0 * x as i32 + (lines.0).1 * y as i32 >= (lines.0).2
                && (lines.1).0 * x as i32 + (lines.1).1 * y as i32 >= (lines.1).2
                && (lines.2).0 * x as i32 + (lines.2).1 * y as i32 >= (lines.2).2
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
    game.run(|_| Ok(true));
}
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn triangle_color() {
    px::launch(init())
}
