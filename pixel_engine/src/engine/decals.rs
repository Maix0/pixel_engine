use px_backend::decals;

pub struct Decal(pub(crate) decals::Decal);

impl Decal {
    pub(crate) fn new(ctx: &mut px_backend::Context, spr: &px_draw::graphics::Sprite) -> Self {
        Decal(ctx.create_decal((&spr.get_raw(), (spr.width, spr.height))))
    }
    pub fn size(&self) -> (u32, u32) {
        self.0.size
    }
}

impl std::fmt::Debug for Decal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Decal").field("0", &"Inner Decal").finish()
    }
}
use px_draw::vector2::Vf2d;
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
    fn draw_explicit_decal<P: Into<Vf2d> + Copy>(&mut self, pos: [P; 4], uv: [P; 4], decal: &Decal);
    /// Draw a decal from the given position
    fn draw_decal<P: Into<Vf2d> + Copy>(&mut self, pos: P, decal: &Decal);
    /// Draw a decal with a given scale
    fn draw_decal_scaled<P: Into<Vf2d> + Copy>(&mut self, pos: P, decal: &Decal, scale: P);

    /// Draw a partial decal from the given position
    fn draw_partial_decal<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: P,
        decal: &Decal,
        source_pos: P,
        source_size: P,
    );
    /// Draw a partial decal with a given scale
    fn draw_partial_decal_scaled<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: P,
        decal: &Decal,
        source_pos: P,
        source_size: P,
        scale: P,
    );

    /// Draw a decal where all Corner are given, this will set the uv correctly to allow texture
    /// warping
    /// The points are in order:
    ///
    /// TopLeft, BottomLeft, BottomRight, TopRight
    fn draw_warped_decal<P: Into<Vf2d> + Copy>(&mut self, pos: [P; 4], decal: &Decal);
    /// Draw a decal where all Corner are given, this will set the uv correctly to allow texture
    /// warping
    /// The points are in order:
    ///
    /// TopLeft, BottomLeft, BottomRight, TopRight
    fn draw_warped_partial_decal<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: [P; 4],
        source_pos: P,
        source_size: P,
        decal: &Decal,
    );

    /// Draw a decal rotated `angle` radians around `center`
    /// center is an offset in pixel from the top left corner of the decal
    fn draw_rotated_decal<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: P,
        decal: &Decal,
        angle: f32,
        center: P,
    );
    /// Same as `draw_rotated_decal` but with scaling
    fn draw_rotated_decal_scaled<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: P,
        decal: &Decal,
        angle: f32,
        center: P,
        scale: P,
    );
    /// Draw a zone of a decal and rotate it
    fn draw_partial_rotated_decal<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: P,
        decal: &Decal,
        angle: f32,
        center: P,
        source_pos: P,
        source_size: P,
    );
    /// Draw a zone of a decal, rotate it and scaled it
    fn draw_partial_rotated_decal_scaled<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: P,
        decal: &Decal,
        angle: f32,
        center: P,
        source_pos: P,
        source_size: P,
        scaled: P,
    );
}

