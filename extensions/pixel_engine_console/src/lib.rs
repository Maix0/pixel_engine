use pixel_engine::vector2::Vf2d;

extern crate log;

mod console_logger;

#[derive(Debug)]
pub struct DecalStorage {
    pub error: pixel_engine::decals::Decal,
    pub warn: pixel_engine::decals::Decal,
    pub info: pixel_engine::decals::Decal,
    pub debug: pixel_engine::decals::Decal,
    pub trace: pixel_engine::decals::Decal,
    pub background: pixel_engine::decals::Decal,
    pub separator: pixel_engine::decals::Decal,
    pub separator_colors: [pixel_engine::Color; 5],
}

impl DecalStorage {
    pub fn new(engine: &mut pixel_engine::Engine) -> Self {
        use pixel_engine::{Color, Sprite};
        static ERROR_SPRITE_DATA: &[u8] = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/Error-Logo.png"
        ));
        static WARN_SPRITE_DATA: &[u8] =
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/Warn-Logo.png"));
        static INFO_SPRITE_DATA: &[u8] =
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/Info-Logo.png"));
        static DEBUG_SPRITE_DATA: &[u8] = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/Debug-Logo.png"
        ));
        static TRACE_SPRITE_DATA: &[u8] = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/Trace-Logo.png"
        ));
        static BACKGROUND_SPRITE_DATA: &[u8] = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/Background.png"
        ));
        static SEPARATOR_SPRITE_DATA: &[u8] =
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/Separator.png"));
        let error_sprite = Sprite::load_image_bytes(ERROR_SPRITE_DATA).unwrap();
        let warn_sprite = Sprite::load_image_bytes(WARN_SPRITE_DATA).unwrap();
        let info_sprite = Sprite::load_image_bytes(INFO_SPRITE_DATA).unwrap();
        let debug_sprite = Sprite::load_image_bytes(DEBUG_SPRITE_DATA).unwrap();
        let trace_sprite = Sprite::load_image_bytes(TRACE_SPRITE_DATA).unwrap();
        //let background_sprite = Sprite::load_image_bytes(BACKGROUND_SPRITE_DATA).unwrap();
        let mut background_sprite = Sprite::new_with_color(
            500,
            500,
            Color {
                r: 255,
                g: 0,
                b: 0,
                a: 127,
            },
        );
        let separator_sprite = Sprite::load_image_bytes(SEPARATOR_SPRITE_DATA).unwrap();

        for x in 0..10 {
            for y in 0..10 {
                background_sprite.set_pixel(
                    50 + x,
                    50 + y,
                    Color {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 0,
                    },
                )
            }
        }

        let error = engine.create_decal(&error_sprite);
        let warn = engine.create_decal(&warn_sprite);
        let info = engine.create_decal(&info_sprite);
        let debug = engine.create_decal(&debug_sprite);
        let trace = engine.create_decal(&trace_sprite);
        let background = engine.create_decal(&background_sprite);
        let separator = engine.create_decal(&separator_sprite);
        Self {
            error,
            warn,
            info,
            debug,
            trace,
            background,
            separator,
            separator_colors: [
                Color::new(255, 0, 0),
                Color::new(255, 103, 0),
                Color::new(99, 155, 255),
                Color::new(118, 66, 138),
                Color::new(255, 255, 255),
            ],
        }
    }
}

pub trait ConsoleGame: pixel_engine::Game + Sized {
    fn create_console_game(
        engine: &mut pixel_engine::Engine,
    ) -> Result<(Self, PixelConsoleOptions), Box<dyn std::error::Error>>;

    fn receive_console_input(&mut self, _engine: &mut pixel_engine::Engine, _input: String) {}

    fn open_console(
        &mut self,
        exit_key: pixel_engine::inputs::Keycodes,
        background_processing: bool,
    ) {
        CONSOLE_OPEN_METADATA.with(|c| c.set(Some((exit_key, background_processing))));
    }

