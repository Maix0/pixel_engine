#![allow(
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation
)]

use std::{cell::UnsafeCell, convert::TryInto, mem::transmute, sync::Arc};

use once_cell::sync::OnceCell;
use parking_lot::{Mutex, RwLock, RwLockWriteGuard};

use crate::vector2::{Vi2d, Vu2d};

/// The Drawing Mode used
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PixelMode {
    /// Basic drawing mode
    /// The pixel data after the draw will be the same as the pixel given
    Normal,
    /// Using Alpha in the Draw, Will be more computaion heavy
    /// You should only activate when you need the alpha blending and then change it back
    Alpha,
    /// Will draw only if the alpha is equals to 255
    Mask,
}

/// Represent a Sprite
#[derive(Debug)]
pub struct Sprite {
    //raw: image::RgbaImage,
    /// The data of the sprite, wrapped in a UnsafeCell because there is a need to with the refs;
    raw: Box<UnsafeCell<[u8]>>,
    size: Vu2d,
    /// List of all of the sub sprites areas, to make sure there is no overlap
    areas: Mutex<slab::Slab<Area>>,
    /// Makes sures that the
    read_lock: Arc<RwLock<()>>,
}

impl std::clone::Clone for Sprite {
    fn clone(&self) -> Self {
        let raw = Self::boxed_slice_to_cell(self.get_read_lock().0.to_vec().into_boxed_slice());
        Self {
            size: self.size,
            raw,
            areas: Mutex::new(slab::Slab::new()),
            read_lock: Arc::new(RwLock::new(())),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Area {
    pub pos: Vu2d,
    pub size: Vu2d,
}

impl Area {
    fn overlap(&self, other: &Self) -> bool {
        (self.pos.x < other.pos.x && other.pos.x < self.pos.x + self.size.x)
            || (self.pos.y < other.pos.y && other.pos.y < self.pos.y + self.size.y)
            || (other.pos.x < self.pos.x && self.pos.x < other.pos.x + other.size.x)
            || (other.pos.y < self.pos.y && self.pos.y < other.pos.y + other.size.y)
    }
}

#[derive(Debug)]
pub struct SpriteMutRef<'spr> {
    /// A reference to the base sprite;
    spr: &'spr Sprite,
    /// Top left of the subsprite (in pixels)
    pos: Vu2d,
    /// Size fot the left subsprite (in pixels)
    size: Vu2d,
    /// To do pointer math
    /// this is the with (in pixel) of the sprite.
    spr_width: u32,
    /// Signal that a write/read is happening
    /// The contract is that when a subsprite is writing or reading, it will take a ReadGuard and
    /// when then sprite want to read from its data, it lock with an exclusive access so it is the
    /// only one accessing the data.
    write_lock: Arc<RwLock<()>>,
    /// The area index inside the sprite's areas field.
    index: usize,
}

unsafe impl Send for Sprite {}
unsafe impl<'spr> Send for SpriteMutRef<'spr> {}

impl<'spr> Drop for SpriteMutRef<'spr> {
    fn drop(&mut self) {
        self.spr.areas.lock().remove(self.index);
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct OverlappingError;

impl std::fmt::Display for OverlappingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tried to create overlapping subsprite")
    }
}

impl std::error::Error for OverlappingError {}

impl Sprite {
    pub fn size(&self) -> &Vu2d {
        &self.size
    }

    pub fn width(&self) -> u32 {
        self.size.x
    }

    pub fn height(&self) -> u32 {
        self.size.y
    }

    pub fn get_read_lock(&self) -> (&mut [u8], RwLockWriteGuard<'_, ()>) {
        let lock = self.read_lock.write();
        // SAFETY: We have exclusive access as the contract is everyone that wishese to write to
        // the sprite gets a ReadGuard, and when you want to read from the whole sprite you get a
        // WriteGuard (so it is exclusive) to the data behind self.raw
        // We also return the lock so we keep the exclusive access.
        //
        (unsafe { &mut *self.raw.get() }, lock)
    }

    fn image_to_boxedslice(img: image::RgbaImage) -> Box<UnsafeCell<[u8]>> {
        Self::boxed_slice_to_cell(img.into_raw().into_boxed_slice())
    }

    /// Transmute a owned slice to an owned `UnsafeCell` wrapped slice
    pub(crate) fn boxed_slice_to_cell(slice: Box<[u8]>) -> Box<UnsafeCell<[u8]>> {
        // This is because we can't have the slice on the stack to create a Boxed slice and there
        // is no .map for a Box.
        // Transmuting the Box<[u8]> is fine because UnsafeCell is #[repr(transparent)]
        unsafe { transmute(slice) }
    }
    /// Load an rgba slice as an Sprite, will clone the slice
    /// the dimentions are in pixel
    ///
    /// # Errors
    ///
    /// if the slice isn't an rgba slice it will return an `Err`
    /// this means that `slice.len() % 4 = 0` and that `slice.len() = width * height * 4`
    pub fn load_rgba(rgba: &[u8], width: usize, height: usize) -> Result<Self, String> {
        if rgba.len() % 4 != 0 || rgba.len() != width * height * 4 {
            Err("Wrong Image len".to_string())
        } else {
            Ok(Self {
                size: Vu2d {
                    x: width.try_into().expect("Image too large"),
                    y: height.try_into().expect("Image too large"),
                },
                raw: Self::boxed_slice_to_cell(rgba.to_vec().into_boxed_slice()),
                areas: Mutex::new(slab::Slab::new()),
                read_lock: Arc::new(RwLock::new(())),
            })
        }
    }

    /// Creates a subsprite on a given parent sprite
    /// The real position will be clamped to the sprite dimentions, so if the `pos` has some
    /// negative parts, it will be clipped, only showing the positive part.
    /// Same with if the `size + pos` goes outside of the sprite.
    ///
    /// # Errors
    ///
    /// This will return an error if an subsprite already exists and that it overlaps
    #[allow(clippy::cast_sign_loss)]
    pub fn create_sub_sprite(
        &self,
        pos: Vi2d,
        mut size: Vu2d,
    ) -> Result<SpriteMutRef<'_>, OverlappingError> {
        let real_pos = Vu2d {
            x: if pos.x < 0 {
                size.x -= (-pos.x) as u32;
                0
            } else {
                (pos.x as u32).min(self.size.x)
            },
            y: if pos.y < 0 {
                size.y -= (-pos.y) as u32;
                0
            } else {
                (pos.y as u32).min(self.size.y)
            },
        };

        size.x = size.x.min(self.size.x - real_pos.x);
        size.y = size.y.min(self.size.y - real_pos.y);

        let area = Area {
            pos: real_pos,
            size,
        };
        let mut lock = self.areas.lock();

        let overlap = lock.iter().any(|(_, a)| a.overlap(&area));

        if overlap {
            return Err(OverlappingError);
        };

        let index = lock.insert(area);

        // SAFETY:
        // We checked that the subsprite is inside the main sprite
        // We checked that there is no overlap$
        // We added the area to the list
        unsafe { Ok(self.create_sub_sprite_unchecked(real_pos, size, index)) }
    }

    /// Creates a subsprite without any checks
    ///
    /// # Safety
    ///
    /// - You need to make sure there is no overlap with another subsprite
    /// - You need to make sure that the size is correct and fit inside the base sprites
    /// - You need to make sure that the area has been inserted in the area list and that the given
    /// index is the one pointing to the subsprite's area;
    pub unsafe fn create_sub_sprite_unchecked(
        &self,
        pos: Vu2d,
        size: Vu2d,
        index: usize,
    ) -> SpriteMutRef<'_> {
        SpriteMutRef {
            spr: self,
            size,
            pos,
            spr_width: self.width(),
            index,
            write_lock: self.read_lock.clone(),
        }
    }

