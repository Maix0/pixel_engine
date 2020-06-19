use super::graphics::{Color, Sprite};
use super::screen::{PixelMode, Screen};

macro_rules! impl_trait {
    ($trait:ident) => {
        impl<T: ScreenTrait> $trait for T {}
    };
}

pub trait ScreenTrait {
    fn get_size(&mut self) -> (usize, usize);
    fn get_textsheet(&self) -> &Sprite;
    fn clear(&mut self, col: Color);
    fn draw(&mut self, x: u32, y: u32, col: Color);
    fn get_pixel(&self, x: u32, y: u32) -> Color;
    fn get_pixel_mode(&self) -> PixelMode;
    fn set_pixel_mode(&mut self, mode: PixelMode);
    fn get_blend_factor(&self) -> f32;
    fn set_blend_factor(&mut self, f: f32);
}
impl ScreenTrait for Screen {
    fn get_size(&mut self) -> (usize, usize) {
        (self.screen.width as usize, self.screen.height as usize)
    }
    fn clear(&mut self, col: Color) {
        self.screen = Sprite::new_with_color(self.size.0, self.size.1, col);
    }
    fn get_textsheet(&self) -> &Sprite {
        &self.textsheet
    }
    fn draw(&mut self, x: u32, y: u32, col: Color) {
        if x >= self.size.0 || y >= self.size.1 {
            return;
        }
        self.updated = true;
        match self.get_pixel_mode() {
            PixelMode::Normal => {
                self.screen.set_pixel(x, y, col);
            }
            PixelMode::Alpha => {
                if col.a == 255 {
                    self.screen.set_pixel(x, y, col);
                }
            }
            PixelMode::Mask => {
                let current_color: Color = self.get_pixel(x, y);
                let alpha: f32 = (col.a as f32 / 255.0f32) as f32 * self.get_blend_factor();
                let inverse_alpha: f32 = 1.0 - alpha;
                let red: f32 = alpha * col.r as f32 + inverse_alpha * current_color.r as f32;
                let green: f32 = alpha * col.g as f32 + inverse_alpha * current_color.g as f32;
                let blue: f32 = alpha * col.b as f32 + inverse_alpha * current_color.b as f32;
                self.screen.set_pixel(x, y, [red, green, blue].into());
            }
        }
    }
    fn get_pixel_mode(&self) -> PixelMode {
        self.pixel_mode
    }

    fn set_pixel_mode(&mut self, mode: PixelMode) {
        self.pixel_mode = mode;
    }
    fn get_pixel(&self, x: u32, y: u32) -> Color {
        self.screen.get_pixel(x, y)
    }
    fn get_blend_factor(&self) -> f32 {
        self.blend_factor
    }
    fn set_blend_factor(&mut self, f: f32) {
        self.blend_factor = {
            if f < 0.0 {
                0.0
            } else if f > 1.0 {
                1.0
            } else {
                f
            }
        }
    }
}
pub trait ShapesTrait: ScreenTrait {
    fn draw_text(&mut self, x: u32, y: u32, scale: u32, col: Color, text: String) {
        let mut sx = 0;
        let mut sy = 0;
        for chr in text.chars() {
            if chr == '\n' {
                sx = 0;
                sy += 8 * scale;
            } else {
                let ox: u32 = (chr as u32 - 32) % 16;
                let oy: u32 = (chr as u32 - 32) / 16;
                if scale > 1 {
                    for i in 0..8 {
                        for j in 0..8 {
                            if self.get_textsheet().get_pixel(i + ox * 8, j + oy * 8).r > 0 {
                                for is in 0..=scale {
                                    for js in 0..=scale {
                                        self.draw(
                                            x + sx + (i * scale) + is,
                                            y + sy + (j * scale) + js,
                                            col,
                                        )
                                    }
                                }
                            }
                        }
                    }
                } else {
                    for i in 0..8 {
                        for j in 0..8 {
                            if self.get_textsheet().get_pixel(i + ox * 8, j + oy * 8).r > 0 {
                                self.draw(x + sx + i, y + sy + j, col)
                            }
                        }
                    }
                }
            }
            sx += 8 * scale;
        }
    }
    fn draw_line(&mut self, p1: (u32, u32), p2: (u32, u32), col: Color) {
        let (p1, p2) = if p1.0 < p2.0 { (p1, p2) } else { (p2, p1) };
        if p1.0 == p2.0 {
            let iter = if p1.1 < p2.1 {
                p1.1..=(p2.1)
            } else {
                p2.1..=(p1.1)
            };
            for y in iter {
                self.draw(p1.0, y, col);
            }
        } else {
            let a = (p1.1 as f32 - p2.1 as f32) / (p1.0 as f32 - p2.0 as f32);
            let b = p1.1 as f32 - (a * p1.0 as f32);
            /*println!(
                "START {:?} || END: {:?} || a = {:.2} = {:.2}/{:.2} || b = {:.2}",
                p1,
                p2,
                a,
                (p1.1 as f32 - p2.1 as f32),
                (p1.0 as f32 - p2.0 as f32),
                b
            );*/
            if -1.0 <= a && a <= 1.0 {
                for x in p1.0..=(p2.0) {
                    let y = a * x as f32 + b;
                    self.draw(x, y.round() as u32, col);
                }
            } else if a > 1.0 || a < -1.0 {
                let iter = if p1.1 < p2.1 {
                    p1.1..=p2.1
                } else {
                    p2.1..=p1.1
                };
                for y in iter {
                    let x = ((y as f32 - b) / a).round() as u32;
                    self.draw(x, y, col);
                }
            }
        }
    }
    fn draw_rect(&mut self, x: u32, y: u32, w: u32, h: u32, col: Color) {
        self.draw_line((x, y), (x + w, y), col);
        self.draw_line((x + w, y), (x + w, y + h), col);
        self.draw_line((x + w, y + h), (x, y + h), col);
        self.draw_line((x, y + h), (x, y), col);
    }
    fn fill_rect(&mut self, x: u32, y: u32, w: u32, h: u32, col: Color) {
        for nx in x..=(x + w) {
            self.draw_line((nx, y), (nx, y + h), col);
        }
    }
    fn draw_circle(&mut self, x: u32, y: u32, r: u32, col: Color) {
        let x = x as i32;
        let y = y as i32;
        let mut x0: i32 = 0;
        let mut y0: i32 = r as i32;
        let mut d: i32 = 3 - 2 * r as i32;
        if r == 0 {
            return;
        }
        while y0 >= x0 {
            self.draw((x + x0) as u32, (y - y0) as u32, col);
            self.draw((x + y0) as u32, (y - x0) as u32, col);
            self.draw((x + y0) as u32, (y + x0) as u32, col);
            self.draw((x + x0) as u32, (y + y0) as u32, col);

            self.draw((x - x0) as u32, (y + y0) as u32, col);
            self.draw((x - y0) as u32, (y + x0) as u32, col);
            self.draw((x - y0) as u32, (y - x0) as u32, col);
            self.draw((x - x0) as u32, (y - y0) as u32, col);

            x0 += 1;
            if d < 0 {
                d += 4 * x0 + 6;
            } else {
                y0 -= 1;
                d += 4 * (x0 - y0) + 10;
            }
        }
    }

