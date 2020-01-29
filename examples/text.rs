extern crate pixel_engine_gl as engine;

fn main() -> Result<(), String> {
    let mut game = engine::Engine::new("Text".to_owned(), (500, 500, 1), &game_logic);
    game.run();
    Ok(())
}
fn game_logic(game: &mut engine::Engine) {
    let running = true;
    game.screen
        .draw_text(0, 0, 1, [255, 255, 255].into(), "BONJOUR".into());
    while game.new_frame() && running {}
}
