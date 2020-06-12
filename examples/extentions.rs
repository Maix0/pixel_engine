use pixel_engine_gl;
use pixel_engine_gl::*;
use pxengine_ext_test;
use pxengine_ext_test::*;

fn main() {
    let mut game = Engine::new(String::from("A window title"), (10, 10, 50), &game_logic);
    game.run();
}

fn game_logic(game: &mut Engine) {
    while game.new_frame() {
        game.screen.draw_red(0, 0);
        game.screen.draw_green(2, 5);
        game.screen.draw_blue(5, 8);
    }
}
