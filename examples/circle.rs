extern crate pixel_engine_gl as engine;

fn main() -> Result<(), String> {
    let mut game = engine::logic::Engine::new("Circle".to_owned(), (501, 501, 1), &game_logic);
    game.run()?;
    println!("request drop!");
    game.stop();
    println!("droped");
    Ok(())
}
fn game_logic(game: &mut engine::logic::Engine) {
    let mut running = true;
    game.screen.clear(engine::graphics::Color::WHITE);
    game.screen
        .draw_circle(250, 250, 250, engine::graphics::Color::BLACK);
    game.screen
        .fill_circle(250, 255, 125, engine::graphics::Color::BLUE);
    let mut timer = 0f64;
    while game.new_frame() && running {
        //dbg!(timer);
        timer += game.elapsed;
        if timer > 2.5 {
            //running = false;
        }
    }
    println!("closed SOON");
    return;
}
