extern crate pixel_engine_gl as engine;
extern crate rand;

fn main() -> Result<(), String> {
    let mut game = engine::Engine::new("Random".to_owned(), (256, 240, 2), &game_logic);
    game.run();
    Ok(())
}
fn game_logic(game: &mut engine::Engine) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    while game.new_frame() {
        for x in 0..game.size.0 {
            for y in 0..game.size.1 {
                game.screen
                    .draw(x, y, engine::Color::new(rng.gen(), rng.gen(), rng.gen()))
            }
        }
    }
}