    /// This is used when using [`create_sub_sprite_unchecked`()](Sprite::create_sub_sprite_unchecked)
    ///
    /// # Safety
    ///
    /// Editing this could lead to overlapping subsprite so be careful when handling the list
    /// (slab) of areas.
    pub unsafe fn get_areas(&self) -> &Mutex<slab::Slab<Area>> {
        &self.areas
    }

    /// Load an image from bytes, will clone the slice
    ///
    /// # Errors
    ///
    /// If the slice isn't an valid image format handled by the image crate, returns an error
    pub fn load_image_bytes(bytes: &[u8]) -> Result<Self, String> {
        let img = image::load_from_memory(bytes)
            .map_err(|err| err.to_string())?
            .to_rgba8();

        Ok(Sprite {
            size: Vu2d {
                x: (&img).width(),
                y: (&img).height(),
            },
            raw: Self::image_to_boxedslice(img),
            areas: Mutex::new(slab::Slab::new()),
            read_lock: Arc::new(RwLock::new(())),
        })
    }

    ///Load a image file and return a Sprite object representing that image
    /// # Errors
    ///
    /// If the file isn't an valid image format handled by the image crate or if the file IO failed, returns an error
    pub fn load_from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Sprite, String> {
        let img = image::open(path).map_err(|err| err.to_string())?.to_rgba8();

        Ok(Sprite {
            size: Vu2d {
                x: img.width(),
                y: img.height(),
            },
            raw: Self::image_to_boxedslice(img),
            areas: Mutex::new(slab::Slab::new()),
            read_lock: Arc::new(RwLock::new(())),
        })
    }
    /// Create [Sprite] with a size of 1x1
    #[must_use]
    pub fn new_blank() -> Sprite {
        Sprite {
            size: Vu2d { x: 1, y: 1 },
            raw: Self::boxed_slice_to_cell(vec![0x00; 4].into_boxed_slice()),
            areas: Mutex::new(slab::Slab::new()),
            read_lock: Arc::new(RwLock::new(())),
        }
    }
    /// Create [Sprite] with given size and [Color]
    #[must_use]
    pub fn new_with_color(w: u32, h: u32, col: Color) -> Self {
        Sprite {
            size: Vu2d { x: w, y: h },
            raw: Self::boxed_slice_to_cell(
                vec![col.r, col.g, col.b, col.a]
                    .repeat(w as usize * h as usize)
                    .into_boxed_slice(),
            ),
            areas: Mutex::new(slab::Slab::new()),
            read_lock: Arc::new(RwLock::new(())),
            //image::ImageBuffer::from_raw(w, h, raw) /*; (w * h) as usize]) */
            //.unwrap(), // as image::RgbaImage,
        }
    }
    /// Create a blank [Sprite] with given size
    #[must_use]
    pub fn new(w: u32, h: u32) -> Sprite {
        Sprite {
            size: Vu2d { x: w, y: h },
            raw: Self::boxed_slice_to_cell(
                vec![0x00; w as usize * h as usize * 4].into_boxed_slice(),
            ),
            areas: Mutex::new(slab::Slab::new()),
            read_lock: Arc::new(RwLock::new(())),
        }
    }
    /// Set pixel's [Color] on a [Sprite]
    pub fn set_pixel(&mut self, x: u32, y: u32, col: Color) {
        let width = self.width();
        if y >= self.height() || x >= width {
            return;
        }

        self.raw.get_mut()[(y * width + x) as usize * 4] = col.r;
        self.raw.get_mut()[(y * width + x) as usize * 4 + 1] = col.g;
        self.raw.get_mut()[(y * width + x) as usize * 4 + 2] = col.b;
        self.raw.get_mut()[(y * width + x) as usize * 4 + 3] = col.a;
    }
    /// Return the [Color] of the pixel at given coordinates, if it exist
    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        let mut col = Color::BLANK;
        if x >= self.width() || y >= self.height() {
            return col;
        }
        let (raw, lock) = self.get_read_lock();

