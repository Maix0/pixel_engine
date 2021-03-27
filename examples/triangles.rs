extern crate pixel_engine as engine;

use engine::inputs::Keycodes::*;
use engine::traits::*;
fn main() {
    let game = engine::EngineWrapper::new("Template".to_owned(), (500, 500, 1));
    let mut pts1 = (350, 250);
    let mut pts2 = (121, 126);
    let mut pts3 = (256, 485);
    let mut selected = 0;
    let mut fill = false;
    let text_scale = 4;
    game.run(move |game: &mut engine::Engine| {
        let mut selected_pts = match selected {
            0 => &mut pts1,
            1 => &mut pts2,
            2 => &mut pts3,
            _ => &mut pts1,
        };
        game.screen.clear([0, 0, 0].into());
        if game.get_key(Escape).any() {
            return Ok(false);
        }
        if game.get_key(A).any() {
            selected = 0;
            selected_pts = &mut pts1;
        }
        if game.get_key(Z).any() {
            selected = 1;
            selected_pts = &mut pts2;
        }
        if game.get_key(E).any() {
            selected = 2;
            selected_pts = &mut pts3;
        }
        if game.get_key(Up).any() {
            if selected_pts.1 > 8 * text_scale * 3 {
                (*selected_pts).1 -= 1;
            }
        }
        if game.get_key(Down).any() {
            if selected_pts.1 < game.size.1 - 1 {
                (*selected_pts).1 += 1;
            }
        }
        if game.get_key(Left).any() {
            if selected_pts.0 > 0 {
                (*selected_pts).0 -= 1;
            }
        }
        if game.get_key(Right).any() {
            if selected_pts.0 < game.size.0 - 1 {
                (*selected_pts).0 += 1;
            }
        }
        if game.get_key(Space).pressed {
            fill = !fill;
        }
        if fill {
            game.screen
                .fill_triangle(pts1, pts2, pts3, [255, 255, 255].into());
        } else {
            game.screen
                .draw_triangle(pts1, pts2, pts3, [255, 255, 255].into());
        }
        //game.screen.draw(pts1.0, pts1.1, [255, 0, 0].into());
        //game.screen.draw(pts2.0, pts2.1, [0, 255, 0].into());
        //game.screen.draw(pts3.0, pts3.1, [0, 0, 255].into());
        game.screen.draw_text(
            0,
            0,
            text_scale,
            [255, 0, 0].into(),
            &format!(
                "{}({:>2},{:>2})",
                match selected {
                    0 => "@",
                    _ => " ",
                },
                pts1.0,
                pts1.1
            ),
        );
        game.screen.draw_text(
            0,
            text_scale * 8,
            text_scale,
            [0, 255, 0].into(),
            &format!(
                "{}({:>2},{:>2})",
                match selected {
                    1 => "@",
                    _ => " ",
                },
                pts2.0,
                pts2.1
            ),
        );
        game.screen.draw_text(
            0,
            text_scale * 8 * 2,
            text_scale,
            [0, 0, 255].into(),
            &format!(
                "{}({:>2},{:>2})",
                match selected {
                    2 => "@",
                    _ => " ",
                },
                pts3.0,
                pts3.1
            ),
        );
        game.screen.draw_text(
            game.size.0 - 5 * 8 * text_scale,
            0,
            text_scale,
            [255, 255, 255].into(),
            "fill",
        );
        game.screen.draw_text(
            match fill {
                true => game.size.0 - "Y".len() as u32 * 8 * text_scale,
                false => game.size.0 - "X".len() as u32 * 8 * text_scale,
            },
            0,
            text_scale,
            match fill {
                true => [0.0, 1.0, 0.0].into(),
                false => [1.0, 0.0, 0.0].into(),
            },
            match fill {
                true => "V",
                false => "X",
            },
        );
        game.screen.draw_line(
            (0, text_scale * 3 * 8),
            (game.size.0, text_scale * 3 * 8),
            [255, 255, 255].into(),
        );
        Ok(true)
    });
}