    fn close_console(&mut self, engine: &mut pixel_engine::Engine) {
        let input_str = engine.get_input_buffer();
        LAST_INPUT_STRING.with(|c| {
            let mut br = c.borrow_mut();
            br.clear();
            br.push_str(input_str);
        });
        engine.force_stop_input_mode();
        CONSOLE_OPEN_METADATA.with(|c| c.set(None));
    }

    fn is_console_opened(&self) -> bool {
        CONSOLE_OPEN_METADATA.with(|c| c.get().is_some())
    }
}

thread_local! {
    static CONSOLE_OPEN_METADATA: std::cell::Cell<Option<(pixel_engine::inputs::Keycodes, bool)>> = None.into();
    static LAST_INPUT_STRING: std::cell::RefCell<String> = String::new().into();
}

#[derive(Debug)]
pub struct GameWrapper<G: ConsoleGame> {
    char_scale: Vf2d,
    line_height: f32,
    current_line: usize,
    console_height: f32,
    decals: DecalStorage,
    logger: &'static console_logger::ConsoleLogger,
    inner: G,
}

impl<G: ConsoleGame> pixel_engine::Game for GameWrapper<G> {
    fn update(
        &mut self,
        engine: &mut pixel_engine::Engine,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let data = CONSOLE_OPEN_METADATA.with(|c| c.get());
        let mut toggled = data.is_some();
        LAST_INPUT_STRING.with(|c| {
            let mut br = c.borrow_mut();
            if toggled && !br.is_empty() {
                engine.set_input_buffer(br.as_str());
                br.clear();
            }
        });
        if toggled
            && engine
                .get_key(unsafe { data.as_ref().unwrap_unchecked().0 })
                .any()
        {
            self.close_console(engine);
            toggled = false;
        }

        return if toggled {
            engine
                .set_input_passthrough([unsafe { data.as_ref().unwrap_unchecked().0 }].into_iter());
            engine.start_input();
            let res = if unsafe { data.as_ref().unwrap_unchecked().1 } {
                self.inner.update(engine)
            } else {
                Ok(true)
            };
            self.render(engine);
            res
        } else {
            self.inner.update(engine)
        };
    }

    fn create(engine: &mut pixel_engine::Engine) -> Result<Self, Box<dyn std::error::Error>> {
        let (inner, options) = G::create_console_game(engine)?;

        Self::new_with_options(engine, options, inner)
    }

    fn receive_input(&mut self, engine: &mut pixel_engine::Engine, input: String) {
        if self.is_console_opened() {
            self.inner.receive_console_input(engine, input);
        } else {
            self.inner.receive_input(engine, input);
        }
    }
}

