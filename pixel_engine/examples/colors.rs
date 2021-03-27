extern crate pixel_engine as engine;
use engine::traits::*;

fn main() {
    let mut game = engine::EngineWrapper::new("Colors".to_owned(), (500, 500, 1));

    for x in 0..game.size.0 {
        for y in 0..game.size.1 {
            let red: f32 = if x < (game.size.0 - 1) / 2 {
                1f32 - x as f32 / ((game.size.0 - 1) / 2) as f32
            } else {
                0f32
            };
            let green: f32 = 1f32
                - ((x as f32 - ((game.size.0) / 2) as f32).abs() / ((game.size.0) / 2) as f32)
                    as f32;
            let blue: f32 = if x > (game.size.0 - 1) / 2 {
                x as f32 / ((game.size.0 - 1) / 2) as f32
            } else {
                0f32
            };
            game.draw((x, y), [red, green, blue].into());
        }
    }
    game.run(|_| Ok(true));
}
