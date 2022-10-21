use px_draw::traits::SmartDrawingTrait;
  

impl px_draw::graphics::DrawSpriteTrait for crate::Engine {
    fn size(&self) -> pixel_engine_draw::vector2::Vu2d {
        self.screen.size()
    }

    fn get_pixel(
        &self,
        pos: pixel_engine_draw::vector2::Vi2d,
    ) -> Option<pixel_engine_draw::graphics::Color> {
        px_draw::graphics::DrawSpriteTrait::get_pixel(&self.screen, pos)
    }
    fn set_pixel(
        &mut self,
        pos: pixel_engine_draw::vector2::Vi2d,
        col: pixel_engine_draw::graphics::Color,
    ) {
        self.screen.set_pixel(pos, col)
    }
    unsafe fn get_pixel_unchecked(
        &self,
        pos: pixel_engine_draw::vector2::Vu2d,
    ) -> pixel_engine_draw::graphics::Color {
        self.screen.get_pixel_unchecked(pos)
    }

    unsafe fn set_pixel_unchecked(
        &mut self,
        pos: pixel_engine_draw::vector2::Vu2d,
        col: pixel_engine_draw::graphics::Color,
    ) {
        self.screen.set_pixel_unchecked(pos, col)
    }
}

impl SmartDrawingTrait for crate::Engine {
    fn draw<P: Into<pixel_engine_draw::vector2::Vi2d>>(
        &mut self,
        pos: P,
        col: pixel_engine_draw::graphics::Color,
    ) {
        self.screen.draw(pos, col)
    }

    fn clear(&mut self, col: pixel_engine_draw::graphics::Color) {
        self.screen.clear(col)
    }

    fn get_size(&self) -> pixel_engine_draw::vector2::Vu2d {
        self.screen.get_size()
    }

    fn set_pixel_mode(&mut self, mode: pixel_engine_draw::graphics::PixelMode) {
        self.screen.set_pixel_mode(mode)
    }

    fn get_pixel<P: Into<pixel_engine_draw::vector2::Vi2d>>(
        &self,
        pos: P,
    ) -> Option<pixel_engine_draw::graphics::Color> {
        self.screen.get_pixel(pos)
    }

    fn set_blend_factor(&mut self, f: f32) {
        self.screen.set_blend_factor(f)
    }

    fn get_textsheet(&self) -> &'static pixel_engine_draw::graphics::Sprite {
        self.screen.get_textsheet()
    }

    fn get_pixel_mode(&self) -> pixel_engine_draw::graphics::PixelMode {
        self.screen.get_pixel_mode()
    }

    fn get_blend_factor(&self) -> f32 {
        self.screen.get_blend_factor()
    }
}