        col.r = raw[(y * self.width() + x) as usize * 4];
        col.g = raw[(y * self.width() + x) as usize * 4 + 1];
        col.b = raw[(y * self.width() + x) as usize * 4 + 2];
        col.a = raw[(y * self.width() + x) as usize * 4 + 3];
        drop(lock);
        col
    }
    /// Return the [Color] of the pixel at given sample
    /// This sampling will take only the fractional part of the given sample coordinates, so it is
    /// effictivly from [O; 1)
    pub fn get_sample(&self, x: f64, y: f64) -> Color {
        let x = x.fract();
        let y = y.fract();
        let sample_x = ((x * f64::from(self.width())) as u32).min(self.width() - 1);
        let sample_y = ((y * f64::from(self.height())) as u32).min(self.height() - 1);
        self.get_pixel(sample_x, sample_y)
    }
}

impl<'spr> SpriteMutRef<'spr> {
    fn get_nth_ptr(&self, row: u32) -> *mut u8 {
        unsafe {
            let base_offset = (self.pos.y * self.spr_width + self.pos.x) as usize * 4;
            let base_ptr = self.spr.raw.get().cast::<u8>().add(base_offset);
            base_ptr.add((self.spr_width * row * 4) as usize)
        }
    }

    fn get_nth_slice(&self, row: u32) -> &'spr [u8] {
        unsafe {
            std::slice::from_raw_parts(self.get_nth_ptr(row) as *const u8, self.size.x as usize * 4)
        }
    }
    fn get_nth_slice_mut(&mut self, row: u32) -> &'spr mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.get_nth_ptr(row), self.size.x as usize * 4) }
    }

    pub fn get_pixel(&self, p: impl Into<Vu2d>) -> Option<Color> {
        let pos = p.into();
        if pos.x >= self.size.x || pos.y >= self.size.y {
            return None;
        }
        let slice = self.get_nth_slice(pos.y);
        Some(
            [
                slice[pos.x as usize * 4],
                slice[pos.x as usize * 4 + 1],
                slice[pos.x as usize * 4 + 2],
                slice[pos.x as usize * 4 + 3],
            ]
            .into(),
        )
    }

    pub fn set_pixel(&mut self, p: impl Into<Vu2d>, col: impl Into<Color>) {
        let pos = p.into();
        let col = col.into();
        if pos.x >= self.size.x || pos.y >= self.size.y {
            return;
        }
        let slice = self.get_nth_slice_mut(pos.y);
        let lock = self.write_lock.read();

        slice[pos.x as usize * 4] = col.r;
        slice[pos.x as usize * 4 + 1] = col.g;
        slice[pos.x as usize * 4 + 2] = col.b;
        slice[pos.x as usize * 4 + 3] = col.a;
        drop(lock); // I like to explicitly drop the lock
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
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 255 }
    }
    /// Return a [Color] where alpha is also a argument
    #[must_use]
    pub const fn new_with_alpha(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
    /// White [Color]
    pub const WHITE: Color = Color::new(255, 255, 255);
    /// Gray [Color]
    pub const GREY: Color = Color::new(192, 192, 192);
    /// Dark Grey [Color]
    pub const DARK_GREY: Color = Color::new(128, 128, 128);
    /// Very Dark Grey [Color]
    pub const VERY_DARK_GREY: Color = Color::new(64, 64, 64);
    /// Red [Color]
    pub const RED: Color = Color::new(255, 0, 0);
    /// Dark Red [Color]
    pub const DARK_RED: Color = Color::new(128, 0, 0);
    /// Very Dark Red [Color]
    pub const VERY_DARK_RED: Color = Color::new(64, 0, 0);
    /// Yellow [Color]
    pub const YELLOW: Color = Color::new(255, 255, 0);
    /// Dark Yellow [Color]
    pub const DARK_YELLOW: Color = Color::new(128, 128, 0);
    /// Very Dark Yellow [Color]
    pub const VERY_DARK_YELLOW: Color = Color::new(64, 64, 0);
    /// Green [Color]
    pub const GREEN: Color = Color::new(0, 255, 0);
    /// Dark Green [Color]
    pub const DARK_GREEN: Color = Color::new(0, 128, 0);
    /// Very Dark Green [Color]
    pub const VERY_DARK_GREEN: Color = Color::new(0, 64, 0);
    /// Cyan [Color]
    pub const CYAN: Color = Color::new(0, 255, 255);
    /// Dark Cyan [Color]
    pub const DARK_CYAN: Color = Color::new(0, 128, 128);
    /// Very Dark Cyan [Color]
    pub const VERY_DARK_CYAN: Color = Color::new(0, 64, 64);
    /// Blue [Color]
    pub const BLUE: Color = Color::new(0, 0, 255);
    /// Dark Blue [Color]
    pub const DARK_BLUE: Color = Color::new(0, 0, 128);
    /// Very Dark Blue [Color]
    pub const VERY_DARK_BLUE: Color = Color::new(0, 0, 64);
    /// Magenta [Color]
    pub const MAGENTA: Color = Color::new(255, 0, 255);
    /// Dark Magenta [Color]
    pub const DARK_MAGENTA: Color = Color::new(128, 0, 128);
    /// Very Dark Magenta [Color]
    pub const VERY_DARK_MAGENTA: Color = Color::new(64, 0, 64);
    /// Black [Color]
    pub const BLACK: Color = Color::new(0, 0, 0);
    /// Blank [Color]
    pub const BLANK: Color = Color::new_with_alpha(0, 0, 0, 0);
}

