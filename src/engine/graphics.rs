use memblock::*;
/// Represent a Sprite
#[derive(Debug, Clone)]
pub struct Sprite {
    //raw: image::RgbaImage,
    raw: MemBlock,
    width: u32,
    height: u32,
}

impl Sprite {
    fn image_to_memblock(img: image::RgbaImage) -> MemBlock {
        let mut memblock = MemBlock::new((img.width() as usize, img.height() as usize));
        let mut _a: &mut [u8] = &mut memblock;
        _a = &mut img.into_raw();
        memblock
    }

    ///Load a image file and return a Sprite object representing that image
    pub fn load_from_file(path: &std::path::Path) -> Result<Sprite, String> {
        let img = image::open(path).map_err(|err| err.to_string())?.to_rgba();
        //println!("{:?}", img.pixels().nth(((&img).width() / 2) as usize));

        Ok(Sprite {
            width: (&img).width(),
            height: (&img).height(),
            raw: Self::image_to_memblock(img),
        })
    }
    /// Create [Sprite] with a size of 1x1
    pub fn new_blank() -> Sprite {
        Sprite {
            width: 1,
            height: 1,
            raw: MemBlock::new((1, 1)),
        }
    }
    /// Create [Sprite] with given size and [Color]
    pub fn new_with_color(w: u32, h: u32, col: Color) -> Self {
        let memblock = MemBlock::new_with_value((w as usize, h as usize), col.into());
        Sprite {
            width: w,
            height: h,
            raw: memblock,
            //image::ImageBuffer::from_raw(w, h, raw) /*; (w * h) as usize]) */
            //.unwrap(), // as image::RgbaImage,
        }
    }
    /// Create a blank [Sprite] with given size
    pub fn new(w: u32, h: u32) -> Sprite {
        Sprite {
            width: w,
            height: h,
            raw: MemBlock::new((w as usize, h as usize)),
        }
    }
    /// Set pixel's [Color] on a [Sprite]
    pub fn set_pixel(&mut self, x: u32, y: u32, col: Color) {
        self.raw.write((x as usize, y as usize), col.into());
    }
    /// Return the [Color] of the pixel at given coordinates, if it exist
    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        if x >= self.width || y >= self.height {
            return Color::new_with_alpha(0, 0, 0, 0);
        }
        self.raw.read((x as usize, y as usize)).into()
    }
    /// Return the [Color] of the pixel at given sample
    /// It needs to be between 0.0 and 1.0 (both included)
    pub fn get_sample(&self, x: f64, y: f64) -> Color {
        if x < 0.0 || x > 1.0 || y < 0.0 || y > 1.0 {
            panic!("WTF ARE YOU DOING , SAMPLE NOT IN BOUND")
        }
        let sample_x = ((x * (self.width) as f64) as u32).min(self.width - 1);
        let sample_y = ((y * (self.height) as f64) as u32).min(self.height - 1);
        self.raw.read((sample_x as usize, sample_y as usize)).into()
    }
    /// Return the raw Image of the sprite
    pub fn get_raw(&self) -> MemBlock {
        self.raw.clone()
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
/// Represent a [Color] in a RGBA format
pub struct Color {
    /// Red part of the color
    pub r: u8,
    /// Green part of the color
    pub g: u8,
    /// Blue part of the color
    pub b: u8,
    /// Alpha part of the color
    pub a: u8,
}

impl Color {
    /// Return a [Color] with alpha set at 255
    pub const fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 255 }
    }
    /// Return a [Color] where alpha is also a argument
    pub const fn new_with_alpha(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
    /// Const [Color]
    pub const WHITE: Color = Color::new(255, 255, 255);
    /// Const [Color]
    pub const GREY: Color = Color::new(192, 192, 192);
    /// Const [Color]
    pub const DARK_GREY: Color = Color::new(128, 128, 128);
    /// Const [Color]
    pub const VERY_DARK_GREY: Color = Color::new(64, 64, 64);
    /// Const [Color]
    pub const RED: Color = Color::new(255, 0, 0);
    /// Const [Color]
    pub const DARK_RED: Color = Color::new(128, 0, 0);
    /// Const [Color]
    pub const VERY_DARK_RED: Color = Color::new(64, 0, 0);
    /// Const [Color]
    pub const YELLOW: Color = Color::new(255, 255, 0);
    /// Const [Color]
    pub const DARK_YELLOW: Color = Color::new(128, 128, 0);
    /// Const [Color]
    pub const VERY_DARK_YELLOW: Color = Color::new(64, 64, 0);
    /// Const [Color]
    pub const GREEN: Color = Color::new(0, 255, 0);
    /// Const [Color]
    pub const DARK_GREEN: Color = Color::new(0, 128, 0);
    /// Const [Color]
    pub const VERY_DARK_GREEN: Color = Color::new(0, 64, 0);
    /// Const [Color]
    pub const CYAN: Color = Color::new(0, 255, 255);
    /// Const [Color]
    pub const DARK_CYAN: Color = Color::new(0, 128, 128);
    /// Const [Color]
    pub const VERY_DARK_CYAN: Color = Color::new(0, 64, 64);
    /// Const [Color]
    pub const BLUE: Color = Color::new(0, 0, 255);
    /// Const [Color]
    pub const DARK_BLUE: Color = Color::new(0, 0, 128);
    /// Const [Color]
    pub const VERY_DARK_BLUE: Color = Color::new(0, 0, 64);
    /// Const [Color]
    pub const MAGENTA: Color = Color::new(255, 0, 255);
    /// Const [Color]
    pub const DARK_MAGENTA: Color = Color::new(128, 0, 128);
    /// Const [Color]
    pub const VERY_DARK_MAGENTA: Color = Color::new(64, 0, 64);
    /// Const [Color]
    pub const BLACK: Color = Color::new(0, 0, 0);
    /// Const [Color]
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

impl From<u32> for Color {
    fn from(col: u32) -> Self {
        u32_to_slice(col).into()
    }
}

impl From<Color> for u32 {
    fn from(col: Color) -> Self {
        slice_to_u32(col.into())
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

fn u32_to_slice(n: u32) -> [u8; 4] {
    [(n >> 24) as u8, (n >> 16) as u8, (n >> 8) as u8, n as u8]
}

fn slice_to_u32(n: [u8; 4]) -> u32 {
    (n[0] as u32) << 24 | (n[1] as u32) << 16 | (n[2] as u32) << 8 | (n[3] as u32)
}
