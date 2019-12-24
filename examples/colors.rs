extern crate pixel_engine_gl as engine;

fn main() -> Result<(), String> {
    let mut game = engine::logic::Engine::new("Colors".to_owned(), (500, 500, 1), &game_logic);
    game.run()?;
    Ok(())
}
fn game_logic(game: &mut engine::logic::Engine) -> Result<(), String> {
    let running = true;
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
            game.screen.draw(x, y, [red, green, blue].into());
        }
    }
    while game.new_frame() && running {}
    Ok(())
}
