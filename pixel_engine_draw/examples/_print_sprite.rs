#![allow(dead_code)]

extern crate pixel_engine_draw as px_draw;
use px_draw::graphics::{Color, Sprite};

fn main() {
    println!("This is a library to print sprite to the console (crudely)");
}

const RECT: char = ' ';

pub fn print_sprite(spr: &Sprite) {
    println!();
    for y in 0..spr.height() {
        for x in 0..spr.width() {
            let Color { r, g, b, .. } = spr.get_pixel(x, y);
            print!("\x1b[48;2;{r};{g};{b}m{RECT}{RECT}");
        }
        println!("\x1b[0m");
    }
}
