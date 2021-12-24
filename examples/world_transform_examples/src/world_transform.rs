extern crate pixel_engine as px;
extern crate world_transform;
use px::traits::*;
use px::vector2::*;



#[cfg(target_arch = "wasm32")]
extern crate wasm_bindgen;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

async fn init() {
    let game = px::EngineWrapper::new("World Transform".to_owned(), (500, 500, 1)).await;

    let mut selected_cell: Vi2d = (0, 0).into();

    let mut transform = world_transform::Transform::new(&game, (1f32, 1f32).into());

    game.run(move |game| {
        game.clear(px::Color::BLACK);
        transform.handle_pan(
            game,
            |game| game.get_mouse_btn(px::inputs::MouseBtn::Middle).pressed,
            |game| game.get_mouse_btn(px::inputs::MouseBtn::Middle).held,
        );
        transform.handle_zoom(
            game,
            |game| game.get_key(px::inputs::Keycodes::A).any(),
            |game| game.get_key(px::inputs::Keycodes::E).any(),
        );

        for y in 0..=10 {
            let y = y as f32;
            if transform.is_visible_y(y) {
                let (start, end): (Vf2d, Vf2d) = ((0.0f32, y).into(), (10.0f32, y).into());

                let start_pixel = transform.world_to_screen(start);
                let end_pixel = transform.world_to_screen(end);

                game.draw_line(start_pixel, end_pixel, px::Color::WHITE);
            }
        }

        for x in 0..=10 {
            let x = x as f32;
            if transform.is_visible_x(x) {
                let (start, end): (Vf2d, Vf2d) = ((x, 0f32).into(), (x, 10f32).into());

                let start_pixel = transform.world_to_screen(start);
                let end_pixel = transform.world_to_screen(end);

                game.draw_line(start_pixel, end_pixel, px::Color::WHITE);
            }
        }
        
        /*
        let center_selected = transform.world_to_screen();
        let cr = 0.3 * transform.scale().x;

        game.fill_circle(center_selected, cr as u32, px::Color::RED);

        if game.get_mouse_btn(px::inputs::MouseBtn::Left).released {
            selected_cell = center_selected;
        }*/

        Ok(true)
    });
}
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn world_transform() {
    px::launch(init())
}
