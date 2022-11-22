extern crate pixel_engine_draw as px_draw;
use px_draw::{graphics::{Color, DrawSpriteTrait, Sprite}, vector2::Vu2d};

#[path = "./_print_sprite.rs"]
mod print_spr;
use print_spr::print_sprite;

fn main() {
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

    print_sprite(&spr);
}