#[derive(Debug, Clone)]
struct DrawData {
    pixel_mode: PixelMode,
    blend_factor: f32,
}

#[derive(Clone, Debug)]
pub struct DrawingSprite<S: DrawSpriteTrait> {
    draw_data: DrawData,
    sprite: S,
}
impl<S: DrawSpriteTrait> DrawSpriteTrait for DrawingSprite<S> {
    fn size(&self) -> Vu2d {
        self.sprite.size()
    }

    fn get_pixel(&self, pos: Vi2d) -> Option<Color> {
        self.sprite.get_pixel(pos)
    }

    fn set_pixel(&mut self, pos: Vi2d, col: Color) {
        self.sprite.set_pixel(pos, col);
    }

    unsafe fn get_pixel_unchecked(&self, pos: Vu2d) -> Color {
        self.sprite.get_pixel_unchecked(pos)
    }

    unsafe fn set_pixel_unchecked(&mut self, pos: Vu2d, col: Color) {
        self.sprite.set_pixel_unchecked(pos, col);
    }
}

impl<S: DrawSpriteTrait> crate::traits::SmartDrawingTrait for DrawingSprite<S> {
    fn get_size(&self) -> Vu2d {
        self.sprite.size()
    }

    fn get_pixel<P: Into<Vi2d>>(&self, pos: P) -> Option<Color> {
        let pos = pos.into();
        self.sprite.get_pixel(pos)
    }
    fn draw<P: Into<Vi2d>>(&mut self, pos: P, col: Color) {
        let pixel_mode = self.get_pixel_mode();
        let blend_factor = self.get_blend_factor();
        let pos @ Vi2d { x, y } = pos.into();
        if x >= self.sprite.size().x.try_into().unwrap()
            || y >= self.sprite.size().y.try_into().unwrap()
            || x < 0
            || y < 0
        {
            return;
        }
        match pixel_mode {
            PixelMode::Normal => unsafe {
                self.sprite.set_pixel_unchecked(pos.cast_u32(), col);
            },
            PixelMode::Mask => {
                if col.a == 255 {
                    unsafe {
                        self.sprite.set_pixel_unchecked(pos.cast_u32(), col);
                    }
                }
            }
            PixelMode::Alpha => {
                let current_color: Color =
                    unsafe { self.sprite.get_pixel_unchecked(pos.cast_u32()) };
                let alpha: f32 = (f32::from(col.a) / 255.0f32) * blend_factor;
                let inverse_alpha: f32 = 1.0 - alpha;
                let red: f32 =
                    alpha * f32::from(col.r) + inverse_alpha * f32::from(current_color.r);
                let green: f32 =
                    alpha * f32::from(col.g) + inverse_alpha * f32::from(current_color.g);
                let blue: f32 =
                    alpha * f32::from(col.b) + inverse_alpha * f32::from(current_color.b);
                unsafe {
                    self.sprite
                        .set_pixel_unchecked(pos.cast_u32(), [red, green, blue].into());
                }
            }
        }
    }

