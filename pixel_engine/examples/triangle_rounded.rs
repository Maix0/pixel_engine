extern crate pixel_engine as engine;
use engine::traits::*;
async fn init() {
    let mut game = engine::EngineWrapper::new("Triangle".to_string(), (550, 550, 1)).await;
    fn dist(p1: (i32, i32), p2: (i32, i32)) -> f64 {
        return (((p2.0 - p1.0).pow(2) + (p2.1 - p1.1).pow(2)) as f64).sqrt();
    }
    let base = 0.95;
    let offset = (game.size.1 as f32 * 0.05) as i32;
    let green = (
        (game.size.0 as f32 * (1f32 - base)) as i32,
        ((game.size.1 as f32 * base) - 100f32 * base) as i32 + offset,
    );
    let blue = (
        (game.size.0 as f32 * base) as i32,
        ((game.size.1 as f32 * base) - 100f32 * base) as i32 + offset,
    );
    let red: (i32, i32) = (
        (game.size.0 as f32 * 0.50f32) as i32,
        ((game.size.1 as f32 * base) - 100f32 * base) as i32
            - (dist(green, blue).powi(2) - (dist(green, blue) / 2f64).powi(2)).sqrt() as i32
            + offset,
    );
    let max_dist = dist(red, blue);
    for x in 0..(game.size.0) {
        for y in 0..(game.size.1) {
            let point = (x as i32, y as i32);
            if dist(point, red) < max_dist
                && dist(point, blue) < max_dist
                && dist(point, green) < max_dist
            {
                game.draw(
                    (x as i32, y as i32),
                    engine::Color::new(
                        ((1f64 - dist((x as i32, y as i32), red) / dist(red, blue)) * 255f64) as u8,
                        ((1f64 - dist((x as i32, y as i32), green) / dist(red, blue)) * 255f64)
                            as u8,
                        ((1f64 - dist((x as i32, y as i32), blue) / dist(red, blue)) * 255f64)
                            as u8,
                    ),
                )
            } else {
                game.draw((x as i32, y as i32), engine::Color::BLACK)
            }
        }
    }
    game.run(|_| Ok(true));
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
