#![allow(dead_code)]
extern crate pixel_engine as engine;
extern crate ron;
extern crate serde;

#[path = "../../maps.rs"]
mod maps;
use engine::inputs::Keycodes as Keycode;
use engine::traits::*;
fn sprite_frame(game: &mut engine::Engine, spr: &Option<engine::Sprite>) {
    if let Some(spr) = spr {
        for x in 0..257 {
            for y in 0..257 {
                game.draw(
                    (x + 5, y + 305),
                    spr.get_sample(x as f64 / 256_f64, y as f64 / 256_f64),
                )
            }
        }
    } else {
        game.draw_line((6, 306), (6 + 254, 306 + 254), engine::Color::WHITE);
        game.draw_line((6, 306 + 254), (6 + 254, 306), engine::Color::WHITE);
    }
}

async fn init() {
    let game = engine::EngineWrapper::new("FPS Map Editor".to_owned(), (600, 600, 1)).await;
    let args: Vec<_> = std::env::args().collect();
    //let mut c_world = WorldConstructor::new();
    let mut c_world: maps::WorldConstructor;
    if args.len() > 1 {
        c_world = maps::WorldConstructor::load_file(args[1].clone()).unwrap();
    } else {
        panic!("Filename required");
    }
    let mut typing = false;
    let mut typed_string = String::new();
    let mut finished_string = String::new();
    std::fs::write(&args[1], ron::ser::to_string(&c_world.to_world()).unwrap()).unwrap();
    for tile in &mut c_world.tiles {
        tile.1.load().unwrap();
    }
    let selected_tile: Option<&maps::Tile> = None;
    let mut selected_tile_index = 0;

    let mut add_tile_t: maps::Tile = maps::Tile {
        sprite: None,
        sprite_path: String::new(),
        chr: '\u{0000}',
    };
    let mut add_tile = false;
    let mut add_tile_chr_buf: String = String::new();
    let mut add_tile_field = 0;

    // MAP EDIT VARS
    let mut c_tile_x: f64 = 0.0;
    let mut c_tile_y: f64 = 0.0;
    // END
    game.run(move |game: &mut engine::Engine| {
        if game.get_key(Keycode::Escape).any() {
            return Ok(false);
        }
        if typing {
            let keys = game.get_pressed();
            for key in &keys {
                if *key == Keycode::Return || *key == Keycode::NumpadEnter {
                    typing = false;
                    finished_string = typed_string.clone();
                    typed_string = String::new();
                } else if *key == Keycode::Return {
                    typed_string.pop();
                } else {
                    typed_string += &normalize(*key);
                }
            }
        }
        if !typing && !add_tile && game.get_key(Keycode::P).pressed {
            selected_tile_index += 1;
            if selected_tile_index > c_world.tiles.len() {
                selected_tile_index = 0;
            }
        }
        if !typing && !add_tile && game.get_key(Keycode::M).pressed {
            if selected_tile_index > 0 {
                selected_tile_index -= 1;
            }
            if selected_tile_index > c_world.tiles.len() {
                selected_tile_index = 0;
            }
        }

        if !typing && finished_string.len() > 0 {
            println!("finished === {}", finished_string);
            finished_string = String::new();
        }

        game.clear(engine::Color::BLACK);
        game.draw_rect((5, 5), (590, 295), engine::Color::RED);
        game.draw_rect((5, 305), (257, 256), engine::Color::BLUE);
        game.draw_rect((5, 566), (257, 29), engine::Color::WHITE);
        game.draw_rect((266, 305), (329, 290), engine::Color::GREEN);
        //game
        //    .draw_string(0, 0, typed_string.clone(), engine::Color::WHITE, 1)?;

        // HANDLE MAP VIEW + EDIT

        let offset_x = 8;
        let offset_y = 8;
        let mut index_x;
        let mut index_y = 0;
        if !typing {
            if game.get_key(Keycode::Left).any() {
                if c_tile_x != 0.0 {
                    c_tile_x -= 10.0 * game.elapsed;
                }
                if c_tile_x < 0.0 {
                    c_tile_x = 0.0;
                }
            }
            if game.get_key(Keycode::Right).any() {
                c_tile_x += 10.0 * game.elapsed;
                if c_tile_x > 72.0 {
                    c_tile_x = 72.0;
                }
            }
            if game.get_key(Keycode::Up).any() {
                if c_tile_y != 0.0 {
                    c_tile_y -= 10.0 * game.elapsed;
                }
                if c_tile_y < 0.0 {
                    c_tile_y = 0.0;
                }
            }
            if game.get_key(Keycode::Down).any() {
                c_tile_y += 10.0 * game.elapsed;
                if c_tile_y > 35.0 {
                    c_tile_y = 35.0;
                }
            }
            if game.get_key(Keycode::Space).any() {
                let selected_char = match &selected_tile {
                    Some(t) => t.chr,
                    None => '.',
                };
                c_world.map_set_y(c_tile_y as usize + 1);
                c_world.map_set_x(c_tile_x as usize);
                c_world.map_set(c_tile_x as usize, c_tile_y as usize, selected_char);
            }
        }
        game.fill_rect(
            (
                (offset_x + 8 * c_tile_x as usize) as i32,
                (offset_y + 8 * c_tile_y as usize) as i32,
            ),
            (8, 8),
            engine::Color::GREY,
        );
        for row in &mut c_world.map {
            index_x = 0;
            for chr in row.chars() {
                if index_y == c_tile_y as usize && index_x == c_tile_x as usize {
                    game.draw_text(
                        (
                            (offset_x + 8 * index_x) as i32,
                            (offset_y + 8 * index_y) as i32,
                        ),
                        1,
                        engine::Color::BLACK,
                        &format!("{}", chr),
                    );
                } else {
                    game.draw_text(
                        (
                            (offset_x + 8 * index_x) as i32,
                            (offset_y + 8 * index_y) as i32,
                        ),
                        1,
                        engine::Color::WHITE,
                        &format!("{}", chr),
                    );
                }
                index_x += 1;
            }
            index_y += 1;
        }

        // END
        if game.get_key(Keycode::S).pressed {
            std::fs::write(
                &args[1],
                ron::ser::to_string(&c_world.to_world()).map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?;
        }

        // FIXME: THIS WHOLE BLOCK !
        if !add_tile && game.get_key(Keycode::A).pressed {
            //#[allow(unused_assignments)]
            add_tile = true;
            typing = true;
            add_tile_field = 0;
            add_tile_t = maps::Tile {
                sprite: None,
                sprite_path: String::new(),
                chr: '\u{0000}',
            }
        }
        //add_tile = false; // FORCE TO NOT GO TO THE ADD TILE MENU BC IT DON'T WORK I DON'T KNOW WHY !!!
        if add_tile {
            if add_tile_field == 0 {
                if !typed_string.is_empty() {
                    add_tile_t.sprite_path = typed_string.clone();
                }
            //current_string = &mut add_tile_path;
            //*current_string = format!("{}", typed_string.clone());
            //add_tile_t.sprite_path = copy_string(format!("{}", typed_string));
            } else if add_tile_field == 1 {
                while typed_string.len() > 4 {
                    typed_string.pop();
                }
                //println!("{}", add_tile_t.sprite_path);
                /*
                current_string = &mut add_tile_chr_buf;
                *current_string = format!("{}", typed_string.clone());

                */
                if !typed_string.is_empty() {
                    add_tile_chr_buf = typed_string.clone();
                }
            }

            game.fill_rect((50, 150), (500, 100), engine::Color::BLACK);
            game.draw_rect((50, 150), (500, 100), engine::Color::MAGENTA);
            game.draw_text((51, 151), 2, engine::Color::WHITE, "Sprite Path");
            game.draw_rect((51, 167), (498, 10), engine::Color::RED);
            game.draw_text(
                (52, 168),
                1,
                engine::Color::WHITE,
                &format!("{}", &add_tile_t.sprite_path),
            );
            game.draw_text((51, 177), 2, engine::Color::WHITE, "Sprite Char");
            game.draw_rect((51, 167 + 26), (16 * 4 + 2, 18), engine::Color::YELLOW);
            game.draw_rect(
                (51 + 16 * 4 + 2 + 5, 167 + 26),
                (18, 18),
                engine::Color::YELLOW,
            );
            game.draw_text(
                (52, 168 + 26),
                2,
                engine::Color::WHITE,
                &format!("{}", add_tile_chr_buf.clone()),
            );
            game.draw_text(
                (52 + 16 * 4 + 2 + 5, 168 + 26),
                2,
                engine::Color::WHITE,
                &format!("{}", add_tile_t.chr),
            );
            match u32::from_str_radix(&add_tile_chr_buf, 16) {
                Ok(int) => {
                    add_tile_t.chr = match std::char::from_u32(int) {
                        Some(chr) => chr,
                        None => std::char::from_u32(0xFFFD).unwrap(),
                    };
                }
                _ => {}
            };
            if !typing && add_tile_field < 3 {
                typing = true;
                add_tile_field += 1;
                if add_tile_field > 2 {
                    typing = false;
                    //add_tile_t.chr = add_tile_chr.clone();
                    c_world
                        .tiles
                        .insert(add_tile_t.chr.clone(), add_tile_t.clone());
                    add_tile_t = maps::Tile {
                        sprite: None,
                        sprite_path: String::new(),
                        chr: 'c',
                    };
                }
            }
            /*
            game.draw_text(
                0,
                0,
                format!("{}", add_tile_field),
                engine::Color::WHITE,
                1,
            );*/
        }

        if !typing && add_tile && game.get_key(Keycode::D).pressed {
            add_tile = false;
        }

        // HANDLE SPRITE LIST
        /* 267,306,328,289 */
        let mut selected_tile = c_world.tiles.values().nth(selected_tile_index as usize);
        let mut d_offset = 0;
        for (chr, spr) in &c_world.tiles {
            match selected_tile.clone() {
                Some(_) => {
                    if *chr == selected_tile.clone().unwrap().chr {
                        game.fill_rect(
                            (267, 306 + d_offset),
                            (596 - 267 - 2, 8),
                            engine::Color::VERY_DARK_GREY,
                        );
                        game.draw_text(
                            (267, 306 + d_offset as i32),
                            1,
                            engine::Color::WHITE,
                            &format!("'{}': {}", chr, str_normalize(spr.sprite_path.clone())),
                        );
                    } else {
                        game.draw_text(
                            (267, 306 + d_offset as i32),
                            1,
                            engine::Color::WHITE,
                            &format!("'{}': {}", chr, str_normalize(spr.sprite_path.clone())),
                        );
                    }
                }
                _ => {
                    game.draw_text(
                        (267, 306 + d_offset as i32),
                        1,
                        engine::Color::WHITE,
                        &format!("'{}': {}", chr, str_normalize(spr.sprite_path.clone())),
                    );
                }
            }
            d_offset += 8;
        }

        // END OF SPRITE LIST

        // HANDLE SPRITE DATA
        match &mut selected_tile {
            Some(tile) => {
                sprite_frame(game, &tile.sprite); // Handle Sprite Preview

                game.draw_text(
                    (6, 567),
                    1,
                    engine::Color::GREY,
                    &format!(
                        "Current Tile: {} \nSpr Path: {}",
                        tile.chr, tile.sprite_path
                    ),
                );
            }
            None => {
                sprite_frame(game, &None);
                game.draw_text((6, 566 + 8), 2, engine::Color::GREY, "No Tile Selected");
            }
        };
        /*if let Some(tile) = selected_tile {
        } else {
        }*/
        // END OF SPRITE DATA
        game.draw_text(
            (0, 0),
            1,
            if add_tile {
                engine::Color::GREEN
            } else {
                engine::Color::RED
            },
            &format!("{:?}", add_tile),
        );
        Ok(true)
    });
}

fn str_normalize(source: String) -> String {
    let mut res: String = String::new();
    for chr in source.chars() {
        if res.len() < 10 {
            res.push(chr);
        }
    }
    res
}

fn normalize(source: Keycode) -> String {
    use Keycode::*;
    (match source {
        A => "a",
        B => "b",
        C => "c",
        D => "d",
        E => "e",
        F => "f",
        G => "g",
        H => "h",
        I => "i",
        J => "j",
        K => "k",
        L => "l",
        M => "m",
        N => "n",
        O => "o",
        P => "p",
        Q => "q",
        R => "r",
        S => "s",
        T => "t",
        U => "u",
        V => "v",
        W => "w",
        X => "x",
        Y => "y",
        Z => "z",
        Key1 | Numpad1 => "1",
        Key2 | Numpad2 => "2",
        Key3 | Numpad3 => "3",
        Key4 | Numpad4 => "4",
        Key5 | Numpad5 => "5",
        Key6 | Numpad6 => "6",
        Key7 | Numpad7 => "7",
        Key8 | Numpad8 => "8",
        Key9 | Numpad9 => "9",
        Key0 | Numpad0 => "0",
        Space => " ",
        _ => "",
    })
    .to_owned()
}
fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        use std::panic;
        panic::set_hook(Box::new(pixel_engine::console_error_panic_hook::hook));
        pixel_engine::wasm_bindgen_futures::spawn_local(init());
    };
    #[cfg(not(target_arch = "wasm32"))]
    pixel_engine::futures::executor::block_on(init());
}