    fn get_textsheet(&self) -> &'static Sprite {
        create_text()
    }

    fn clear(&mut self, col: Color) {
        let size = self.sprite.size();
        for y in 0..size.y {
            for x in 0..size.x {
                unsafe {
                    self.sprite.set_pixel_unchecked(Vu2d { x, y }, col);
                }
            }
        }
    }

    fn get_pixel_mode(&self) -> PixelMode {
        self.draw_data.pixel_mode
    }
    fn get_blend_factor(&self) -> f32 {
        self.draw_data.blend_factor
    }

    fn set_pixel_mode(&mut self, mode: PixelMode) {
        self.draw_data.pixel_mode = mode;
    }
    fn set_blend_factor(&mut self, f: f32) {
        self.draw_data.blend_factor = f;
    }
}

pub trait DrawSpriteTrait {
    fn get_pixel(&self, pos: Vi2d) -> Option<Color>;
    fn set_pixel(&mut self, pos: Vi2d, col: Color);
    fn size(&self) -> Vu2d;
    /// Get the pixel at the given location, but bypassing any bounds check
    ///
    /// # Safety
    ///     You must ensure that the pos in bounds
    unsafe fn get_pixel_unchecked(&self, pos: Vu2d) -> Color;
    /// Set the pixel at the given location, but bypassing any bounds check
    ///
    /// # Safety
    ///     You must ensure that the pos in bounds
    unsafe fn set_pixel_unchecked(&mut self, pos: Vu2d, col: Color);
}

