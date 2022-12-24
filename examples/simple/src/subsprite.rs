use pixel_engine::*;
use traits::*;
use vector2::*;
pub fn subsprite() {
    let spr = Sprite::new_with_color(20, 20, Color::GREEN);
    let sub1 = spr.create_sub_sprite((0, 0).into(), (10, 10).into());
    let sub2 = spr.create_sub_sprite((0, 10).into(), (10, 10).into());
    let sub3 = spr.create_sub_sprite((10, 0).into(), (10, 10).into());
    let sub4 = spr.create_sub_sprite((10, 10).into(), (10, 10).into());
    let subs = [
        (sub1, Color::BLUE),
        (sub2, Color::YELLOW),
        (sub3, Color::WHITE),
        (sub4, Color::BLACK),
    ];

    for (sub, col) in subs {
        let mut sub = sub.unwrap();
        for y in 0..10u32 {
            for x in 0..10u32 {
                sub.set_pixel(Vu2d { x, y }, col);
            }
        }
    }

    launch(async move {
        let engine = EngineWrapper::new("SubSprite".into(), (50, 50, 15)).await;
        let spr_clone = spr.clone();
        engine.run(move |game: &mut Engine| {
            for y in (0..game.size().y).step_by(3) {
                for x in (0..game.size().x).step_by(3) {
                    game.draw((x as i32, y as i32), Color::MAGENTA);
                    game.draw((x as i32 + 1, y as i32), Color::GREEN);
                    game.draw((x as i32 + 2, y as i32), Color::VERY_DARK_CYAN);

                    game.draw((x as i32 + 1, y as i32 + 1), Color::MAGENTA);
                    game.draw((x as i32 + 2, y as i32 + 1), Color::GREEN);
                    game.draw((x as i32, y as i32 + 1), Color::VERY_DARK_CYAN);

                    game.draw((x as i32 + 2, y as i32 + 2), Color::MAGENTA);
                    game.draw((x as i32, y as i32 + 2), Color::GREEN);
                    game.draw((x as i32 + 1, y as i32 + 2), Color::VERY_DARK_CYAN)
                }
            }
            game.draw_sprite(Vi2d { x: 5, y: 5 }, 1, &spr, (false, false));
            game.draw_sprite(Vi2d { x: 15, y: 15 }, 1, &spr_clone, (false, false));
            Ok(true)
        });
    })
}
