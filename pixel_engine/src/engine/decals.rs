#![allow(clippy::too_many_arguments, clippy::cast_precision_loss)]

use px_backend::decals;
use px_draw::graphics::Color;
use px_draw::vector2::Vf2d;

/// A sprite that lives on the GPU.
/// To not get a (GPU) memory leak, you need to destroy the decal manually through the method
/// `Engine::destroy_decal`
pub struct Decal(
    pub(crate) std::mem::ManuallyDrop<decals::Decal>,
    pub(crate) std::cell::Cell<bool>, // is the decal still valid ?
);

impl Decal {
    pub(crate) fn new(ctx: &mut px_backend::Context, spr: &px_draw::graphics::Sprite) -> Self {
        let (raw, _lock) = spr.get_read_lock();
        Decal(
            std::mem::ManuallyDrop::new(ctx.create_decal((raw, (spr.width(), spr.height())))),
            std::cell::Cell::new(true),
        )
    }

    pub(crate) fn clone_decal(&self) -> Self {
        Decal(self.0.clone(), self.1.clone())
    }

    /// Get the size of the decal in pixel
    #[must_use]
    pub fn size(&self) -> (u32, u32) {
        self.0.size
    }

    /// return `true` if the decal hasn't been destroyed, `false` if it was
    pub fn is_valid(&self) -> bool {
        self.1.get()
    }
}

impl std::fmt::Debug for Decal {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Decal").field("0", &"Inner Decal").finish()
    }
}

// Screen Space to normalized Space ((0,0)-game.size => (-1.0,-1.0)-(1.0,1.0))
// With the correct axis

macro_rules! normalize {
    ($pos:tt, $size:tt) => {{
        ((($pos) / $size * 2.0 - Vf2d { x: 1.0, y: 1.0 }) * Vf2d { x: 1.0, y: -1.0 })
    }};
}
macro_rules! into {
    ($var:tt) => {
        let $var: Vf2d = $var.into();
    };
    ($($v:tt),+) => {
        $(
            into!($v);
        )+
    }
}

/// Draw Decals
pub trait DecalDraw {
    /* Function signature copied from:
     * "https://github.com/OneLoneCoder/olcPixelGameEngine/blob/2beadab671b93a7547c261f010d2030548af2cf2/olcPixelGameEngine.h#L661"

    DrawDecal( vf2d& pos, Decal& decal, vf2d& scale = { 1.0f,1.0f }, Pixel& tint);

    // Draws fully user controlled 4 vertices, pos(pixels), uv(pixels), colours
    DrawExplicitDecal(Decal& decal, [vf2d;4] pos, [vf2d;4] uv, Pixel *col);

    // Draws a decal with 4 arbitrary points, warping the texture to look "correct"
    DrawWarpedDecal(Decal& decal, [vf2d;4] pos, Pixel& tint);
    DrawPartialWarpedDecal(Decal& decal, [vf2d;4] pos, vf2d source_pos, vf2d source_size)

    // Draws a decal rotated to specified angle, wit point of rotation offset
    DrawRotatedDecal( vf2d& pos, Decal& decal, f32 fAngle, vf2d& center = { 0.0f, 0.0f }, vf2d& scale, Pixel& tint);
    */