    fn fill_circle(&mut self, x: u32, y: u32, r: u32, col: Color) {
        use std::cmp::max;
        let x = x as i32;
        let y = y as i32;
        let mut x0: i32 = 0;
        let mut y0: i32 = r as i32;
        let mut d: i32 = 3 - 2 * r as i32;
        if r == 0 {
            return;
        }
        while y0 >= x0 {
            self.draw_line(
                (max(x - x0, 0) as u32, max(y - y0, 0) as u32),
                (max(x + x0, 0) as u32, max(y - y0, 0) as u32),
                col,
            );
            self.draw_line(
                (max(x - y0, 0) as u32, max(y - x0, 0) as u32),
                (max(x + y0, 0) as u32, max(y - x0, 0) as u32),
                col,
            );
            self.draw_line(
                (max(x - x0, 0) as u32, max(y + y0, 0) as u32),
                (max(x + x0, 0) as u32, max(y + y0, 0) as u32),
                col,
            );
            self.draw_line(
                (max(x - y0, 0) as u32, max(y + x0, 0) as u32),
                (max(x + y0, 0) as u32, max(y + x0, 0) as u32),
                col,
            );
            x0 += 1;
            if d < 0 {
                d += 4 * x0 + 6;
            } else {
                y0 -= 1;
                d += 4 * (x0 - y0) + 10;
            }
        }
    }
    fn draw_triangle(&mut self, pts1: (u32, u32), pts2: (u32, u32), pts3: (u32, u32), col: Color) {
        self.draw_line(pts1, pts2, col);
        self.draw_line(pts1, pts3, col);
        self.draw_line(pts2, pts3, col);
    }

