use crate::graphics;
#[rustfmt::skip]
#[derive(Debug)]
enum Commands {
    /* Draw */
    Draw        {x: u32, y: u32, col: graphics::Color},
    DrawLine    {p1: (u32, u32), p2: (u32, u32),col: graphics::Color},
    DrawCircle  {x: u32, y: u32, r:u32, col: graphics::Color},
    DrawText    {x: u32, y: u32, scale: u32, col: graphics::Color, text: String},
    DrawRect    {x: u32, y: u32, w: u32, h: u32, col: graphics::Color},    
    /* Fill */
    FillCircle  {x: u32, y: u32, r:u32, col: graphics::Color},
    /* Other */
    Clear       {col:graphics::Color},
}

pub struct Screen {
    screen: graphics::Sprite,
    pub size: (u32, u32, u32),
    pub updated: bool,
    pub textsheet: graphics::Sprite,
    sender: std::sync::mpsc::Sender<Commands>,
    receiver: std::sync::mpsc::Receiver<Commands>,
}
impl Screen {
    // DRAW
    pub fn draw(&mut self, x: u32, y: u32, col: graphics::Color) {
        self.sender.send(Commands::Draw { x, y, col }).unwrap();
    }
    pub fn draw_line(&mut self, p1: (u32, u32), p2: (u32, u32), col: graphics::Color) {
        self.sender
            .send(Commands::DrawLine { p1, p2, col })
            .unwrap();
    }
    pub fn draw_circle(&mut self, x: u32, y: u32, r: u32, col: graphics::Color) {
        self.sender
            .send(Commands::DrawCircle { x, y, r, col })
            .unwrap();
    }
    pub fn draw_text(&mut self, x: u32, y: u32, scale: u32, col: graphics::Color, text: String) {
        self.sender
            .send(Commands::DrawText {
                x,
                y,
                scale,
                col,
                text,
            })
            .unwrap();
    }
    pub fn draw_rect(&mut self, x: u32, y: u32, w: u32, h: u32, col: graphics::Color) {
        self.sender
            .send(Commands::DrawRect { x, y, w, h, col })
            .unwrap();
    }

    // FILL

    pub fn fill_circle(&mut self, x: u32, y: u32, r: u32, col: graphics::Color) {
        self.sender
            .send(Commands::FillCircle { x, y, r, col })
            .unwrap();
    }

    // OTHER

    pub fn clear(&mut self, col: graphics::Color) {
        self.sender.send(Commands::Clear { col }).unwrap();
    }
}
impl Screen {
    pub fn launch(&mut self) {
        while let Ok(r) = self.receiver.recv() {
            use Commands::*;
            match r {
                /* Draw */
                Draw { x, y, col } => self.r_draw(x, y, col),
                DrawRect { x, y, w, h, col } => self.r_draw_rect(x, y, w, h, col),
                DrawCircle { x, y, r, col } => self.r_draw_circle(x, y, r, col),
                DrawLine { p1, p2, col } => self.r_draw_line(p1, p2, col),
                DrawText {
                    x,
                    y,
                    scale,
                    col,
                    text,
                } => self.r_draw_text(x, y, scale, col, text),
                /* Fill */
                FillCircle { x, y, r, col } => self.r_fill_circle(x, y, r, col),
                /* Other */
                Clear { col } => self.r_clear(col),
                //_ => panic!("{:?} isn't implemented in the threads receiver!!!!", r),
                //_ => println!("{:?}", r),
            };
        }
    }
    pub fn get_image(&self) -> image::RgbaImage {
        self.screen.get_raw()
    }