    /// Draw A decal with given position and uv (You probably don't want to use this)
    fn draw_explicit_decal<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: [P; 4],
        uv: [P; 4],
        decal: &Decal,
        tint: Color,
    );
    /// Draw a decal from the given position
    #[inline]
    fn draw_decal<P: Into<Vf2d> + Copy>(&mut self, pos: P, decal: &Decal) {
        self.draw_decal_tinted(pos, decal, Color::WHITE);
    }
    /// Draw a decal with a given scale
    #[inline]
    fn draw_decal_scaled<P: Into<Vf2d> + Copy>(&mut self, pos: P, decal: &Decal, scale: P) {
        self.draw_decal_scaled_tinted(pos, decal, scale, Color::WHITE);
    }
    /// Draw a partial decal from the given position
    #[inline]
    fn draw_partial_decal<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: P,
        decal: &Decal,
        source_pos: P,
        source_size: P,
    ) {
        self.draw_partial_decal_tinted(pos, decal, source_pos, source_size, Color::WHITE);
    }
    /// Draw a partial decal with a given scale
    #[inline]
    fn draw_partial_decal_scaled<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: P,
        decal: &Decal,
        source_pos: P,
        source_size: P,
        scale: P,
    ) {
        self.draw_partial_decal_scaled_tinted(
            pos,
            decal,
            source_pos,
            source_size,
            scale,
            Color::WHITE,
        );
    }

    /// Draw a decal where all Corner are given, this will set the uv correctly to allow texture
    /// warping
    /// The points are in order:
    ///
    /// `TopLeft`, `BottomLeft`, `BottomRight`, `TopRight`
    #[inline]
    fn draw_warped_decal<P: Into<Vf2d> + Copy>(&mut self, pos: [P; 4], decal: &Decal) {
        self.draw_warped_decal_tinted(pos, decal, Color::WHITE);
    }
    /// Draw a decal where all Corner are given, this will set the uv correctly to allow texture
    /// warping
    /// The points are in order:
    ///
    /// `TopLeft`, `BottomLeft`, `BottomRight`, `TopRight`
    #[inline]
    fn draw_warped_partial_decal<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: [P; 4],
        source_pos: P,
        source_size: P,
        decal: &Decal,
    ) {
        self.draw_warped_partial_decal_tinted(pos, source_pos, source_size, decal, Color::WHITE);
    }

    /// Draw a decal rotated `angle` radians around `center`
    /// center is an offset in pixel from the top left corner of the decal
    #[inline]
    fn draw_rotated_decal<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: P,
        decal: &Decal,
        angle: f32,
        center: P,
    ) {
        self.draw_rotated_decal_tinted(pos, decal, center, angle, Color::WHITE);
    }
    /// Same as `draw_rotated_decal` but with scaling
    #[inline]
    fn draw_rotated_decal_scaled<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: P,
        decal: &Decal,
        angle: f32,
        center: P,
        scale: P,
    ) {
        self.draw_rotated_decal_scaled_tinted(pos, decal, center, angle, scale, Color::WHITE);
    }
    /// Draw a zone of a decal and rotate it
    #[inline]
    fn draw_partial_rotated_decal<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: P,
        decal: &Decal,
        angle: f32,
        center: P,
        source_pos: P,
        source_size: P,
    ) {
        self.draw_partial_rotated_decal_tinted(
            pos,
            decal,
            angle,
            center,
            source_pos,
            source_size,
            Color::WHITE,
        );
    }
    /// Draw a zone of a decal, rotate it and scaled it
    #[inline]
    fn draw_partial_rotated_decal_scaled<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: P,
        decal: &Decal,
        angle: f32,
        center: P,
        source_pos: P,
        source_size: P,
        scaled: P,
    ) {
        self.draw_partial_rotated_decal_scaled_tinted(
            pos,
            decal,
            angle,
            center,
            source_pos,
            source_size,
            scaled,
            Color::WHITE,
        );
    }

    /// Same as the non tinted variant, but with an tint color parameter

    fn draw_decal_tinted<P: Into<Vf2d> + Copy>(&mut self, pos: P, decal: &Decal, tint: Color);

    /// Same as the non tinted variant, but with an tint color parameter

    fn draw_decal_scaled_tinted<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: P,
        decal: &Decal,
        scale: P,
        tint: Color,
    );
    /// Same as the non tinted variant, but with an tint color parameter

    fn draw_partial_decal_tinted<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: P,
        decal: &Decal,
        source_pos: P,
        source_size: P,
        tint: Color,
    );
    /// Same as the non tinted variant, but with an tint color parameter

    fn draw_partial_decal_scaled_tinted<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: P,
        decal: &Decal,
        source_pos: P,
        source_size: P,
        scale: P,
        tint: Color,
    );
    /// Same as the non tinted variant, but with an tint color parameter

    fn draw_warped_decal_tinted<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: [P; 4],
        decal: &Decal,
        tint: Color,
    );
    /// Same as the non tinted variant, but with an tint color parameter

    fn draw_warped_partial_decal_tinted<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: [P; 4],
        source_pos: P,
        source_size: P,
        decal: &Decal,
        tint: Color,
    );
    /// Same as the non tinted variant, but with an tint color parameter

    fn draw_rotated_decal_tinted<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: P,
        decal: &Decal,
        center: P,
        angle: f32,
        tint: Color,
    );
    /// Same as the non tinted variant, but with an tint color parameter

    fn draw_rotated_decal_scaled_tinted<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: P,
        decal: &Decal,
        center: P,
        angle: f32,
        scale: P,
        tint: Color,
    );
    /// Same as the non tinted variant, but with an tint color parameter

    fn draw_partial_rotated_decal_tinted<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: P,
        decal: &Decal,
        angle: f32,
        center: P,
        source_pos: P,
        source_size: P,
        tint: Color,
    );
    /// Same as the non tinted variant, but with an tint color parameter

    fn draw_partial_rotated_decal_scaled_tinted<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: P,
        decal: &Decal,
        angle: f32,
        center: P,
        source_pos: P,
        source_size: P,
        scaled: P,
        tint: Color,
    );
}

