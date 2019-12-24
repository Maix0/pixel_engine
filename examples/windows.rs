extern crate pixel_engine_gl as engine;
extern crate rand;
fn main() -> Result<(), String> {
    let mut game =
        engine::logic::Engine::new("Multiples Windows".to_owned(), (1, 1, 500), &game_logic);
    game.run()?;
    Ok(())
}
fn game_logic(game: &mut engine::logic::Engine) -> Result<(), String> {
    let running = true;
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let col = engine::graphics::Color::new(rng.gen(), rng.gen(), rng.gen());
    for x in 0..game.size.0 {
        for y in 0..game.size.1 {
            game.screen.draw(x, y, col);
        }
    }
    while game.new_frame() && running {
        game.screen.draw(0, 0, col);
        if game.is_pressed(engine::keyboard::Keycodes::A) {
            main()?
        }
    }
    Ok(())
}