    pub fn screenshot<'a>(&self, filename: &'a std::path::Path) {
        self.screen.get_raw().save(filename).unwrap();
    }
    pub fn get_texture<F, R>(&self, factory: &mut F) -> gfx::handle::ShaderResourceView<R, [f32; 4]>
    where
        F: gfx::Factory<R>,
        R: gfx::Resources,
    {
        let img = self.get_image();
        let (width, height) = img.dimensions();
        let kind =
            gfx::texture::Kind::D2(width as u16, height as u16, gfx::texture::AaMode::Single);
        let (_, view) = factory
            .create_texture_immutable_u8::<crate::handler::ColorFormat>(
                kind,
                gfx::texture::Mipmap::Provided,
                &[&img],
            )
            .unwrap();
        view
    }
    fn create_text() -> graphics::Sprite {
        let mut sheet = graphics::Sprite::new(128, 48);
        let mut data = String::new();
        data += "?Q`0001oOch0o01o@F40o0<AGD4090LAGD<090@A7ch0?00O7Q`0600>00000000";
        data += "O000000nOT0063Qo4d8>?7a14Gno94AA4gno94AaOT0>o3`oO400o7QN00000400";
        data += "Of80001oOg<7O7moBGT7O7lABET024@aBEd714AiOdl717a_=TH013Q>00000000";
        data += "720D000V?V5oB3Q_HdUoE7a9@DdDE4A9@DmoE4A;Hg]oM4Aj8S4D84@`00000000";
        data += "OaPT1000Oa`^13P1@AI[?g`1@A=[OdAoHgljA4Ao?WlBA7l1710007l100000000";
        data += "ObM6000oOfMV?3QoBDD`O7a0BDDH@5A0BDD<@5A0BGeVO5ao@CQR?5Po00000000";
        data += "Oc``000?Ogij70PO2D]??0Ph2DUM@7i`2DTg@7lh2GUj?0TO0C1870T?00000000";
        data += "70<4001o?P<7?1QoHg43O;`h@GT0@:@LB@d0>:@hN@L0@?aoN@<0O7ao0000?000";
        data += "OcH0001SOglLA7mg24TnK7ln24US>0PL24U140PnOgl0>7QgOcH0K71S0000A000";
        data += "00H00000@Dm1S007@DUSg00?OdTnH7YhOfTL<7Yh@Cl0700?@Ah0300700000000";
        data += "<008001QL00ZA41a@6HnI<1i@FHLM81M@@0LG81?O`0nC?Y7?`0ZA7Y300080000";
        data += "O`082000Oh0827mo6>Hn?Wmo?6HnMb11MP08@C11H`08@FP0@@0004@000000000";
        data += "00P00001Oab00003OcKP0006@6=PMgl<@440MglH@000000`@000001P00000000";
        data += "Ob@8@@00Ob@8@Ga13R@8Mga172@8?PAo3R@827QoOb@820@0O`0007`0000007P0";
        data += "O`000P08Od400g`<3V=P0G`673IP0`@3>1`00P@6O`P00g`<O`000GP800000000";
        data += "?P9PL020O`<`N3R0@E4HC7b0@ET<ATB0@@l6C4B0O`H3N7b0?P01L3R000000020";
        let mut px = 0;
        let mut py = 0;
        for b in (0..1024).step_by(4) {
            let sym1 = data.chars().nth(b + 0).unwrap() as u32 - 48;
            let sym2 = data.chars().nth(b + 1).unwrap() as u32 - 48;
            let sym3 = data.chars().nth(b + 2).unwrap() as u32 - 48;
            let sym4 = data.chars().nth(b + 3).unwrap() as u32 - 48;
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
}

impl Screen {
    pub fn new(size: (u32, u32, u32)) -> Screen {
        if size.0 <= 0 || size.1 <= 0 || size.2 <= 0 {
            panic!("Size elements can't be equal to 0")
        }
        let (sender, receiver) = std::sync::mpsc::channel();
        Screen {
            screen: graphics::Sprite::new(size.0 * size.2, size.1 * size.2),
            size,
            updated: false,
            textsheet: Self::create_text(),
            sender,
            receiver,
        }
    }
    fn r_draw_rect(&mut self, x: u32, y: u32, w: u32, h: u32, col: graphics::Color) {
        self.draw_line((x, y), (x + w, y), col);
        self.draw_line((x + w, y), (x + w, y + h), col);
        self.draw_line((x + w, y + h), (x, y + h), col);
        self.draw_line((x, y + h), (x, y), col);
    }
    fn r_draw_circle(&mut self, x: u32, y: u32, r: u32, col: graphics::Color) {
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

    fn r_fill_circle(&mut self, x: u32, y: u32, r: u32, col: graphics::Color) {
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
                ((x - x0) as u32, (y - y0) as u32),
                ((x + x0) as u32, (y - y0) as u32),
                col,
            );
            self.draw_line(
                ((x - y0) as u32, (y - x0) as u32),
                ((x + y0) as u32, (y - x0) as u32),
                col,
            );
            self.draw_line(
                ((x - x0) as u32, (y + y0) as u32),
                ((x + x0) as u32, (y + y0) as u32),
                col,
            );
            self.draw_line(
                ((x - y0) as u32, (y + x0) as u32),
                ((x + y0) as u32, (y + x0) as u32),
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

    fn r_draw(&mut self, x: u32, y: u32, col: graphics::Color) {
        if x >= self.size.0 || y >= self.size.1 {
            return;
        }
        self.updated = true;
        if self.size.2 == 1 {
            self.screen.set_pixel(x, y, col);
        } else {
            for i in 0..(self.size.2) {
                for j in 0..(self.size.2) {
                    self.screen
                        .set_pixel(x * self.size.2 + i, y * self.size.2 + j, col);
                }
            }
        }
    }
    fn r_draw_text(&mut self, x: u32, y: u32, scale: u32, col: graphics::Color, text: String) {
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
                            if self.textsheet.get_pixel(i + ox * 8, j + oy * 8).unwrap().r > 0 {
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
                            if self.textsheet.get_pixel(i + ox * 8, j + oy * 8).unwrap().r > 0 {
                                self.draw(x + sx + i, y + sy + j, col)
                            }
                        }
                    }
                }
            }
            sx += 8 * scale;
        }
    }
    fn r_draw_line(&mut self, p1: (u32, u32), p2: (u32, u32), col: graphics::Color) {
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
    fn r_clear(&mut self, col: graphics::Color) {
        self.screen = graphics::Sprite::new_with_color(
            self.size.0 * self.size.2,
            self.size.1 * self.size.2,
            col,
        );
    }
}