impl DecalDraw for crate::Engine {
    #[inline]
    fn draw_explicit_decal<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: [P; 4],
        uv: [P; 4],
        decal: &Decal,
        tint: Color,
    ) {
        if decal.is_valid() {
            let pos: [Vf2d; 4] = [pos[0].into(), pos[1].into(), pos[2].into(), pos[3].into()];
            let uv: [Vf2d; 4] = [uv[0].into(), uv[1].into(), uv[2].into(), uv[3].into()];
            let mut di = px_backend::decals::DecalInstances {
                id: decal.0.id(),
                pos: [(0.0, 0.0), (0.0, 0.0), (0.0, 0.0), (0.0, 0.0)],
                uv: [(0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (1.0, 0.0)],
                w: [1.0; 4],
                tint: tint.into(),
            };
            for i in 0..4 {
                di.pos[i] = (
                    (pos[i].x / (self.size.0) as f32) * 2.0 - 1.0,
                    ((pos[i].y / (self.size.1) as f32) * 2.0 - 1.0) * -1.0,
                );
                di.uv[i] = (uv[i].x, uv[i].y);
            }
            self.handler.draw_decal_instance(di);
        }
    }
    fn draw_decal<P: Into<Vf2d> + Copy>(&mut self, pos: P, decal: &Decal) {
        self.draw_decal_tinted(pos, decal, Color::WHITE);
    }
    fn draw_decal_scaled<P: Into<Vf2d> + Copy>(&mut self, pos: P, decal: &Decal, scale: P) {
        self.draw_decal_scaled_tinted(pos, decal, scale, Color::WHITE);
    }
    fn draw_partial_decal<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: P,
        decal: &Decal,
        source_pos: P,
        source_size: P,
    ) {
        self.draw_partial_decal_tinted(pos, decal, source_pos, source_size, Color::WHITE);
    }

    fn draw_partial_decal_scaled<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: P,
        decal: &Decal,
        source_pos: P,
        source_size: P,
        scale: P,
    ) {
        self.draw_partial_decal_scaled_tinted(
            pos,
            decal,
            source_pos,
            source_size,
            scale,
            Color::WHITE,
        );
    }
    fn draw_warped_decal<P: Into<Vf2d> + Copy>(&mut self, pos: [P; 4], decal: &Decal) {
        self.draw_warped_decal_tinted(pos, decal, Color::WHITE);
    }

    fn draw_warped_partial_decal<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: [P; 4],
        source_pos: P,
        source_size: P,
        decal: &Decal,
    ) {
        self.draw_warped_partial_decal_tinted(pos, source_pos, source_size, decal, Color::WHITE);
    }

    fn draw_rotated_decal<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: P,
        decal: &Decal,
        angle: f32,
        center: P,
    ) {
        self.draw_rotated_decal_tinted(pos, decal, center, angle, Color::WHITE);
    }
    fn draw_rotated_decal_scaled<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: P,
        decal: &Decal,
        angle: f32,
        center: P,
        scale: P,
    ) {
        self.draw_rotated_decal_scaled_tinted(pos, decal, center, angle, scale, Color::WHITE);
    }
    fn draw_partial_rotated_decal<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: P,
        decal: &Decal,
        angle: f32,
        center: P,
        source_pos: P,
        source_size: P,
    ) {
        self.draw_partial_rotated_decal_tinted(
            pos,
            decal,
            angle,
            center,
            source_pos,
            source_size,
            Color::WHITE,
        );
    }
    fn draw_partial_rotated_decal_scaled<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: P,
        decal: &Decal,
        angle: f32,
        center: P,
        source_pos: P,
        source_size: P,
        scaled: P,
    ) {
        self.draw_partial_rotated_decal_scaled_tinted(
            pos,
            decal,
            angle,
            center,
            source_pos,
            source_size,
            scaled,
            Color::WHITE,
        );
    }

    #[inline]
    fn draw_decal_tinted<P: Into<Vf2d> + Copy>(&mut self, pos: P, decal: &Decal, tint: Color) {
        if decal.is_valid() {
            into!(pos);
            let screen_size: Vf2d = (self.size.0 as f32, self.size.1 as f32).into();
            let topleft = normalize!(pos, screen_size);
            let bottomright = normalize!(
                {
                    pos + Vf2d {
                        x: decal.0.size.0 as f32,
                        y: decal.0.size.1 as f32,
                    }
                },
                screen_size
            );
            self.handler
                .draw_decal_instance(px_backend::decals::DecalInstances {
                    id: decal.0.id(),
                    pos: [
                        (topleft.x, topleft.y),         // A
                        (topleft.x, bottomright.y),     // B
                        (bottomright.x, bottomright.y), // C
                        (bottomright.x, topleft.y),     // D
                    ],
                    uv: [(0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (1.0, 0.0)],
                    w: [1.0; 4],
                    tint: tint.into(),
                });
        }
    }

    fn draw_decal_scaled_tinted<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: P,
        decal: &Decal,
        scale: P,
        tint: Color,
    ) {
        if decal.is_valid() {
            into!(scale, pos);
            let screen_size: Vf2d = (self.size.0 as f32, self.size.1 as f32).into();
            let topleft = normalize!(pos, screen_size);
            let bottomright = normalize!(
                {
                    pos + Vf2d {
                        x: decal.0.size.0 as f32,
                        y: decal.0.size.1 as f32,
                    } * scale
                },
                screen_size
            );
            self.handler
                .draw_decal_instance(px_backend::decals::DecalInstances {
                    id: decal.0.id(),
                    pos: [
                        (topleft.x, topleft.y),         // A
                        (topleft.x, bottomright.y),     // B
                        (bottomright.x, bottomright.y), // C
                        (bottomright.x, topleft.y),     // D
                    ],
                    uv: [(0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (1.0, 0.0)],
                    w: [1.0; 4],

                    tint: tint.into(),
                });
        }
    }

    #[inline]
    fn draw_partial_decal_tinted<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: P,
        decal: &Decal,
        source_pos: P,
        source_size: P,
        tint: Color,
    ) {
        if decal.is_valid() {
            into!(pos, source_pos, source_size);
            let screen_size: Vf2d = (self.size.0 as f32, self.size.1 as f32).into();
            let topleft = normalize!(pos, screen_size);
            let bottomright = normalize!(
                {
                    pos + Vf2d {
                        x: source_size.x,
                        y: source_size.x,
                    }
                },
                screen_size
            );
            let mut uv = [(0f32, 0f32); 4];
            let uv_scale: Vf2d = decal.0.uv_scale.into();
            let uv_topleft = source_pos * uv_scale;
            let uv_bottomright = uv_topleft + (source_size * uv_scale);
            uv[0] = (uv_topleft.x, uv_topleft.y);
            uv[1] = (uv_topleft.x, uv_bottomright.y);
            uv[2] = (uv_bottomright.x, uv_bottomright.y);
            uv[3] = (uv_bottomright.x, uv_topleft.y);

            self.handler
                .draw_decal_instance(px_backend::decals::DecalInstances {
                    id: decal.0.id(),
                    pos: [
                        (topleft.x, topleft.y),         // A
                        (topleft.x, bottomright.y),     // B
                        (bottomright.x, bottomright.y), // C
                        (bottomright.x, topleft.y),     // D
                    ],
                    uv,
                    w: [1.0; 4],
                    tint: tint.into(),
                });
        }
    }

    #[inline]
    fn draw_partial_decal_scaled_tinted<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: P,
        decal: &Decal,
        source_pos: P,
        source_size: P,
        scale: P,
        tint: Color,
    ) {
        if decal.is_valid() {
            into!(pos, source_pos, source_size, scale);
            let screen_size: Vf2d = (self.size.0 as f32, self.size.1 as f32).into();
            let topleft = normalize!(pos, screen_size);
            let bottomright = normalize!(
                {
                    pos + Vf2d {
                        x: source_size.x,
                        y: source_size.x,
                    } * scale
                },
                screen_size
            );
            let mut uv = [(0f32, 0f32); 4];
            let uv_scale: Vf2d = decal.0.uv_scale.into();
            let uv_topleft = source_pos * uv_scale;
            let uv_bottomright = uv_topleft + (source_size * uv_scale);
            uv[0] = (uv_topleft.x, uv_topleft.y);
            uv[1] = (uv_topleft.x, uv_bottomright.y);
            uv[2] = (uv_bottomright.x, uv_bottomright.y);
            uv[3] = (uv_bottomright.x, uv_topleft.y);

            self.handler
                .draw_decal_instance(px_backend::decals::DecalInstances {
                    id: decal.0.id(),
                    pos: [
                        (topleft.x, topleft.y),         // A
                        (topleft.x, bottomright.y),     // B
                        (bottomright.x, bottomright.y), // C
                        (bottomright.x, topleft.y),     // D
                    ],
                    uv,
                    w: [1.0; 4],
                    tint: tint.into(),
                });
        }
    }

    #[inline]
    fn draw_warped_decal_tinted<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: [P; 4],
        decal: &Decal,
        tint: Color,
    ) {
        if decal.is_valid() {
            const POINT_ONE: usize = 3;
            const POINT_TWO: usize = 0;
            const POINT_THREE: usize = 1;
            const POINT_FOUR: usize = 2;
            let screen_size: Vf2d = (self.size.0 as f32, self.size.1 as f32).into();
            let pos: [Vf2d; 4] = [pos[0].into(), pos[1].into(), pos[2].into(), pos[3].into()];
            let pos: [Vf2d; 4] = [
                normalize!({ pos[0] }, screen_size),
                normalize!({ pos[1] }, screen_size),
                normalize!({ pos[2] }, screen_size),
                normalize!({ pos[3] }, screen_size),
            ];
            let mut center: Vf2d = (0.0, 0.0).into();
            let mut di = px_backend::decals::DecalInstances {
                id: decal.0.id(),
                pos: [(0.0, 0.0), (0.0, 0.0), (0.0, 0.0), (0.0, 0.0)],
                uv: [(0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (1.0, 0.0)],
                w: [1.0; 4],
                tint: tint.into(),
            };
            let rd = (pos[POINT_THREE].x - pos[POINT_ONE].x)
                * (pos[POINT_FOUR].y - pos[POINT_TWO].y)
                - (pos[POINT_FOUR].x - pos[POINT_TWO].x) * (pos[POINT_THREE].y - pos[POINT_ONE].y);
            if rd != 0.0 {
                let rd = 1.0 / rd;
                let rn = ((pos[POINT_FOUR].x - pos[POINT_TWO].x)
                    * (pos[POINT_ONE].y - pos[POINT_TWO].y)
                    - (pos[POINT_FOUR].y - pos[POINT_TWO].y)
                        * (pos[POINT_ONE].x - pos[POINT_TWO].x))
                    * rd;
                let sn = ((pos[POINT_THREE].x - pos[POINT_ONE].x)
                    * (pos[POINT_ONE].y - pos[POINT_TWO].y)
                    - (pos[POINT_THREE].y - pos[POINT_ONE].y)
                        * (pos[POINT_ONE].x - pos[POINT_TWO].x))
                    * rd;
                if !(!(0.0..=1.0).contains(&rn) || !(0.0..=1.0).contains(&sn)) {
                    center = pos[POINT_ONE] + (pos[POINT_THREE] - pos[POINT_ONE]) * rn;
                };
                let mut d = [0.0; 4];
                for i in 0..4 {
                    d[i] = (pos[i] - center).mag();
                }
                for i in 0..4 {
                    let q = if d[i] == 0.0 {
                        1.0
                    } else {
                        (d[i] + d[(i + 2) & 3]) / d[(i + 2) & 3]
                    };
                    di.uv[i].0 *= q;
                    di.uv[i].1 *= q;
                    di.w[i] *= q;
                    di.pos[i] = (pos[i].x, pos[i].y);
                }
                self.handler.draw_decal_instance(di);
            }
        }
    }

    #[inline]
    fn draw_warped_partial_decal_tinted<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: [P; 4],
        source_pos: P,
        source_size: P,
        decal: &Decal,
        tint: Color,
    ) {
        if decal.is_valid() {
            const POINT_ONE: usize = 3;
            const POINT_TWO: usize = 0;
            const POINT_THREE: usize = 1;
            const POINT_FOUR: usize = 2;
            let screen_size: Vf2d = (self.size.0 as f32, self.size.1 as f32).into();
            into!(source_pos, source_size);
            let pos: [Vf2d; 4] = [pos[0].into(), pos[1].into(), pos[2].into(), pos[3].into()];
            let pos: [Vf2d; 4] = [
                normalize!({ pos[0] }, screen_size),
                normalize!({ pos[1] }, screen_size),
                normalize!({ pos[2] }, screen_size),
                normalize!({ pos[3] }, screen_size),
            ];
            let mut center: Vf2d = (0.0, 0.0).into();
            let mut di = px_backend::decals::DecalInstances {
                id: decal.0.id(),
                pos: [(0.0, 0.0), (0.0, 0.0), (0.0, 0.0), (0.0, 0.0)],
                uv: [(0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (1.0, 0.0)],
                w: [1.0; 4],
                tint: tint.into(),
            };
            let rd = (pos[POINT_THREE].x - pos[POINT_ONE].x)
                * (pos[POINT_FOUR].y - pos[POINT_TWO].y)
                - (pos[POINT_FOUR].x - pos[POINT_TWO].x) * (pos[POINT_THREE].y - pos[POINT_ONE].y);
            if rd != 0.0 {
                let uv_scale: Vf2d = decal.0.uv_scale.into();
                let uv_topleft = source_pos * uv_scale;
                let uv_bottomright = uv_topleft + (source_size * uv_scale);
                di.uv[0] = (uv_topleft.x, uv_topleft.y);
                di.uv[1] = (uv_topleft.x, uv_bottomright.y);
                di.uv[2] = (uv_bottomright.x, uv_bottomright.y);
                di.uv[3] = (uv_bottomright.x, uv_topleft.y);

                let rd = 1.0 / rd;
                let rn = ((pos[POINT_FOUR].x - pos[POINT_TWO].x)
                    * (pos[POINT_ONE].y - pos[POINT_TWO].y)
                    - (pos[POINT_FOUR].y - pos[POINT_TWO].y)
                        * (pos[POINT_ONE].x - pos[POINT_TWO].x))
                    * rd;
                let sn = ((pos[POINT_THREE].x - pos[POINT_ONE].x)
                    * (pos[POINT_ONE].y - pos[POINT_TWO].y)
                    - (pos[POINT_THREE].y - pos[POINT_ONE].y)
                        * (pos[POINT_ONE].x - pos[POINT_TWO].x))
                    * rd;
                if !(!(0.0..=1.0).contains(&rn) || !(0.0..=1.0).contains(&sn)) {
                    center = pos[POINT_ONE] + (pos[POINT_THREE] - pos[POINT_ONE]) * rn;
                };
                let mut d = [0.0; 4];
                for i in 0..4 {
                    d[i] = (pos[i] - center).mag();
                }
                for i in 0..4 {
                    let q = if d[i] == 0.0 {
                        1.0
                    } else {
                        (d[i] + d[(i + 2) & 3]) / d[(i + 2) & 3]
                    };
                    di.uv[i].0 *= q;
                    di.uv[i].1 *= q;
                    di.w[i] *= q;
                    di.pos[i] = (pos[i].x, pos[i].y);
                }
                self.handler.draw_decal_instance(di);
            }
        }
    }

    #[inline]
    fn draw_rotated_decal_tinted<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: P,
        decal: &Decal,
        center: P,
        angle: f32,
        tint: Color,
    ) {
        if decal.is_valid() {
            let screen_size: Vf2d = (self.size.0 as f32, self.size.1 as f32).into();
            into!(pos, center);
            let mut pos_arr = [Vf2d { x: 0.0, y: 0.0 }; 4];
            pos_arr[0] = Vf2d {x:0.0,y:0.0} - center /* * scale*/;
            pos_arr[1] = Vf2d {x: 0.0, y:  decal.size().1 as f32} - center /* * scale*/;
            pos_arr[2] = Vf2d {x:decal.size().0 as f32, y:decal.size().1 as f32} - center/*  * scale*/;
            pos_arr[3] = Vf2d {x:decal.size().0 as f32, y: 0.0} - center /* * scale*/;
            let (s, c) = angle.sin_cos();
            for pos_index in &mut pos_arr {
                *pos_index = normalize!(
                    {
                        pos + Vf2d {
                            x: pos_index.x * c - pos_index.y * s,
                            y: pos_index.x * s + pos_index.y * c,
                        }
                    },
                    screen_size
                );
                // *pos_index = pos
                //     + Vf2d {
                //         x: pos_index.x * c - pos_index.y * s,
                //         y: pos_index.x * s + pos_index.y * c,
                //     };
                // *pos_index = *pos_index * screen_size * 2.0 - Vf2d { x: 1.0, y: 1.0 };
                // pos_index.y *= -1.0;
                /*
                di.pos[i] = pos + olc::vf2d(di.pos[i].x * c - di.pos[i].y * s, di.pos[i].x * s + di.pos[i].y * c);
                di.pos[i] = di.pos[i] * vInvScreenSize * 2.0f - olc::vf2d(1.0f, 1.0f);
                di.pos[i].y *= -1.0f;
                di.w[i] = 1;
                */
            }
            self.handler.draw_decal_instance(decals::DecalInstances {
                id: decal.0.id(),
                pos: [
                    (pos_arr[0].x, pos_arr[0].y),
                    (pos_arr[1].x, pos_arr[1].y),
                    (pos_arr[2].x, pos_arr[2].y),
                    (pos_arr[3].x, pos_arr[3].y),
                ],
                w: [1.0; 4],
                uv: [(0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (1.0, 0.0)],
                tint: tint.into(),
            });
        }
    }

    #[inline]
    fn draw_rotated_decal_scaled_tinted<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: P,
        decal: &Decal,
        center: P,
        angle: f32,
        scale: P,
        tint: Color,
    ) {
        if decal.is_valid() {
            let screen_size: Vf2d = (self.size.0 as f32, self.size.1 as f32).into();
            into!(pos, center, scale);
            let mut pos_arr = [Vf2d { x: 0.0, y: 0.0 }; 4];
            pos_arr[0] = Vf2d {x:0.0,y:0.0} - center /* * scale*/;
            pos_arr[1] = Vf2d {x: 0.0, y:  decal.size().1 as f32} - center /* * scale*/;
            pos_arr[2] = Vf2d {x:decal.size().0 as f32, y:decal.size().1 as f32} - center/*  * scale*/;
            pos_arr[3] = Vf2d {x:decal.size().0 as f32, y: 0.0} - center /* * scale*/;
            for p in &mut pos_arr {
                *p = *p * scale;
            }
            let (c, s) = angle.sin_cos();
            for pos_index in &mut pos_arr {
                *pos_index = normalize!(
                    {
                        pos + Vf2d {
                            x: pos_index.x * c - pos_index.y * s,
                            y: pos_index.x * s + pos_index.y * c,
                        }
                    },
                    screen_size
                );
            }
            self.handler.draw_decal_instance(decals::DecalInstances {
                id: decal.0.id(),
                pos: [
                    (pos_arr[0].x, pos_arr[0].y),
                    (pos_arr[1].x, pos_arr[1].y),
                    (pos_arr[2].x, pos_arr[2].y),
                    (pos_arr[3].x, pos_arr[3].y),
                ],
                w: [1.0; 4],
                uv: [(0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (1.0, 0.0)],
                tint: tint.into(),
            });
        }
    }

    #[inline]
    fn draw_partial_rotated_decal_tinted<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: P,
        decal: &Decal,
        angle: f32,
        center: P,
        source_pos: P,
        source_size: P,
        tint: Color,
    ) {
        if decal.is_valid() {
            let screen_size: Vf2d = (self.size.0 as f32, self.size.1 as f32).into();

            let uv_scale: Vf2d = decal.0.uv_scale.into();
            into!(pos, center, source_pos, source_size);
            let uv_topleft = source_pos * uv_scale;
            let uv_bottomright = uv_topleft + (source_size * uv_scale);
            let mut uv = [(0f32, 0f32); 4];
            uv[0] = (uv_topleft.x, uv_topleft.y);
            uv[1] = (uv_topleft.x, uv_bottomright.y);
            uv[2] = (uv_bottomright.x, uv_bottomright.y);
            uv[3] = (uv_bottomright.x, uv_topleft.y);
            let mut pos_arr = [Vf2d { x: 0.0, y: 0.0 }; 4];
            pos_arr[0] = Vf2d {x:0.0,y:0.0} - center /* * scale*/;
            pos_arr[1] = Vf2d {x: 0.0, y:  source_size.y} - center /* * scale*/;
            pos_arr[2] = Vf2d {x:source_size.x, y: source_size.y } - center/*  * scale*/;
            pos_arr[3] = Vf2d {x:source_size.x, y: 0.0} - center /* * scale*/;
            let (c, s) = angle.sin_cos();
            for pos_index in &mut pos_arr {
                *pos_index = normalize!(
                    {
                        pos + Vf2d {
                            x: pos_index.x * c - pos_index.y * s,
                            y: pos_index.x * s + pos_index.y * c,
                        }
                    },
                    screen_size
                );
            }

            self.handler.draw_decal_instance(decals::DecalInstances {
                id: decal.0.id(),
                pos: [
                    (pos_arr[0].x, pos_arr[0].y),
                    (pos_arr[1].x, pos_arr[1].y),
                    (pos_arr[2].x, pos_arr[2].y),
                    (pos_arr[3].x, pos_arr[3].y),
                ],
                uv,
                w: [1.0; 4],
                tint: tint.into(),
            });
        }
    }

    #[inline]
    fn draw_partial_rotated_decal_scaled_tinted<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: P,
        decal: &Decal,
        angle: f32,
        center: P,
        source_pos: P,
        source_size: P,
        scaled: P,
        tint: Color,
    ) {
        if decal.is_valid() {
            into!(pos, center, source_pos, source_size, scaled);
            let screen_size: Vf2d = (self.size.0 as f32, self.size.1 as f32).into();
            let uv_scale: Vf2d = decal.0.uv_scale.into();
            let uv_topleft = source_pos * uv_scale;
            let uv_bottomright = uv_topleft + (source_size * uv_scale);
            let mut uv = [(0f32, 0f32); 4];
            uv[0] = (uv_topleft.x, uv_topleft.y);
            uv[1] = (uv_topleft.x, uv_bottomright.y);
            uv[2] = (uv_bottomright.x, uv_bottomright.y);
            uv[3] = (uv_bottomright.x, uv_topleft.y);
            let mut pos_arr = [Vf2d { x: 0.0, y: 0.0 }; 4];
            pos_arr[0] = Vf2d {x:0.0,y:0.0} - center /* * scale*/;
            pos_arr[1] = Vf2d {x: 0.0, y:  source_size.y} - center /* * scale*/;
            pos_arr[2] = Vf2d {x:source_size.x, y: source_size.y } - center/*  * scale*/;
            pos_arr[3] = Vf2d {x:source_size.x, y: 0.0} - center /* * scale*/;
            for p in &mut pos_arr {
                *p = *p * scaled;
            }
            let (c, s) = angle.sin_cos();
            for pos_index in &mut pos_arr {
                *pos_index = normalize!(
                    {
                        pos + Vf2d {
                            x: pos_index.x * c - pos_index.y * s,
                            y: pos_index.x * s + pos_index.y * c,
                        }
                    },
                    screen_size
                );
            }
            self.handler.draw_decal_instance(decals::DecalInstances {
                id: decal.0.id(),
                pos: [
                    (pos_arr[0].x, pos_arr[0].y),
                    (pos_arr[1].x, pos_arr[1].y),
                    (pos_arr[2].x, pos_arr[2].y),
                    (pos_arr[3].x, pos_arr[3].y),
                ],
                w: [1.0; 4],
                uv,
                tint: tint.into(),
            });
        }
    }
}

