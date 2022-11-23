use super::graphics::{Color, PixelMode, Sprite};
use super::vector2::{Vi2d, Vu2d};

use crate::graphics::DrawSpriteTrait;

macro_rules! impl_trait {
    ($trait:ident) => {
        impl<T: SmartDrawingTrait> $trait for T {}
    };
}

///The Basic Drawing Trait
///All that is needed to draw one pixel one the target
pub trait SmartDrawingTrait: DrawSpriteTrait {
    /// Get the size of the target
    fn get_size(&self) -> Vu2d;
    /// Get The textsheet (A [`Sprite`])
    fn get_textsheet(&self) -> &'static Sprite;
    /// Clear the Sprite With the given [`Color`]
    fn clear(&mut self, col: Color);
    /// Set the pixel data at the given coordinates to the given Color
    /// Will use the current [`PixelMode`]
    fn draw<P: Into<Vi2d>>(&mut self, pos: P, col: Color);
    /// Get the Pixel Data at the given coordinates
    fn get_pixel<P: Into<Vi2d>>(&self, pos: P) -> Option<Color>;
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

pub trait DottedShapeTrait: SmartDrawingTrait {
    /// Draw a dotted line
    fn draw_line_dotted<P: Into<Vi2d>>(&mut self, p1: P, p2: P, col: Color, mut pattern: u32) {
        let mut rol = || {
            pattern = (pattern << 1) | (pattern >> 31);
            pattern & 1 > 0
        };
        let p1: Vi2d = p1.into();
        let p2: Vi2d = p2.into();
        if p1.x == p2.x {
            // VERTICAL LINE
            for y in if p2.y > p1.y {
                p1.y..=p2.y
            } else {
                p2.y..=p1.y
            } {
                if rol() {
                    self.draw((p1.x, y), col);
                }
            }
        } else if p1.y == p2.y {
            // HORIZONTAL LINE
            for x in if p2.x > p1.x {
                p1.x..=p2.x
            } else {
                p2.x..=p1.x
            } {
                if rol() {
                    self.draw((x, p1.y), col);
                }
            }
        } else {
            let (mut x0, mut y0) = (p1.x as i32, p1.y as i32);
            let (mut x1, mut y1) = (p2.x as i32, p2.y as i32);
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
                    if x >= 0 && y >= 0 && rol() {
                        self.draw((x, y), col);
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
                    if x >= 0 && y >= 0 && rol() {
                        self.draw((x, y), col);
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
    /// This is the dotted form
    fn draw_rect_dotted<P: Into<Vi2d>>(&mut self, pos: P, size: P, col: Color, pattern: u32) {
        let Vi2d { x, y } = pos.into();
        let Vi2d { x: w, y: h } = size.into() - Vi2d { x: 1, y: 1 };
        self.draw_line_dotted((x, y), (x + w, y), col, pattern);
        self.draw_line_dotted((x + w, y), (x + w, y + h), col, pattern);
        self.draw_line_dotted((x + w, y + h), (x, y + h), col, pattern);
        self.draw_line_dotted((x, y + h), (x, y), col, pattern);
    }

    /// Draw the edges of a triangle between the three points
    /// This is the dotted form
    fn draw_triangle_dotted<P: Into<Vi2d>>(
        &mut self,
        pts1: P,
        pts2: P,
        pts3: P,
        col: Color,
        pattern: u32,
    ) {
        let pts1: Vi2d = pts1.into();
        let pts2: Vi2d = pts2.into();
        let pts3: Vi2d = pts3.into();
        self.draw_line_dotted(pts1, pts2, col, pattern);
        self.draw_line_dotted(pts1, pts3, col, pattern);
        self.draw_line_dotted(pts2, pts3, col, pattern);
    }
}

/// A trait that regroups all the Shapes Drawing
/// You don't need to implement anything other that [`DrawSpriteTrait`] to use it
pub trait ShapesTrait: SmartDrawingTrait {
    /// Draw text to the screen
    /// `scale` must be >= 1
    /// The textsize will be equal to `scale * 8` for the height and `scale * 8 * text.len()` for
    /// the width
    /// This will handle `\n` treating it as a new line, but wont do any newline stuff if it is
    /// drawing out of the screen
    fn draw_text<P: Into<Vi2d>>(&mut self, pos: P, scale: u32, col: Color, text: &str) {
        #![allow(
            clippy::cast_precision_loss,
            clippy::cast_possible_wrap,
            clippy::cast_sign_loss
        )]
        let Vi2d { x, y } = pos.into();
        let scale: i32 = scale.try_into().unwrap();
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
                let ox: i32 = ((chr as u32 - 32) % 16) as i32;
                let oy: i32 = ((chr as u32 - 32) / 16) as i32;
                if scale > 1 {
                    for i in 0..8i32 {
                        for j in 0..8i32 {
                            if self
                                .get_textsheet()
                                .get_pixel((i + ox * 8) as u32, (j + oy * 8) as u32)
                                .r
                                > 0
                            {
                                for is in 0..=(scale as i32) {
                                    for js in 0..=(scale as i32) {
                                        self.draw(
                                            (x + sx + (i * scale) + is, y + sy + (j * scale) + js),
                                            col,
                                        );
                                    }
                                }
                            }
                        }
                    }
                } else {
                    for i in 0..8i32 {
                        for j in 0..8i32 {
                            if self
                                .get_textsheet()
                                .get_pixel((i + ox * 8) as u32, (j + oy * 8) as u32)
                                .r
                                > 0
                            {
                                self.draw((x + sx + i, y + sy + j), col);
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
    fn draw_line<P: Into<Vi2d>>(&mut self, p1: P, p2: P, col: Color) {
        /* OLD Implementation by me
         * let (p1, p2) = if p1.x < p2.x { (p1, p2) } else { (p2, p1) };
        if p1.x == p2.x {
            let iter = if p1.y < p2.y {
                p1.y..=(p2.y)
            } else {
                p2.y..=(p1.y)
            };
            for y in iter {
                self.draw(p1.x, y, col);
            }
        } else {
            let a = (p1.y as f32 - p2.y as f32) / (p1.x as f32 - p2.x as f32);
            let b = p1.y as f32 - (a * p1.x as f32);
            /*println!(
                "START {:?} || END: {:?} || a = {:.2} = {:.2}/{:.2} || b = {:.2}",
                p1,
                p2,
                a,
                (p1.y as f32 - p2.y as f32),
                (p1.x as f32 - p2.x as f32),
                b
            );*/
            if -1.x <= a && a <= 1.x {
                for x in p1.x..=(p2.x) {
                    let y = a * x as f32 + b;
                    self.draw(x, y.round() as u32, col);
                }
            } else if a > 1.x || a < -1.x {
                let iter = if p1.y < p2.y {
                    p1.y..=p2.y
                } else {
                    p2.y..=p1.y
                };
                for y in iter {
                    let x = ((y as f32 - b) / a).round() as u32;
                    self.draw(x, y, col);
                }
            }
        }*/
        let p1: Vi2d = p1.into();
        let p2: Vi2d = p2.into();
        if p1.x == p2.x {
            // VERTICAL LINE
            for y in if p2.y > p1.y {
                p1.y..=p2.y
            } else {
                p2.y..=p1.y
            } {
                self.draw((p1.x, y), col);
            }
        } else if p1.y == p2.y {
            // HORIZONTAL LINE
            for x in if p2.x > p1.x {
                p1.x..=p2.x
            } else {
                p2.x..=p1.x
            } {
                self.draw((x, p1.y), col);
            }
        } else {
            let (mut x0, mut y0) = (p1.x as i32, p1.y as i32);
            let (mut x1, mut y1) = (p2.x as i32, p2.y as i32);
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
                        self.draw((x, y), col);
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
                        self.draw((x, y), col);
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
    fn draw_rect<P: Into<Vi2d>>(&mut self, pos: P, size: P, col: Color) {
        let Vi2d { x, y } = pos.into();
        let Vi2d { x: w, y: h } = size.into() - Vi2d { x: 1, y: 1 };
        self.draw_line((x, y), (x + w, y), col);
        self.draw_line((x + w, y), (x + w, y + h), col);
        self.draw_line((x + w, y + h), (x, y + h), col);
        self.draw_line((x, y + h), (x, y), col);
    }

    /// Fill a rectangle with the top left corner at `(x, y)`
    /// and the bottom right corner at `(x + w, y + h)` (both inclusive)
    fn fill_rect<P: Into<Vi2d>>(&mut self, pos: P, size: P, col: Color) {
        let Vi2d { x, y } = pos.into();
        let Vi2d { x: w, y: h } = size.into();
        for nx in x..(x + w) {
            self.draw_line((nx, y), (nx, y + h), col);
        }
    }

    /// Draw a circle with center `(x, y)` and raduis `r`
    fn draw_circle<P: Into<Vi2d>>(&mut self, pos: P, r: u32, col: Color) {
        let Vi2d { x, y } = pos.into();
        let r_i32: i32 = r.try_into().unwrap();
        let x = x as i32;
        let y = y as i32;
        let mut x0: i32 = 0;
        let mut y0: i32 = r_i32;
        let mut d: i32 = 3i32 - 2i32 * r_i32;
        if r == 0 {
            return;
        }
        while y0 >= x0 {
            self.draw(((x + x0), (y - y0)), col);
            self.draw(((x + y0), (y - x0)), col);
            self.draw(((x + y0), (y + x0)), col);
            self.draw(((x + x0), (y + y0)), col);

            self.draw(((x - x0), (y + y0)), col);
            self.draw(((x - y0), (y + x0)), col);
            self.draw(((x - y0), (y - x0)), col);
            self.draw(((x - x0), (y - y0)), col);

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
    fn fill_circle<P: Into<Vi2d>>(&mut self, pos: P, r: u32, col: Color) {
        let Vi2d { x, y } = pos.into();
        let r_i32: i32 = r.try_into().unwrap();
        let x = x as i32;
        let y = y as i32;
        let mut x0: i32 = 0;
        let mut y0: i32 = r_i32;
        let mut d: i32 = 3 - 2 * r_i32;
        if r == 0 {
            return;
        }
        while y0 >= x0 {
            self.draw_line((x - x0, y - y0), (x + x0, y - y0), col);
            self.draw_line((x - y0, y - x0), (x + y0, y - x0), col);
            self.draw_line((x - x0, y + y0), (x + x0, y + y0), col);
            self.draw_line((x - y0, y + x0), (x + y0, y + x0), col);
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
    fn draw_triangle<P: Into<Vi2d>>(&mut self, pts1: P, pts2: P, pts3: P, col: Color) {
        let pts1: Vi2d = pts1.into();
        let pts2: Vi2d = pts2.into();
        let pts3: Vi2d = pts3.into();
        self.draw_line(pts1, pts2, col);
        self.draw_line(pts1, pts3, col);
        self.draw_line(pts2, pts3, col);
    }

    /// Fill the given triangle
    fn fill_triangle<P: Into<Vi2d>>(&mut self, pts1: P, pts2: P, pts3: P, col: Color) {
        #![allow(clippy::cast_precision_loss)]
        use std::cmp::{max, min};

        let pts1: Vi2d = pts1.into();
        let pts2: Vi2d = pts2.into();
        let pts3: Vi2d = pts3.into();
        self.draw_triangle(pts1, pts2, pts3, col);

        let pts1 = (pts1.x as i32, pts1.y as i32);
        let pts2 = (pts2.x as i32, pts2.y as i32);
        let pts3 = (pts3.x as i32, pts3.y as i32);
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
                    self.draw((x, y), col);
                }
            }
        }
    }
}
/// A trait that will handle the drawing of Sprite onto the Target
pub trait SpriteTrait: SmartDrawingTrait {
    /// Draw a Sprite with the top left corner at `(x, y)`
    /// the flip arguement will allow fliping of the axis
    /// flip: (horizontal, vertical)
    /// scale is the scale of the result (must be >= 1)
    fn draw_sprite<P: Into<Vi2d>>(
        &mut self,
        pos: P,
        scale: u32,
        sprite: &Sprite,
        flip: (bool, bool),
    ) {
        #![allow(
            clippy::cast_precision_loss,
            clippy::cast_possible_wrap,
            clippy::cast_sign_loss
        )]
        let Vi2d { x, y } = pos.into();
        let (mut fxs, mut fxm) = (0i32, 1i32);
        let (mut fys, mut fym) = (0i32, 1i32);
        let mut fx: i32;
        let mut fy: i32;
        if flip.0 {
            fxs = sprite.width() as i32 - 1;
            fxm = -1;
        }
        if flip.1 {
            fys = sprite.height() as i32 - 1;
            fym = -1;
        }
        if scale > 1 {
            fx = fxs;
            for i in 0..(sprite.width() as i32) {
                fy = fys;
                for j in 0..(sprite.height() as i32) {
                    for is in 0..(scale as i32) {
                        for js in 0..(scale as i32) {
                            self.draw(
                                (x + i * (scale as i32) + is, y + j * (scale as i32) + js),
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
            for i in 0..(sprite.width() as i32) {
                fy = fys;
                for j in 0..(sprite.height() as i32) {
                    self.draw((x + i, y + j), sprite.get_pixel(fx as u32, fy as u32));
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
    fn draw_partial_sprite<P: Into<Vi2d>>(
        &mut self,
        coords: P,
        sprite: &Sprite,
        o: P,
        size: P,
        scale: u32,
        flip: (bool, bool),
    ) {
        #![allow(
            clippy::cast_precision_loss,
            clippy::cast_possible_wrap,
            clippy::cast_sign_loss
        )]
        let Vi2d { x, y } = coords.into();
        let Vi2d { x: ox, y: oy } = o.into();
        let Vi2d { x: w, y: h } = size.into();

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
                    for is in 0..(scale as i32) {
                        for js in 0..(scale as i32) {
                            self.draw(
                                (x + i * (scale as i32) + is, y + j * (scale as i32) + js),
                                sprite.get_pixel((fx + ox) as u32, (fy + oy) as u32),
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
                        (x + i, y + j),
                        sprite.get_pixel(fx as u32 + ox as u32, fy as u32 + oy as u32),
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
impl_trait!(DottedShapeTrait);
