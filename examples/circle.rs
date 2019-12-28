extern crate pixel_engine_gl as engine;

fn main() {
    let mut game = engine::Engine::new("Circle".to_owned(), (51, 51, 10), &game_logic);
    game.run();
    game.stop();
}
fn game_logic(game: &mut engine::Engine) {
    let running = true;
    while game.new_frame() && running {
        game.screen.clear(engine::Color::WHITE);
        game.screen.draw_circle(25, 25, 25, engine::Color::BLACK);
        game.screen.fill_circle(25, 25, 12, engine::Color::BLUE);
    }
}