impl<G: ConsoleGame> std::ops::Deref for GameWrapper<G> {
    type Target = G;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<G: ConsoleGame> std::ops::DerefMut for GameWrapper<G> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[derive(Default)]
pub struct PixelConsoleOptions {
    /// Each decal is in a 1:1 aspect ratio
    pub decals: Option<DecalStorage>,
    /// The length of a single line in character (so *4 = in pixel)
    pub line_size: Option<usize>,
    pub console_height: Option<f32>,
    pub buffer_size: Option<usize>,
    pub logger_passthrough: Option<(&'static dyn log::Log, log::LevelFilter)>,
}

impl<G: ConsoleGame> GameWrapper<G> {
    fn new_with_options(
        game: &mut pixel_engine::Engine,
        PixelConsoleOptions {
            decals,
            line_size,
            buffer_size,
            logger_passthrough,
            console_height,
        }: PixelConsoleOptions,
        inner: G,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let decals = decals.unwrap_or_else(|| DecalStorage::new(game));
        let line_size =
            line_size.unwrap_or(((((game.size().x * game.scale()) - 3) / 8) / 2) as usize);
        let char_scale: Vf2d = (1.0, 1.0).into();
        console_logger::install(
            buffer_size.unwrap_or(256),
            logger_passthrough.unwrap_or((log::logger(), log::LevelFilter::Off)),
            line_size,
        )
        .expect("Unable to set logger");
        let console_height = console_height.unwrap_or((game.size().y * game.scale()) as f32 / 2.0);
        let logger = get_logger().expect("Error when getting console_logger");

        Ok(Self {
            decals,
            char_scale,
            current_line: 0,
            line_height: char_scale.y * 8.0 * game.scale() as f32 + 2.0,
            console_height,
            logger,
            inner,
        })
    }
    fn render(&self, engine: &mut pixel_engine::Engine) {
        use pixel_engine::decals::DecalText;
        use pixel_engine::traits::DecalDraw;
        let max_lines = (self.console_height / self.line_height).trunc() as usize - 1;
        let lines_lock = self.logger.inner_buffer.read().unwrap();
        let lines = lines_lock.iter().skip(self.current_line).take(max_lines);

        let bottom_right: pixel_engine::vector2::Vf2d = //(8.0, 8.0).into();
            engine.size().cast_f32();
        /*
        engine.draw_warped_decal(
            [
                (50.0, 50.0),
                (0.0, bottom_right.y),
                (bottom_right.x, bottom_right.y),
                (bottom_right.x, 0.0),
            ],
            &self.decals.background,
        );
        */
        engine.draw_decal((50.0, 50.0), &self.decals.background);

        for (index, line) in lines.rev().enumerate() {
            let line_topleft = Vf2d {
                x: 0.0,
                y: self.line_height * index as f32 + 1.0,
            };
            engine.draw_warped_decal(
                [
                    line_topleft,
                    line_topleft + (0.0, self.char_scale.y * 8.0).into(),
                    line_topleft + (self.char_scale * 8.0),
                    line_topleft + (self.char_scale.x * 8.0, 0.0).into(),
                ],
                match line.level {
                    log::Level::Error => &self.decals.error,
                    log::Level::Warn => &self.decals.warn,
                    log::Level::Info => &self.decals.info,
                    log::Level::Debug => &self.decals.debug,
                    log::Level::Trace => &self.decals.trace,
                },
            );
            engine.draw_warped_decal_tinted(
                [
                    line_topleft + (self.char_scale.x * 8.0 * 2.0, 0.0).into(),
                    line_topleft
                        + (0.0, self.char_scale.y * 8.0).into()
                        + (self.char_scale.x * 8.0 * 2.0, 0.0).into(),
                    line_topleft
                        + (self.char_scale * 8.0)
                        + (self.char_scale.x * 8.0 * 2.0, 0.0).into(),
                    line_topleft
                        + (self.char_scale.x * 8.0, 0.0).into()
                        + (self.char_scale.x * 8.0 * 2.0, 0.0).into(),
                ],
                &self.decals.separator,
                self.decals.separator_colors[match line.level {
                    log::Level::Error => 0,
                    log::Level::Warn => 1,
                    log::Level::Info => 2,
                    log::Level::Debug => 3,
                    log::Level::Trace => 4,
                }],
            );
            engine.draw_text_decal(
                line_topleft + (self.char_scale.x * 8.0 * 3.0, 0.0).into(),
                &line.message,
                self.char_scale,
                [255, 255, 255],
            );
        }

        engine.draw_text_decal(
            (0.0, bottom_right.y - self.char_scale.y * 8.0),
            ">",
            self.char_scale,
            pixel_engine::Color::YELLOW,
        );
        engine.draw_text_decal(
            (
                self.char_scale.x * 8.0,
                bottom_right.y - self.char_scale.y * 8.0,
            ),
            // SAFETY: This is safe since `draw_text_decal` doesn't interact with the input buffer
            // (which is where the str is located)
            // the transmute is to change the lifetime into something other than the engine's one;
            unsafe { std::mem::transmute::<&str, &'_ str>(engine.get_input_buffer()) },
            self.char_scale,
            [255, 255, 255],
        );
    }
}

fn get_logger() -> Option<&'static console_logger::ConsoleLogger> {
    console_logger::HAS_CONSOLE_LOGGER
        .load(std::sync::atomic::Ordering::SeqCst)
        .then(|| unsafe {
            &*(log::logger() as *const dyn log::Log as *const console_logger::ConsoleLogger)
        })
}
