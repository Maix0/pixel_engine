extern crate pixel_engine as engine;
extern crate rand;
use engine::{traits::*, Color};
async fn init() {
    const FIRE_WIDTH: u32 = 250;
    const FIRE_HEIGTH: u32 = 120;
    let game =
        engine::EngineWrapper::new("Doom Fire".to_owned(), (FIRE_WIDTH, FIRE_HEIGTH, 3)).await;
    let mut bottomline = true;
    let mut spread_center = 0;
    let mut spread_raduis = 0;
    let palette: [Color; 37] = [
        [0x00, 0x00, 0x00].into(),
        [0x1F, 0x07, 0x07].into(),
        [0x2F, 0x0F, 0x07].into(),
        [0x47, 0x0F, 0x07].into(),
        [0x57, 0x17, 0x07].into(),
        [0x67, 0x1F, 0x07].into(),
        [0x77, 0x1F, 0x07].into(),
        [0x8F, 0x27, 0x07].into(),
        [0x9F, 0x2F, 0x07].into(),
        [0xAF, 0x3F, 0x07].into(),
        [0xBF, 0x47, 0x07].into(),
        [0xC7, 0x47, 0x07].into(),
        [0xDF, 0x4F, 0x07].into(),
        [0xDF, 0x57, 0x07].into(),
        [0xDF, 0x57, 0x07].into(),
        [0xD7, 0x5F, 0x07].into(),
        [0xD7, 0x5F, 0x07].into(),
        [0xD7, 0x67, 0x0F].into(),
        [0xCF, 0x6F, 0x0F].into(),
        [0xCF, 0x77, 0x0F].into(),
        [0xCF, 0x7F, 0x0F].into(),
        [0xCF, 0x87, 0x17].into(),
        [0xC7, 0x87, 0x17].into(),
        [0xC7, 0x8F, 0x17].into(),
        [0xC7, 0x97, 0x1F].into(),
        [0xBF, 0x9F, 0x1F].into(),
        [0xBF, 0x9F, 0x1F].into(),
        [0xBF, 0xA7, 0x27].into(),
        [0xBF, 0xA7, 0x27].into(),
        [0xBF, 0xAF, 0x2F].into(),
        [0xB7, 0xAF, 0x2F].into(),
        [0xB7, 0xB7, 0x2F].into(),
        [0xB7, 0xB7, 0x37].into(),
        [0xCF, 0xCF, 0x6F].into(),
        [0xDF, 0xDF, 0x9F].into(),
        [0xEF, 0xEF, 0xC7].into(),
        [0xFF, 0xFF, 0xFF].into(),
    ];
    let mut firepixel = vec![0x00usize; (game.size.0 * game.size.1) as usize];
    fn spread_fire(src: usize, firepixel: &mut Vec<usize>) {
        let pixel = firepixel[src];
        if pixel == 0 {
            if src >= FIRE_WIDTH as usize {
                firepixel[src - FIRE_WIDTH as usize] = 0;
            }
        } else {
            let rand_idx = (rand::random::<f64>() * 3.0).round() as usize; // & 3;
            let dst = src - rand_idx + 1;
            if dst >= FIRE_WIDTH as usize {
                firepixel[dst - FIRE_WIDTH as usize] = pixel - (rand_idx & 1);
            }
        }
    }
    fn do_fire(firepixel: &mut Vec<usize>) {
        for x in 0..FIRE_WIDTH {
            for y in 1..FIRE_HEIGTH {
                spread_fire((y * FIRE_WIDTH + x) as usize, firepixel);
            }
        }
    }
    for i in 0..game.size.0 {
        firepixel[((game.size.1 - 1) * game.size.0 + i) as usize] = 36;
    }
    game.run(move |game: &mut engine::Engine| {
        if game.get_key(engine::inputs::Keycodes::Escape).any() {
            return Ok(false);
        }
        if game.get_key(engine::inputs::Keycodes::Space).pressed {
            bottomline = !bottomline;
            if bottomline {
                spread_center = (rand::random::<f64>() * FIRE_WIDTH as f64).round() as usize;
                spread_raduis = 0;
            } else {
                spread_center = (rand::random::<f64>() * FIRE_WIDTH as f64).round() as usize;
                spread_raduis = 0;
            }
        }
        /*
        if firepixel[(game.size.1 * game.size.0) as usize - 1] != 0 && !bottomline {
            for i in 0..game.size.0 {
                firepixel[((game.size.1 - 1) * game.size.0 + i) as usize] -= 1;
            }
        }*/

        if !bottomline && spread_center < FIRE_HEIGTH as usize {
            spread_raduis += 1;
            for x in spread_center.saturating_sub(spread_raduis)
                ..=(std::cmp::min(FIRE_WIDTH as usize - 1, spread_center + spread_raduis))
            {
                firepixel[((game.size.1 - 1) * game.size.0 + x as u32) as usize] = firepixel
                    [((game.size.1 - 1) * game.size.0 + x as u32) as usize]
                    .saturating_sub(1);
            }
        }
        if bottomline && spread_center < FIRE_HEIGTH as usize {
            spread_raduis += 1;
            for x in spread_center.saturating_sub(spread_raduis)
                ..=(std::cmp::min(FIRE_WIDTH as usize - 1, spread_center + spread_raduis))
            {
                firepixel[((game.size.1 - 1) * game.size.0 + x as u32) as usize] = std::cmp::min(
                    firepixel[((game.size.1 - 1) * game.size.0 + x as u32) as usize] + 1,
                    36,
                );
            }
        }

        do_fire(&mut firepixel);
        for y in 0..game.size.1 {
            for x in 0..game.size.0 {
                let index = firepixel[(y * game.size.0 + x) as usize];
                let pixel = palette[index];
                game.draw((x as i32, y as i32), pixel);
            }
        }
        game.draw_text(
            (0, 0),
            1,
            engine::Color::WHITE,
            match bottomline {
                true => "On",
                false => "Off",
            },
        );
        Ok(true)
    });
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
