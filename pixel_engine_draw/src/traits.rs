use super::graphics::{Color, PixelMode, Sprite};

macro_rules! impl_trait {
    ($trait:ident) => {
        impl<T: ScreenTrait> $trait for T {}
    };
}
///The Basic Drawing Trait
///All that is needed to draw one pixel one the target
pub trait ScreenTrait {
    /// Get the size of the target
    fn get_size(&mut self) -> (usize, usize);
    /// Get The textsheet (A [`Sprite`])
    fn get_textsheet(&self) -> &Sprite;
    /// Clear the Screen With the given [`Color`]
    fn clear(&mut self, col: Color);
    /// Set the pixel data at the given coordinates to the given Color
    /// Will use the current [`PixelMode`]
    fn draw(&mut self, x: u32, y: u32, col: Color);
    /// Get the Pixel Data at the given coordinates
    fn get_pixel(&self, x: u32, y: u32) -> Color;
    /// Return the [`PixelMode`]
    fn get_pixel_mode(&self) -> PixelMode;
    /// Set the [`PixelMode`]
    fn set_pixel_mode(&mut self, mode: PixelMode);
    /// Get the Blend Factor
    /// Used for alpha calculations
    fn get_blend_factor(&self) -> f32;
    /// Set the Blend Factor
    /// Used for alpha calculations
    fn set_blend_factor(&mut self, f: f32);
}

/// A trait that regroups all the Shapes Drawing
/// You don't need to implement anything other that [`ScreenTrait`] to use it
pub trait ShapesTrait: ScreenTrait {
    /// Draw text to the screen
    /// `scale` must be >= 1
    /// The textsize will be equal to `scale * 8` for the height and `scale * 8 * text.len()` for
    /// the width
    /// This will handle `\n` treating it as a new line, but wont do any newline stuff if it is
    /// drawing out of the screen
    fn draw_text(&mut self, x: u32, y: u32, scale: u32, col: Color, text: &str) {
        let mut sx = 0;
        let mut sy = 0;
        for chr in text.chars() {
            if chr == '\n' {
                sx = 0;
                sy += 8 * scale;
            } else {
                if !chr.is_ascii() {
                    continue;
                }
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

    /// Draw a line between two points,
    /// You don't need to do anything with the points for it to work, it will swap them it needed.
    fn draw_line(&mut self, p1: (u32, u32), p2: (u32, u32), col: Color) {
        /* OLD Implementation by me
         * let (p1, p2) = if p1.0 < p2.0 { (p1, p2) } else { (p2, p1) };
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
        }*/
        if p1.0 == p2.0 {
            // VERTICAL LINE
            for y in if p2.1 > p1.1 {
                p1.1..=p2.1
            } else {
                p2.1..=p1.1
            } {
                self.draw(p1.0, y, col);
            }
        } else if p1.1 == p2.1 {
            // HORIZONTAL LINE
            for x in if p2.0 > p1.0 {
                p1.0..=p2.0
            } else {
                p2.0..=p1.0
            } {
                self.draw(x, p1.1, col);
            }
        } else {
            let (mut x0, mut y0) = (p1.0 as i64, p1.1 as i64);
            let (mut x1, mut y1) = (p2.0 as i64, p2.1 as i64);
            if (y1 - y0).abs() < (x1 - x0).abs() {
                if x0 > x1 {
                    std::mem::swap(&mut x0, &mut x1);
                    std::mem::swap(&mut y0, &mut y1);
                }
                let dx = x1 - x0;
                let mut dy = y1 - y0;
                let mut yi = 1;
                if dy < 0 {
                    yi = -1;
                    dy = -dy;
                }
                let mut d = 2 * dy - dx;
                let mut y = y0;

                for x in x0..x1 {
                    if x >= 0 && y >= 0 {
                        self.draw(x as u32, y as u32, col);
                    }
                    if d > 0 {
                        y += yi;
                        d -= 2 * dx;
                    }
                    d += 2 * dy;
                }
            } else {
                if y0 > y1 {
                    std::mem::swap(&mut x0, &mut x1);
                    std::mem::swap(&mut y0, &mut y1);
                }
                let mut dx = x1 - x0;
                let dy = y1 - y0;
                let mut xi = 1;
                if dx < 0 {
                    xi = -1;
                    dx = -dx;
                }
                let mut d = 2 * dx - dy;
                let mut x = x0;

                for y in y0..=y1 {
                    if x >= 0 && y >= 0 {
                        self.draw(x as u32, y as u32, col);
                    }
                    if d > 0 {
                        x += xi;
                        d -= 2 * dy;
                    }
                    d += 2 * dx;
                }
            }
        }
    }
    /// Draw a rectangle with the top left corner at `(x, y)`
    /// and the bottom right corner at `(x + w, y + h)` (both inclusive)
    fn draw_rect(&mut self, x: u32, y: u32, w: u32, h: u32, col: Color) {
        self.draw_line((x, y), (x + w, y), col);
        self.draw_line((x + w, y), (x + w, y + h), col);
        self.draw_line((x + w, y + h), (x, y + h), col);
        self.draw_line((x, y + h), (x, y), col);
    }

    /// Fill a rectangle with the top left corner at `(x, y)`
    /// and the bottom right corner at `(x + w, y + h)` (both inclusive)
    fn fill_rect(&mut self, x: u32, y: u32, w: u32, h: u32, col: Color) {
        for nx in x..=(x + w) {
            self.draw_line((nx, y), (nx, y + h), col);
        }
    }

    /// Draw a circle with center `(x, y)` and raduis `r`
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

    /// Fill a circle with center `(x, y)` and raduis `r`
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

    /// Draw the edges of a triangle between the three points
    fn draw_triangle(&mut self, pts1: (u32, u32), pts2: (u32, u32), pts3: (u32, u32), col: Color) {
        self.draw_line(pts1, pts2, col);
        self.draw_line(pts1, pts3, col);
        self.draw_line(pts2, pts3, col);
    }

    /// Fill the given triangle
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
/// A trait that will handle the drawing of Sprite onto the Target
pub trait SpriteTrait: ScreenTrait {
    /// Draw a Sprite with the top left corner at `(x, y)`
    /// the flip arguement will allow fliping of the axis
    /// flip: (horizontal, vertical)
    /// scale is the scale of the result (must be >= 1)
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
                                x + i * scale + is,
                                y + j * scale + js,
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
                    self.draw(x + i, y + j, sprite.get_pixel(fx as u32, fy as u32));
                    fy += fym;
                }
                fx += fxm;
            }
        }
    }
    /// Draw a chunk of the given [`Sprite`] onto the Target
    /// `coords` is the top left corner of the Target
    /// `o` is the Top left corner of the Sprite Chunk
    /// and `size` is the `(width, height)` of the chunk
    /// `flip` and `scale` is the same as [`SpriteTrait::draw_sprite()`]
    fn draw_partial_sprite(
        &mut self,
        coords: (u32, u32),
        sprite: &Sprite,
        o: (u32, u32),
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
                                x + i * scale + is,
                                y + j * scale + js,
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
                        x + i,
                        y + j,
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