impl DrawSpriteTrait for Sprite {
    fn get_pixel(&self, pos: Vi2d) -> Option<Color> {
        if pos.x < 0 || pos.y < 0 || pos.x as u32 >= self.size().x || pos.y as u32 >= self.size().y
        {
            return None;
        }
        let (raw, lock) = self.get_read_lock();
        let mut col = Color::BLANK;
        let pos = pos.cast_u32();

        col.r = raw[(pos.y * self.width() + pos.x) as usize * 4];
        col.g = raw[(pos.y * self.width() + pos.x) as usize * 4 + 1];
        col.b = raw[(pos.y * self.width() + pos.x) as usize * 4 + 2];
        col.a = raw[(pos.y * self.width() + pos.x) as usize * 4 + 3];
        drop(lock);
        Some(col)
    }

    fn set_pixel(&mut self, pos: Vi2d, col: Color) {
        if pos.x < 0 || pos.y < 0 {
            return;
        };
        self.set_pixel(pos.x as u32, pos.y as u32, col);
    }
    fn size(&self) -> Vu2d {
        *self.size()
    }

    /// # Safety
    /// its up to the caller to make sure that the given position is in bounds
    unsafe fn get_pixel_unchecked(&self, pos: Vu2d) -> Color {
        let (raw, _lock) = self.get_read_lock();
        let mut col = Color::BLANK;
        col.r = *raw.get_unchecked((pos.y * self.width() + pos.x) as usize * 4);
        col.g = *raw.get_unchecked((pos.y * self.width() + pos.x) as usize * 4 + 1);
        col.b = *raw.get_unchecked((pos.y * self.width() + pos.x) as usize * 4 + 2);
        col.a = *raw.get_unchecked((pos.y * self.width() + pos.x) as usize * 4 + 3);
        col
    }

    /// # Safety
    /// its up to the caller to make sure that the given position is in bounds
    unsafe fn set_pixel_unchecked(&mut self, pos: Vu2d, col: Color) {
        let Vu2d { x, y } = pos;
        let width = self.width();
        // SAFETY: Its up to the caller to make sure that the bounds are correct to not write oob
        *self
            .raw
            .get_mut()
            .get_unchecked_mut((y * width + x) as usize * 4) = col.r;
        *self
            .raw
            .get_mut()
            .get_unchecked_mut((y * width + x) as usize * 4 + 1) = col.g;
        *self
            .raw
            .get_mut()
            .get_unchecked_mut((y * width + x) as usize * 4 + 2) = col.b;
        *self
            .raw
            .get_mut()
            .get_unchecked_mut((y * width + x) as usize * 4 + 3) = col.a;
    }
}
impl<'spr> DrawSpriteTrait for SpriteMutRef<'spr> {
    fn get_pixel(&self, pos: Vi2d) -> Option<Color> {
        if pos.x < 0 || pos.y < 0 {
            return None;
        };
        self.get_pixel(pos.cast_u32())
    }