/// A trait that allows the rendering of text as decals
pub trait DecalText: DecalDraw {
    /// Draw the given string starting at the position given and scaled
    fn draw_text_decal(
        &mut self,
        pos: impl Into<Vf2d>,
        text: impl AsRef<str>,
        scale: impl Into<Vf2d>,
        color: impl Into<Color>,
    );
}

impl DecalText for crate::Engine {
    fn draw_text_decal(
        &mut self,
        pos: impl Into<Vf2d>,
        text: impl AsRef<str>,
        scale: impl Into<Vf2d>,
        color: impl Into<Color>,
    ) {
        let base_pos = pos.into();
        let mut pos = base_pos;
        let scale = scale.into();
        let color = color.into();
        let text = text.as_ref();
        let textsheet_decal = self.textsheet_decal.clone_decal();
        for chr in text.chars() {
            match chr {
                '\n' => {
                    pos.x = base_pos.x;
                    pos.y += scale.y * 8.0;
                }
                '\t' => {
                    pos.x += 4.0 * 8.0 * scale.x;
                }
                other if other.is_ascii() => {
                    let ox = (chr as u32 - 32) % 16;
                    let oy = (chr as u32 - 32) / 16;
                    self.draw_partial_decal_tinted(
                        pos,
                        &textsheet_decal,
                        (px_draw::vector2::Vu2d { x: ox, y: oy }).cast_f32() * 8.0,
                        Vf2d { x: 8.0, y: 8.0 },
                        color,
                    );
                    pos.x += scale.x * 8.0;
                }
                _ => pos.x += scale.x * 8.0,
            }
        }
        std::mem::forget(textsheet_decal);
    }
}
