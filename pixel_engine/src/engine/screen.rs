use px_draw::graphics::{Color, PixelMode, Sprite};
/// The Drawing layer for the engine
/// It is a glorified [`Sprite`]
#[derive(Debug, Clone)]
pub struct Screen {
    pub(crate) screen: Sprite,
    pub(crate) size: (u32, u32),
    pub(crate) textsheet: Sprite,
    pub(crate) updated: bool,
    pub(crate) pixel_mode: PixelMode,
    pub(crate) blend_factor: f32,
}

static TEXT_FONT: &str = "?Q`0001oOch0o01o@F40o0<AGD4090LAGD<090@A7ch0?00O7Q`0600>00000000O000000nOT0063Qo4d8>?7a14Gno94AA4gno94AaOT0>o3`oO400o7QN00000400Of80001oOg<7O7moBGT7O7lABET024@aBEd714AiOdl717a_=TH013Q>00000000720D000V?V5oB3Q_HdUoE7a9@DdDE4A9@DmoE4A;Hg]oM4Aj8S4D84@`00000000OaPT1000Oa`^13P1@AI[?g`1@A=[OdAoHgljA4Ao?WlBA7l1710007l100000000ObM6000oOfMV?3QoBDD`O7a0BDDH@5A0BDD<@5A0BGeVO5ao@CQR?5Po00000000Oc``000?Ogij70PO2D]??0Ph2DUM@7i`2DTg@7lh2GUj?0TO0C1870T?0000000070<4001o?P<7?1QoHg43O;`h@GT0@:@LB@d0>:@hN@L0@?aoN@<0O7ao0000?000OcH0001SOglLA7mg24TnK7ln24US>0PL24U140PnOgl0>7QgOcH0K71S0000A00000H00000@Dm1S007@DUSg00?OdTnH7YhOfTL<7Yh@Cl0700?@Ah0300700000000<008001QL00ZA41a@6HnI<1i@FHLM81M@@0LG81?O`0nC?Y7?`0ZA7Y300080000O`082000Oh0827mo6>Hn?Wmo?6HnMb11MP08@C11H`08@FP0@@0004@00000000000P00001Oab00003OcKP0006@6=PMgl<@440MglH@000000`@000001P00000000Ob@8@@00Ob@8@Ga13R@8Mga172@8?PAo3R@827QoOb@820@0O`0007`0000007P0O`000P08Od400g`<3V=P0G`673IP0`@3>1`00P@6O`P00g`<O`000GP800000000?P9PL020O`<`N3R0@E4HC7b0@ET<ATB0@@l6C4B0O`H3N7b0?P01L3R000000020";

impl Screen {
    fn create_text() -> Sprite {
        let mut sheet = Sprite::new(128, 48);
        let mut px = 0;
        let mut py = 0;
        for b in (0..1024).step_by(4) {
            let sym1 = TEXT_FONT.chars().nth(b).unwrap() as u32 - 48;
            let sym2 = TEXT_FONT.chars().nth(b + 1).unwrap() as u32 - 48;
            let sym3 = TEXT_FONT.chars().nth(b + 2).unwrap() as u32 - 48;
            let sym4 = TEXT_FONT.chars().nth(b + 3).unwrap() as u32 - 48;
            let r: u32 = sym1 << 18 | sym2 << 12 | sym3 << 6 | sym4;
            for i in 0..24 {
                let k = if (r & (1 << i)) != 0 { 255 } else { 0 };
                sheet.set_pixel(px, py, [k, k, k].into());
                py += 1;
                if py == 48 {
                    px += 1;
                    py = 0;
                }
            }
        }
        sheet
    }
    /// Create a new Screen with the given size
    pub(crate) fn new(size: (u32, u32)) -> Screen {
        if size.0 == 0 || size.1 == 0 {
            panic!("Size elements can't be equal to 0")
        }
        Screen {
            screen: Sprite::new(size.0 /* * size.2*/, size.1 /* * size.2*/),
            size,
            textsheet: Self::create_text(),
            updated: false,
            pixel_mode: PixelMode::Normal,
            blend_factor: 1.0,
        }
    }
    pub(crate) fn get_raw(&mut self) -> Box<[u8]> {
        self.screen.get_raw()
    }
}

impl px_draw::traits::ScreenTrait for Screen {
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
                let alpha: f32 = (col.a as f32 / 255.0f32) * self.get_blend_factor();
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