    fn fill_triangle(&mut self, pts1: (u32, u32), pts2: (u32, u32), pts3: (u32, u32), col: Color) {
        self.draw_triangle(pts1, pts2, pts3, col);
        let pts1 = (pts1.0 as i64, pts1.1 as i64);
        let pts2 = (pts2.0 as i64, pts2.1 as i64);
        let pts3 = (pts3.0 as i64, pts3.1 as i64);
        let centroid = (
            ((pts1.0 + pts2.0 + pts3.0) as f32 / 3f32),
            ((pts1.1 + pts2.1 + pts3.1) as f32 / 3f32),
        );
        let lines = (
            (
                pts1.1 - pts2.1,
                pts2.0 - pts1.0,
                (pts1.1 - pts2.1) * pts2.0 + (pts2.0 - pts1.0) * pts2.1,
            ),
            (
                pts3.1 - pts1.1,
                pts1.0 - pts3.0,
                (pts3.1 - pts1.1) * pts1.0 + (pts1.0 - pts3.0) * pts1.1,
            ),
            (
                pts2.1 - pts3.1,
                pts3.0 - pts2.0,
                (pts2.1 - pts3.1) * pts3.0 + (pts3.0 - pts2.0) * pts3.1,
            ),
        );
        //dbg!(&lines);
        use std::cmp::{max, min};
        let iterx = {
            let x1 = min(min(pts1.0, pts2.0), pts3.0);
            let x2 = max(max(pts1.0, pts2.0), pts3.0);
            if x1 > x2 {
                x2..=x1
            } else {
                x1..=x2
            }
        };
        let itery = {
            let y1 = min(min(pts1.1, pts2.1), pts3.1);
            let y2 = max(max(pts1.1, pts2.1), pts3.1);

            if y1 > y2 {
                y2..=y1
            } else {
                y1..=y2
            }
        };
        let l_mul = (
            if (lines.0).0 as f32 * centroid.0 + (lines.0).1 as f32 * centroid.1
                - (lines.0).2 as f32
                >= 0f32
            {
                1
            } else {
                -1
            },
            if (lines.1).0 as f32 * centroid.0 + (lines.1).1 as f32 * centroid.1
                - (lines.1).2 as f32
                >= 0f32
            {
                1
            } else {
                -1
            },
            if (lines.2).0 as f32 * centroid.0 + (lines.2).1 as f32 * centroid.1
                - (lines.2).2 as f32
                >= 0f32
            {
                1
            } else {
                -1
            },
        );

        for x in iterx {
            for y in itery.clone() {
                if ((lines.0).0 * x + (lines.0).1 * y - (lines.0).2) * l_mul.0 >= 0
                    && ((lines.1).0 * x + (lines.1).1 * y - (lines.1).2) * l_mul.1 >= 0
                    && ((lines.2).0 * x + (lines.2).1 * y - (lines.2).2) * l_mul.2 >= 0
                {
                    self.draw(x as u32, y as u32, col)
                }
            }
        }
    }
}

pub trait SpriteTrait: ScreenTrait {
    /// flip: (horizontal, vertical)
    fn draw_sprite(&mut self, x: u32, y: u32, scale: u32, sprite: &Sprite, flip: (bool, bool)) {
        let (mut fxs, mut fxm) = (0i32, 1i32);
        let (mut fys, mut fym) = (0i32, 1i32);
        let mut fx: i32;
        let mut fy: i32;
        if flip.0 {
            fxs = sprite.width as i32 - 1;
            fxm = -1;
        }
        if flip.1 {
            fys = sprite.height as i32 - 1;
            fym = -1;
        }
        if scale > 1 {
            fx = fxs;
            for i in 0..sprite.width {
                fy = fys;
                for j in 0..sprite.height {
                    for is in 0..scale {
                        for js in 0..scale {
                            self.draw(
                                (x + (i * scale) + is) as u32,
                                (y + (j * scale) + js) as u32,
                                sprite.get_pixel(fx as u32, fy as u32),
                            );
                        }
                    }
                    fy += fym;
                }
                fx += fxm;
            }
        } else {
            fx = fxs;
            for i in 0..sprite.width {
                fy = fys;
                for j in 0..sprite.height {
                    self.draw(
                        (x + i) as u32,
                        (y + j) as u32,
                        sprite.get_pixel(fx as u32, fy as u32),
                    );
                    fy += fym;
                }
                fx += fxm;
            }
        }
    }
    fn draw_partial_sprite(
        &mut self,
        /*x: u32,
        y: u32,*/
        coords: (u32, u32),
        sprite: &Sprite,
        /*ox: u32,
        oy: u32,*/
        o: (u32, u32),
        /*w: u32,
        h: u32,*/
        size: (u32, u32),
        scale: u32,
        flip: (bool, bool),
    ) {
        let (x, y) = coords;
        let (ox, oy) = o;
        let (w, h) = size;

        let (mut fxs, mut fxm) = (0i32, 1i32);
        let (mut fys, mut fym) = (0i32, 1i32);
        let mut fx: i32;
        let mut fy: i32;
        if flip.0 {
            fxs = w as i32 - 1;
            fxm = -1;
        }
        if flip.1 {
            fys = h as i32 - 1;
            fym = -1;
        }

        if scale > 1 {
            fx = fxs;
            for i in 0..w {
                fy = fys;
                for j in 0..h {
                    for is in 0..scale {
                        for js in 0..scale {
                            self.draw(
                                (x + (i * scale) + is) as u32,
                                (y + (j * scale) + js) as u32,
                                sprite.get_pixel(fx as u32 + ox, fy as u32 + oy),
                            );
                        }
                    }
                    fy += fym;
                    fx += fxm;
                }
            }
        } else {
            fx = fxs;
            for i in 0..w {
                fy = fys;
                for j in 0..h {
                    self.draw(
                        (x + i) as u32,
                        (y + j) as u32,
                        sprite.get_pixel(fx as u32 + ox, fy as u32 + oy),
                    );
                    fy += fym;
                }
                fx += fxm;
            }
        }
    }
}
impl_trait!(SpriteTrait);
impl_trait!(ShapesTrait);