    fn set_pixel(&mut self, pos: Vi2d, col: Color) {
        if pos.x < 0 || pos.y < 0 {
            return;
        };
        self.set_pixel(pos.cast_u32(), col);
    }
    fn size(&self) -> Vu2d {
        self.size
    }

    /// # Safety
    /// its up to the caller to make sure that the given position is in bounds
    unsafe fn get_pixel_unchecked(&self, pos: Vu2d) -> Color {
        let slice = self.get_nth_slice(pos.y);

        [
            *slice.get_unchecked(pos.x as usize * 4),
            *slice.get_unchecked(pos.x as usize * 4 + 1),
            *slice.get_unchecked(pos.x as usize * 4 + 2),
            *slice.get_unchecked(pos.x as usize * 4 + 3),
        ]
        .into()
    }

    /// # Safety
    /// its up to the caller to make sure that the given position is in bounds
    unsafe fn set_pixel_unchecked(&mut self, pos: Vu2d, col: Color) {
        let slice = self.get_nth_slice_mut(pos.y);
        let lock = self.write_lock.read();

        *slice.get_unchecked_mut(pos.x as usize * 4) = col.r;
        *slice.get_unchecked_mut(pos.x as usize * 4 + 1) = col.g;
        *slice.get_unchecked_mut(pos.x as usize * 4 + 2) = col.b;
        *slice.get_unchecked_mut(pos.x as usize * 4 + 3) = col.a;
        drop(lock); // I like to explicitly drop the lock
    }
}

impl<S: DrawSpriteTrait> DrawingSprite<S> {
    pub fn new(spr: S) -> Self {
        DrawingSprite {
            draw_data: DrawData::new(),
            sprite: spr,
        }
    }
    pub fn into_inner(self) -> S {
        self.sprite
    }

    pub fn get_ref(&self) -> &S {
        &self.sprite
    }

    pub fn get_mut(&mut self) -> &S {
        &mut self.sprite
    }
}

impl DrawData {
    pub(crate) fn new() -> DrawData {
        Self {
            pixel_mode: PixelMode::Normal,
            blend_factor: 1.0f32,
        }
    }
}

fn create_text() -> &'static Sprite {
    struct ForceSendSync<T>(T);
    unsafe impl<T> Send for ForceSendSync<T> {}
    unsafe impl<T> Sync for ForceSendSync<T> {}

    static TEXT_FONT: &[u8] = b"?Q`0001oOch0o01o@F40o0<AGD4090LAGD<090@A7ch0?00O7Q`0600>00000000O000000nOT0063Qo4d8>?7a14Gno94AA4gno94AaOT0>o3`oO400o7QN00000400Of80001oOg<7O7moBGT7O7lABET024@aBEd714AiOdl717a_=TH013Q>00000000720D000V?V5oB3Q_HdUoE7a9@DdDE4A9@DmoE4A;Hg]oM4Aj8S4D84@`00000000OaPT1000Oa`^13P1@AI[?g`1@A=[OdAoHgljA4Ao?WlBA7l1710007l100000000ObM6000oOfMV?3QoBDD`O7a0BDDH@5A0BDD<@5A0BGeVO5ao@CQR?5Po00000000Oc``000?Ogij70PO2D]??0Ph2DUM@7i`2DTg@7lh2GUj?0TO0C1870T?0000000070<4001o?P<7?1QoHg43O;`h@GT0@:@LB@d0>:@hN@L0@?aoN@<0O7ao0000?000OcH0001SOglLA7mg24TnK7ln24US>0PL24U140PnOgl0>7QgOcH0K71S0000A00000H00000@Dm1S007@DUSg00?OdTnH7YhOfTL<7Yh@Cl0700?@Ah0300700000000<008001QL00ZA41a@6HnI<1i@FHLM81M@@0LG81?O`0nC?Y7?`0ZA7Y300080000O`082000Oh0827mo6>Hn?Wmo?6HnMb11MP08@C11H`08@FP0@@0004@00000000000P00001Oab00003OcKP0006@6=PMgl<@440MglH@000000`@000001P00000000Ob@8@@00Ob@8@Ga13R@8Mga172@8?PAo3R@827QoOb@820@0O`0007`0000007P0O`000P08Od400g`<3V=P0G`673IP0`@3>1`00P@6O`P00g`<O`000GP800000000?P9PL020O`<`N3R0@E4HC7b0@ET<ATB0@@l6C4B0O`H3N7b0?P01L3R000000020";
    static SPRITE_TEXTSHEET: OnceCell<ForceSendSync<Sprite>> = OnceCell::new();
    &SPRITE_TEXTSHEET
        .get_or_init(|| {
            let mut sheet = Sprite::new(128, 48);
            let mut px = 0;
            let mut py = 0;
            let chars = TEXT_FONT;
            for b in (0..1024).step_by(4) {
                let sym1 = u32::from(chars[b]) - 48;
                let sym2 = u32::from(chars[b + 1]) - 48;
                let sym3 = u32::from(chars[b + 2]) - 48;
                let sym4 = u32::from(chars[b + 3]) - 48;
                let r: u32 = sym1 << 18 | sym2 << 12 | sym3 << 6 | sym4;
                for i in 0..24 {
                    let k = if (r & (1 << i)) == 0 { 0 } else { 255 };
                    sheet.set_pixel(px, py, [k, k, k].into());
                    py += 1;
                    if py == 48 {
                        px += 1;
                        py = 0;
                    }
                }
            }
            ForceSendSync(sheet)
        })
        .0
}

