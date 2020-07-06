//use super::handler::GlHandler;
use super::inputs::{self, Input, KeySet, Mouse, MouseBtn, MouseWheel};
use super::screen::Screen;
use futures::executor::block_on;
use px_backend::winit::{
    self,
    event::{Event, WindowEvent},
};

/// A Wrapper around an Engine
#[derive(Debug)]
pub struct EngineWrapper(Option<Engine>);

impl EngineWrapper {
    /// Create the Engine and the Wrapper
    pub fn new(title: String, size: (u32, u32, u32)) -> Self {
        Self(Some(Engine::new(title, size)))
    }
    /// Get a Referance to the inner Engine
    pub fn get_inner(&mut self) -> &mut Engine {
        self.0.as_mut().unwrap()
    }
    /// The core of your program,
    ///
    /// Takes a function F that will be run every frame, It will do the event handling  and similar
    /// things between frames.
    pub fn run<F>(mut self, mut main_func: F) -> Self
    where
        F: (FnMut(&mut Engine) -> Result<bool, Box<dyn std::error::Error>>) + 'static,
    {
        let mut engine = self.0.unwrap();
        self.0 = None;
        let mut force_exit = false;
        let event_loop = engine.event_loop.unwrap();
        engine.event_loop = None;
        let mut redraw = true;
        let mut redraw_last_frame = false;
        event_loop.run(move |e, _, control_flow| {
            engine.elapsed = (std::time::SystemTime::now()
                .duration_since(engine.timer)
                .map_err(|err| err.to_string())
                .unwrap())
            .as_secs_f64();
            // End
            engine.timer = std::time::SystemTime::now();
            engine.frame_timer += engine.elapsed;
            engine.frame_count += 1;
            if engine.frame_timer > 1.0 {
                engine.frame_timer -= 1.0;
                engine
                    .window
                    .set_title(&format!("{} - {}fps", engine.title, engine.frame_count));
                engine.frame_count = 0;
            }
            if redraw_last_frame {
                for key in &engine.k_pressed {
                    engine.k_held.insert(*key);
                }
                engine.k_pressed.clear();
                engine.k_released.clear();
                for i in 0..3 {
                    if engine.mouse.buttons[i].released {
                        engine.mouse.buttons[i].released = false;
                    }
                    if engine.mouse.buttons[i].pressed {
                        engine.mouse.buttons[i].pressed = false;
                        engine.mouse.buttons[i].held = true;
                    }
                }
                engine.mouse.wheel = MouseWheel::None;
                redraw_last_frame = false;
            }
            match e {
                Event::WindowEvent {
                    event: e,
                    window_id,
                } if window_id == engine.window.id() => match e {
                    WindowEvent::KeyboardInput { input: inp, .. } => {
                        if let Some(k) = inp.virtual_keycode {
                            if inp.state == winit::event::ElementState::Released {
                                engine.k_pressed.remove(&inputs::Key::from(inp));
                                engine.k_held.remove(&inputs::Key::from(inp));
                                engine.k_released.insert(inputs::Key::from(inp));
                            } else if !engine.k_held.has(k) {
                                engine.k_pressed.insert(inputs::Key::from(inp));
                            }
                        }
                    }
                    WindowEvent::CloseRequested => {
                        force_exit = true;
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        let (x, y): (f64, f64) = position.into();
                        //events.push(Events::MouseMove(x, y));
                        engine.mouse.pos = (
                            (x / engine.size.2 as f64).floor() as u32,
                            (y / engine.size.2 as f64).floor() as u32,
                        );
                    }
                    WindowEvent::MouseWheel { delta, .. } => match delta {
                        winit::event::MouseScrollDelta::LineDelta(x, y) => {
                            engine.mouse.wheel = if x.abs() > y.abs() {
                                if x > 0.0 {
                                    MouseWheel::Right
                                } else if x < 0.0 {
                                    MouseWheel::Left
                                } else {
                                    MouseWheel::None
                                }
                            } else if y > 0.0 {
                                MouseWheel::Down
                            } else if y < 0.0 {
                                MouseWheel::Up
                            } else {
                                MouseWheel::None
                            };
                        }
                        winit::event::MouseScrollDelta::PixelDelta(lp) => {
                            let (x, y): (f64, f64) = lp.into();
                            engine.mouse.wheel = if x.abs() > y.abs() {
                                if x > 0.0 {
                                    MouseWheel::Right
                                } else if x < 0.0 {
                                    MouseWheel::Left
                                } else {
                                    MouseWheel::None
                                }
                            } else if y > 0.0 {
                                MouseWheel::Down
                            } else if y < 0.0 {
                                MouseWheel::Up
                            } else {
                                MouseWheel::None
                            };
                        }
                    },
                    WindowEvent::MouseInput { button, state, .. } => {
                        if std::mem::discriminant(&button)
                            != std::mem::discriminant(&winit::event::MouseButton::Other(0))
                        {
                            let btn = match button {
                                winit::event::MouseButton::Left => MouseBtn::Left,
                                winit::event::MouseButton::Right => MouseBtn::Right,
                                winit::event::MouseButton::Middle => MouseBtn::Middle,
                                winit::event::MouseButton::Other(_) => {
                                    unreachable!("MouseButton::Other()")
                                }
                            };
                            if state == winit::event::ElementState::Pressed {
                                engine.mouse.buttons[match btn {
                                    MouseBtn::Left => 0,
                                    MouseBtn::Right => 1,
                                    MouseBtn::Middle => 2,
                                }]
                                .pressed = true;
                            } else {
                                engine.mouse.buttons[match btn {
                                    MouseBtn::Left => 0,
                                    MouseBtn::Right => 1,
                                    MouseBtn::Middle => 2,
                                }]
                                .released = true;
                                engine.mouse.buttons[match btn {
                                    MouseBtn::Left => 0,
                                    MouseBtn::Right => 1,
                                    MouseBtn::Middle => 2,
                                }]
                                .held = false;
                            }
                        }
                    }
                    _ => {}
                },
                Event::RedrawRequested(_) => {
                    redraw = true;
                }
                Event::MainEventsCleared => {
                    engine.window.request_redraw();
                }
                _ => {}
            }
            if redraw {
                let r = (main_func)(&mut engine);
                if r.is_err() || r.as_ref().ok() == Some(&false) || force_exit {
                    if let Err(e) = r {
                        if cfg!(debug_assertions) {
                            println!("Game Stopped:\n{:?}", e);
                        } else {
                            println!("Game Stopped:\n{}", e);
                        }
                    }
                    *control_flow = winit::event_loop::ControlFlow::Exit;
                }
                engine
                    .handler
                    .get_screen_slice()
                    .clone_from_slice(&engine.screen.get_raw());
                engine.handler.render();
                redraw = false;
                redraw_last_frame = true;
            }
        });
        #[allow(unreachable_code)]
        {
            engine.event_loop = Some(event_loop);
            self.0 = Some(engine);
            self
        }
    }
}