impl DecalDraw for crate::Engine {
    fn draw_decal_scaled<P: Into<Vf2d> + Copy>(&mut self, pos: P, decal: &Decal, scale: P) {
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
            });
    }
    fn draw_decal<P: Into<Vf2d> + Copy>(&mut self, pos: P, decal: &Decal) {
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
            });
    }
    fn draw_partial_decal<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: P,
        decal: &Decal,
        source_pos: P,
        source_size: P,
    ) {
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
            });
    }
    fn draw_partial_decal_scaled<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: P,
        decal: &Decal,
        source_pos: P,
        source_size: P,
        scale: P,
    ) {
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
            });
    }

    fn draw_warped_decal<P: Into<Vf2d> + Copy>(&mut self, pos: [P; 4], decal: &Decal) {
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
        };
        const POINT_ONE: usize = 3;
        const POINT_TWO: usize = 0;
        const POINT_THREE: usize = 1;
        const POINT_FOUR: usize = 2;
        let rd = (pos[POINT_THREE].x - pos[POINT_ONE].x) * (pos[POINT_FOUR].y - pos[POINT_TWO].y)
            - (pos[POINT_FOUR].x - pos[POINT_TWO].x) * (pos[POINT_THREE].y - pos[POINT_ONE].y);
        if rd != 0.0 {
            let rd = 1.0 / rd;
            let rn = ((pos[POINT_FOUR].x - pos[POINT_TWO].x)
                * (pos[POINT_ONE].y - pos[POINT_TWO].y)
                - (pos[POINT_FOUR].y - pos[POINT_TWO].y) * (pos[POINT_ONE].x - pos[POINT_TWO].x))
                * rd;
            let sn = ((pos[POINT_THREE].x - pos[POINT_ONE].x)
                * (pos[POINT_ONE].y - pos[POINT_TWO].y)
                - (pos[POINT_THREE].y - pos[POINT_ONE].y) * (pos[POINT_ONE].x - pos[POINT_TWO].x))
                * rd;
            if !(rn < 0.0 || rn > 1.0 || sn < 0.0 || sn > 1.0) {
                center = pos[POINT_ONE] + (pos[POINT_THREE] - pos[POINT_ONE]) * rn;
                //println!("fdshuifdsqhiyfgqsyfgqsfgdqs");
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
    fn draw_warped_partial_decal<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: [P; 4],
        source_pos: P,
        source_size: P,
        decal: &Decal,
    ) {
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
        };
        const POINT_ONE: usize = 3;
        const POINT_TWO: usize = 0;
        const POINT_THREE: usize = 1;
        const POINT_FOUR: usize = 2;
        let rd = (pos[POINT_THREE].x - pos[POINT_ONE].x) * (pos[POINT_FOUR].y - pos[POINT_TWO].y)
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
                - (pos[POINT_FOUR].y - pos[POINT_TWO].y) * (pos[POINT_ONE].x - pos[POINT_TWO].x))
                * rd;
            let sn = ((pos[POINT_THREE].x - pos[POINT_ONE].x)
                * (pos[POINT_ONE].y - pos[POINT_TWO].y)
                - (pos[POINT_THREE].y - pos[POINT_ONE].y) * (pos[POINT_ONE].x - pos[POINT_TWO].x))
                * rd;
            if !(rn < 0.0 || rn > 1.0 || sn < 0.0 || sn > 1.0) {
                center = pos[POINT_ONE] + (pos[POINT_THREE] - pos[POINT_ONE]) * rn;
                //println!("fdshuifdsqhiyfgqsyfgqsfgdqs");
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

    fn draw_explicit_decal<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: [P; 4],
        uv: [P; 4],
        decal: &Decal,
    ) {
        let pos: [Vf2d; 4] = [pos[0].into(), pos[1].into(), pos[2].into(), pos[3].into()];
        let uv: [Vf2d; 4] = [uv[0].into(), uv[1].into(), uv[2].into(), uv[3].into()];
        let mut di = px_backend::decals::DecalInstances {
            id: decal.0.id(),
            pos: [(0.0, 0.0), (0.0, 0.0), (0.0, 0.0), (0.0, 0.0)],
            uv: [(0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (1.0, 0.0)],
            w: [1.0; 4],
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

    fn draw_rotated_decal<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: P,
        decal: &Decal,
        angle: f32,
        center: P,
    ) {
        let screen_size: Vf2d = (self.size.0 as f32, self.size.1 as f32).into();
        into!(pos, center);
        let mut pos_arr = [Vf2d { x: 0.0, y: 0.0 }; 4];
        pos_arr[0] = Vf2d {x:0.0,y:0.0} - center /* * scale*/;
        pos_arr[1] = Vf2d {x: 0.0, y:  decal.size().1 as f32} - center /* * scale*/;
        pos_arr[2] = Vf2d {x:decal.size().0 as f32, y:decal.size().1 as f32} - center/*  * scale*/;
        pos_arr[3] = Vf2d {x:decal.size().0 as f32, y: 0.0} - center /* * scale*/;
        let (c, s) = angle.sin_cos();
        for i in 0..4 {
            pos_arr[i] = normalize!(
                {
                    pos + Vf2d {
                        x: pos_arr[i].x * c + pos_arr[i].y * s,
                        y: pos_arr[i].x * s - pos_arr[i].y * c,
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
        });
    }
    fn draw_rotated_decal_scaled<P: Into<Vf2d> + Copy>(
        &mut self,
        pos: P,
        decal: &Decal,
        angle: f32,
        center: P,
        scale: P,
    ) {
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
        for i in 0..4 {
            pos_arr[i] = normalize!(
                {
                    pos + Vf2d {
                        x: pos_arr[i].x * c + pos_arr[i].y * s,
                        y: pos_arr[i].x * s - pos_arr[i].y * c,
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
        });
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
        for i in 0..4 {
            pos_arr[i] = normalize!(
                {
                    pos + Vf2d {
                        x: pos_arr[i].x * c + pos_arr[i].y * s,
                        y: pos_arr[i].x * s - pos_arr[i].y * c,
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
        });
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
        for i in 0..4 {
            pos_arr[i] = normalize!(
                {
                    pos + Vf2d {
                        x: pos_arr[i].x * c + pos_arr[i].y * s,
                        y: pos_arr[i].x * s - pos_arr[i].y * c,
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
        });
    }
}
