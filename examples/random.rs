extern crate pixel_engine_gl as engine;
extern crate rand;

fn main() -> Result<(), String> {
    let mut game = engine::logic::Engine::new("Random".to_owned(), (500, 500, 1), &game_logic);
    game.run();
    Ok(())
}
fn game_logic(game: &mut engine::logic::Engine) -> Result<(), String> {
    let mut running = true;
    running = true;
    use rand::Rng;
    let mut rng = rand::thread_rng();
    for x in 0..game.size.0 {
        for y in 0..game.size.1 {
            game.screen.draw(
                x,
                y,
                engine::graphics::Color::new(rng.gen(), rng.gen(), rng.gen()),
            )
        }
    }
    while game.new_frame() && running {}
    Ok(())
}