impl From<[f32; 4]> for Color {
    fn from(col: [f32; 4]) -> Self {
        Color::new_with_alpha(
            (col[0].clamp(0f32, 1f32) * 255f32) as u8,
            (col[1].clamp(0f32, 1f32) * 255f32) as u8,
            (col[2].clamp(0f32, 1f32) * 255f32) as u8,
            (col[3].clamp(0f32, 1f32) * 255f32) as u8,
        )
    }
}
impl From<[f64; 4]> for Color {
    fn from(col: [f64; 4]) -> Self {
        Color::new_with_alpha(
            (col[0].clamp(0.0, 1.0) * 255f64) as u8,
            (col[1].clamp(0.0, 1.0) * 255f64) as u8,
            (col[2].clamp(0.0, 1.0) * 255f64) as u8,
            (col[3].clamp(0.0, 1.0) * 255f64) as u8,
        )
    }
}
impl From<[f32; 3]> for Color {
    fn from(col: [f32; 3]) -> Self {
        Color::new(
            (col[0].clamp(0.0, 1.0) * 255f32) as u8,
            (col[1].clamp(0.0, 1.0) * 255f32) as u8,
            (col[2].clamp(0.0, 1.0) * 255f32) as u8,
        )
    }
}
impl From<[f64; 3]> for Color {
    fn from(col: [f64; 3]) -> Self {
        Color::new(
            (col[0].clamp(0.0, 1.0) * 255f64) as u8,
            (col[1].clamp(0.0, 1.0) * 255f64) as u8,
            (col[2].clamp(0.0, 1.0) * 255f64) as u8,
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
        col.to_le_bytes().into()
    }
}

impl From<Color> for u32 {
    fn from(col: Color) -> Self {
        u32::from_le_bytes(col.into())
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
            f64::from(col.r) / 255f64,
            f64::from(col.g) / 255f64,
            f64::from(col.b) / 255f64,
            f64::from(col.a) / 255f64,
        ]
    }
}
impl From<Color> for [f64; 3] {
    fn from(col: Color) -> Self {
        [
            f64::from(col.r) / 255f64,
            f64::from(col.g) / 255f64,
            f64::from(col.b) / 255f64,
        ]
    }
}

impl From<Color> for [f32; 4] {
    fn from(col: Color) -> Self {
        [
            f32::from(col.r) / 255f32,
            f32::from(col.g) / 255f32,
            f32::from(col.b) / 255f32,
            f32::from(col.a) / 255f32,
        ]
    }
}
impl From<Color> for [f32; 3] {
    fn from(col: Color) -> Self {
        [
            f32::from(col.r) / 255f32,
            f32::from(col.g) / 255f32,
            f32::from(col.b) / 255f32,
        ]
    }
}