/**
 *  Bone of the Engine, join everything;
 **/
pub struct Engine {
    /* FRONTEND */
    /// Main title of the window, Window's full title will be "Title - fps"
    pub title: String,
    /// Size of the window, with (x-size,y-size,pixel-size)
    pub size: (u32, u32, u32),

    /* TIME */
    /// Time between current frame and last frame, usefull for movement's calculations
    pub elapsed: f64,
    timer: std::time::SystemTime,
    frame_count: u64,
    frame_timer: f64,

    /* BACKEND */
    /// Game's screen manager, let you draw on the screen
    pub screen: Screen,
    handler: px_backend::Context,
    k_pressed: std::collections::HashSet<inputs::Key>,
    k_held: std::collections::HashSet<inputs::Key>,
    k_released: std::collections::HashSet<inputs::Key>,
    mouse: Mouse,
    event_loop: Option<winit::event_loop::EventLoop<()>>,
    window: winit::window::Window,
}
impl std::fmt::Debug for Engine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Engine")
            .field("title", &self.title)
            .field("size", &self.size)
            .field("elapsed", &self.elapsed)
            .field("screen", &self.screen)
            .finish()
    }
}

impl Engine {
    /// Create a new [Engine]
    fn new(title: String, size: (u32, u32, u32)) -> Self {
        let event_loop = winit::event_loop::EventLoop::new();
        let window = winit::window::WindowBuilder::new()
            .with_inner_size(winit::dpi::PhysicalSize::new(
                (size.0 * size.2) as f32,
                (size.1 * size.2) as f32,
            ))
            .with_title(&title)
            .with_resizable(false)
            .build(&event_loop)
            .expect("Error when constructing window");

        Engine {
            /* FRONTEND */
            size,
            title,

            /* TIME */
            timer: std::time::SystemTime::now(),
            frame_count: 0u64,
            frame_timer: 0f64,
            elapsed: 0f64,
            /* BACKEND */
            handler: block_on(px_backend::Context::new(&window, size)),
            screen: Screen::new((size.0, size.1)),
            k_pressed: std::collections::HashSet::new(),
            k_held: std::collections::HashSet::new(),
            k_released: std::collections::HashSet::new(),
            mouse: Mouse::new(),
            window,
            event_loop: Some(event_loop),
        }
    }
    /// Get The status of a key
    #[inline]
    pub fn get_key(&self, keycode: inputs::Keycodes) -> Input {
        Input::new(
            self.k_pressed.has(keycode),
            self.k_held.has(keycode),
            self.k_released.has(keycode),
        )
    }
    /// Get the status of a Mouse Button
    pub fn get_mouse_btn(&self, btn: MouseBtn) -> Input {
        self.mouse.buttons[match btn {
            MouseBtn::Left => 0,
            MouseBtn::Right => 1,
            MouseBtn::Middle => 2,
        }]
    }

    /// Get the mouse location (in pixel) on the screen
    /// Will be defaulted to (0,0) at the start of the program
    pub fn get_mouse_location(&self) -> (u32, u32) {
        self.mouse.pos
    }
    /// Get the scroll wheel direction (If Any) during the frame
    pub fn get_mouse_wheel(&self) -> MouseWheel {
        self.mouse.wheel
    }
    /// Get all Keys pressed during the last frame
    pub fn get_pressed(&self) -> std::collections::HashSet<inputs::Keycodes> {
        self.k_pressed.clone().iter().map(|k| k.key).collect()
    }
}
