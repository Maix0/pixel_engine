extern crate pixel_engine_gl as engine;

fn main() -> Result<(), String> {
    let mut game = engine::logic::Engine::new("Text".to_owned(), (100, 100, 5), &game_logic);
    game.run()?;
    Ok(())
}
fn game_logic(game: &mut engine::logic::Engine) -> Result<(), String> {
    let running = true;
    for x in 0..game.size.0 {
        let greyscale = x as f32 / game.size.0 as f32;
        game.screen.draw_line(
            (x, 0),
            (x, game.size.1),
            [greyscale, greyscale, greyscale].into(),
        );
    }
    while game.new_frame() && running {}
    Ok(())
}
