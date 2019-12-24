#[derive(Debug, Clone)]
pub struct Sprite {
    raw: image::RgbaImage,
    width: u32,
    height: u32,
}

impl Sprite {
    ///Load a image file and return a Sprite object representing that image
    pub fn load_from_file(path: &std::path::Path) -> Result<Sprite, String> {
        let img = image::open(path).map_err(|err| err.to_string())?.to_rgba();
        //println!("{:?}", img.pixels().nth(((&img).width() / 2) as usize));
        Ok(Sprite {
            width: (&img).width(),
            height: (&img).height(),
            raw: img,
        })
    }
    pub fn new_blank() -> Sprite {
        Sprite {
            width: 1,
            height: 1,
            raw: image::ImageBuffer::from_raw(1, 1, vec![0_u8; 4]).unwrap() as image::RgbaImage,
        }
    }
    pub fn new_with_color(w: u32, h: u32, col: Color) -> Self {
        let mut raw = vec![0; (w * h * 4) as usize];
        for index in (0..(w * h * 4)).step_by(4) {
            raw[(index + 0) as usize] = col.r;
            raw[(index + 1) as usize] = col.g;
            raw[(index + 2) as usize] = col.b;
            raw[(index + 3) as usize] = col.a;
        }
        Sprite {
            width: w,
            height: h,
            raw: image::ImageBuffer::from_raw(w, h, raw) /*; (w * h) as usize]) */
                .unwrap() as image::RgbaImage,
        }
    }

    pub fn new(w: u32, h: u32) -> Sprite {
        Sprite {
            width: w,
            height: h,
            raw: image::ImageBuffer::from_raw(w, h, vec![0_u8; (w * h * 4) as usize]).unwrap()
                as image::RgbaImage,
        }
    }
    pub fn set_pixel(&mut self, x: u32, y: u32, col: Color) {
        self.raw.get_pixel_mut(x, y).0 = [col.r, col.g, col.b, col.a];
    }
    fn ptc(px: Option<&image::Rgba<u8>>) -> Option<Color> {
        match px {
            Some(px) => Some(Color::new_with_alpha(px[0], px[1], px[2], px[3])),
            None => None,
        }
    }
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<Color> {
        if x >= self.width || y >= self.height {
            return Some(Color::new_with_alpha(0, 0, 0, 0));
        }
        Sprite::ptc(Some(self.raw.get_pixel(x, y)))
    }
    pub fn get_sample(&self, x: f64, y: f64) -> Color {
        if x < 0.0 || x > 1.0 || y < 0.0 || y > 1.0 {
            panic!("WTF ARE YOU DOING , SAMPLE NOT IN BOUND")
        }
        let sample_x = ((x * (self.width) as f64) as u32).min(self.width - 1);
        let sample_y = ((y * (self.height) as f64) as u32).min(self.height - 1);
        Sprite::ptc(Some(self.raw.get_pixel(sample_x, sample_y))).unwrap()
    }
    pub fn get_raw(&self) -> image::RgbaImage {
        self.raw.clone()
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 255 }
    }
    pub const fn new_with_alpha(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
    pub const WHITE: Color = Color::new(255, 255, 255);
    pub const GREY: Color = Color::new(192, 192, 192);
    pub const DARK_GREY: Color = Color::new(128, 128, 128);
    pub const VERY_DARK_GREY: Color = Color::new(64, 64, 64);
    pub const RED: Color = Color::new(255, 0, 0);
    pub const DARK_RED: Color = Color::new(128, 0, 0);
    pub const VERY_DARK_RED: Color = Color::new(64, 0, 0);
    pub const YELLOW: Color = Color::new(255, 255, 0);
    pub const DARK_YELLOW: Color = Color::new(128, 128, 0);
    pub const VERY_DARK_YELLOW: Color = Color::new(64, 64, 0);
    pub const GREEN: Color = Color::new(0, 255, 0);
    pub const DARK_GREEN: Color = Color::new(0, 128, 0);
    pub const VERY_DARK_GREEN: Color = Color::new(0, 64, 0);
    pub const CYAN: Color = Color::new(0, 255, 255);
    pub const DARK_CYAN: Color = Color::new(0, 128, 128);
    pub const VERY_DARK_CYAN: Color = Color::new(0, 64, 64);
    pub const BLUE: Color = Color::new(0, 0, 255);
    pub const DARK_BLUE: Color = Color::new(0, 0, 128);
    pub const VERY_DARK_BLUE: Color = Color::new(0, 0, 64);
    pub const MAGENTA: Color = Color::new(255, 0, 255);
    pub const DARK_MAGENTA: Color = Color::new(128, 0, 128);
    pub const VERY_DARK_MAGENTA: Color = Color::new(64, 0, 64);
    pub const BLACK: Color = Color::new(0, 0, 0);
    pub const BLANK: Color = Color::new_with_alpha(0, 0, 0, 0);
}

impl From<[f32; 4]> for Color {
    fn from(col: [f32; 4]) -> Self {
        Color::new_with_alpha(
            (col[0] * 255f32) as u8,
            (col[1] * 255f32) as u8,
            (col[2] * 255f32) as u8,
            (col[3] * 255f32) as u8,
        )
    }
}
impl From<[f64; 4]> for Color {
    fn from(col: [f64; 4]) -> Self {
        Color::new_with_alpha(
            (col[0] * 255f64) as u8,
            (col[1] * 255f64) as u8,
            (col[2] * 255f64) as u8,
            (col[3] * 255f64) as u8,
        )
    }
}
impl From<[f32; 3]> for Color {
    fn from(col: [f32; 3]) -> Self {
        Color::new(
            (col[0] * 255f32) as u8,
            (col[1] * 255f32) as u8,
            (col[2] * 255f32) as u8,
        )
    }
}
impl From<[f64; 3]> for Color {
    fn from(col: [f64; 3]) -> Self {
        Color::new(
            (col[0] * 255f64) as u8,
            (col[1] * 255f64) as u8,
            (col[2] * 255f64) as u8,
        )
    }
}

impl From<[u8; 4]> for Color {
    fn from(col: [u8; 4]) -> Self {
        Color::new_with_alpha(col[0], col[1], col[2], col[3])
    }
}
impl From<[u8; 3]> for Color {
    fn from(col: [u8; 3]) -> Self {
        Color::new(col[0], col[1], col[2])
    }
}

impl From<Color> for [u8; 4] {
    fn from(col: Color) -> Self {
        [col.r, col.g, col.b, col.a]
    }
}
impl From<Color> for [u8; 3] {
    fn from(col: Color) -> Self {
        [col.r, col.g, col.b]
    }
}

impl From<Color> for [f64; 4] {
    fn from(col: Color) -> Self {
        [
            col.r as f64 / 255f64,
            col.g as f64 / 255f64,
            col.b as f64 / 255f64,
            col.a as f64 / 255f64,
        ]
    }
}
impl From<Color> for [f64; 3] {
    fn from(col: Color) -> Self {
        [
            col.r as f64 / 255f64,
            col.g as f64 / 255f64,
            col.b as f64 / 255f64,
        ]
    }
}

impl From<Color> for [f32; 4] {
    fn from(col: Color) -> Self {
        [
            col.r as f32 / 255f32,
            col.g as f32 / 255f32,
            col.b as f32 / 255f32,
            col.a as f32 / 255f32,
        ]
    }
}
impl From<Color> for [f32; 3] {
    fn from(col: Color) -> Self {
        [
            col.r as f32 / 255f32,
            col.g as f32 / 255f32,
            col.b as f32 / 255f32,
        ]
    }
}
