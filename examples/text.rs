extern crate pixel_engine_gl as engine;

fn main() -> Result<(), String> {
    let mut game = engine::Engine::new("Text".to_owned(), (500, 500, 1));
    game.screen
        .draw_text(0, 0, 1, [255, 255, 255].into(), "BONJOUR".into());
    game.run(&mut |_game| Ok(true));
    Ok(())
}
fn game_logic(_game: &mut engine::Engine) -> Result<bool, String> {
    Ok(true)
}
