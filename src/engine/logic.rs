use super::handler::GlHandler;
use super::inputs::{self, Input, KeySet, Mouse, MouseBtn, MouseWheel};
use super::screen::Screen;
use parking_lot::Mutex;
use std::sync::Arc;

// Just used for the blocking of the rendering (no frame jump)
pub(crate) struct RenderBarrier;

#[derive(Debug, Clone, Copy)]
pub(crate) enum Events {
    /// A keyboard input
    Keyboard {
        /// The input
        inp: glutin::KeyboardInput,
    },
    Close,
    MouseMove(f64, f64),
    MouseWheel(MouseWheel),
    /// The bool indicate the type of event
    /// true => pressed
    /// false => released
    MouseClick(MouseBtn, bool),
}

/**
 *  Bone of the Engine, join everything;
 *  
 *  ## Working window:
 *  ```
 *  use pixel_engine_gl as engine;
 *      let mut game = engine::Engine::new(String::from("A window title"), (10,10,10),&game_logic);
 *      game.run();
 *  }
 *  fn game_logic(game:&mut engine::Engine) {
 *      # return; // This is to avoid the loop and everything during tests
 *      // Code run before everything, only once
 *      while game.new_frame() {
 *          // Your game code, run every frame
 *      }
 *      // Code run after everything
 *  ```
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
    screen_mutex: Arc<Mutex<Screen>>,
    handler: GlHandler,
    k_pressed: std::collections::HashSet<inputs::Key>,
    k_held: std::collections::HashSet<inputs::Key>,
    k_released: std::collections::HashSet<inputs::Key>,
    mouse: Mouse,
    blocking: std::sync::mpsc::SyncSender<RenderBarrier>,
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
    pub fn new(title: String, size: (u32, u32, u32)) -> Self {
        let (blocking, unblocking) = std::sync::mpsc::sync_channel(0);
        let screen_mutex = Arc::new(Mutex::new(Screen::new((size.0, size.1))));
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
            screen: Screen::new((size.0, size.1)),
            handler: GlHandler::new(size, unblocking, screen_mutex.clone()),
            k_pressed: std::collections::HashSet::new(),
            k_held: std::collections::HashSet::new(),
            k_released: std::collections::HashSet::new(),
            screen_mutex,
            blocking,
            mouse: Mouse::new(),
        }
    }
    fn events(&mut self) -> Vec<Events> {
        use glutin::{Event, WindowEvent};
        let mut events = Vec::new();
        self.handler.event_loop.poll_events(|e| {
            if let Event::WindowEvent { event: e, .. } = e {
                match e {
                    WindowEvent::KeyboardInput { input: inp, .. } => {
                        events.push(Events::Keyboard { inp });
                    }
                    WindowEvent::CloseRequested => {
                        events.push(Events::Close);
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        let (x, y): (f64, f64) = position.into();
                        events.push(Events::MouseMove(x, y));
                    }
                    WindowEvent::MouseWheel { delta, .. } => match delta {
                        glutin::MouseScrollDelta::LineDelta(x, y) => {
                            events.push(Events::MouseWheel(if x.abs() > y.abs() {
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
                            }));
                        }
                        glutin::MouseScrollDelta::PixelDelta(lp) => {
                            let (x, y): (f64, f64) = lp.into();
                            events.push(Events::MouseWheel(if x.abs() > y.abs() {
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
                            }));
                        }
                    },
                    WindowEvent::MouseInput { button, state, .. } => {
                        if std::mem::discriminant(&button)
                            != std::mem::discriminant(&glutin::MouseButton::Other(0))
                        {
                            events.push(Events::MouseClick(
                                match button {
                                    glutin::MouseButton::Left => MouseBtn::Left,
                                    glutin::MouseButton::Right => MouseBtn::Right,
                                    glutin::MouseButton::Middle => MouseBtn::Middle,
                                    glutin::MouseButton::Other(_) => {
                                        unreachable!("MouseButton::Other()")
                                    }
                                },
                                state == glutin::ElementState::Pressed,
                            ));
                        }
                    }
                    _ => {}
                }
            }
        });
        events
    }
    /// Run the engine with given function;
    pub fn run(
        &mut self,
        mut main_func: impl FnMut(&mut Engine) -> Result<bool, Box<dyn std::error::Error>>,
    ) {
        let mut force_exit = false;
        'mainloop: loop {
            self.elapsed = (std::time::SystemTime::now()
                .duration_since(self.timer)
                .map_err(|err| err.to_string())
                .unwrap())
            .as_secs_f64();
            // End
            self.timer = std::time::SystemTime::now();
            self.frame_timer += self.elapsed;
            self.frame_count += 1;
            if self.frame_timer > 1.0 {
                self.frame_timer -= 1.0;
                self.handler
                    .update_title(format!("{} - {}fps", self.title, self.frame_count));
                self.frame_count = 0;
            }
            for key in &self.k_pressed {
                self.k_held.insert(*key);
            }
            self.k_pressed.clear();
            self.k_released.clear();
            for i in 0..3 {
                if self.mouse.buttons[i].released {
                    self.mouse.buttons[i].released = false;
                }
                if self.mouse.buttons[i].pressed {
                    self.mouse.buttons[i].pressed = false;
                    self.mouse.buttons[i].held = true;
                }
            }

            self.mouse.wheel = MouseWheel::None;
            for event in self.events() {
                match event {
                    Events::Keyboard { inp } => {
                        if let Some(k) = inp.virtual_keycode {
                            if inp.state == glutin::ElementState::Released {
                                self.k_pressed.remove(&(inputs::Key::from(inp)));
                                self.k_held.remove(&(inputs::Key::from(inp)));
                                self.k_released.insert(inputs::Key::from(inp));
                            } else if !self.k_held.has(k) {
                                self.k_pressed.insert(inputs::Key::from(inp));
                            }
                        }
                    }
                    Events::Close => {
                        force_exit = true;
                    }
                    Events::MouseClick(btn, pressed) => {
                        if pressed {
                            self.mouse.buttons[match btn {
                                MouseBtn::Left => 0,
                                MouseBtn::Right => 1,
                                MouseBtn::Middle => 2,
                            }]
                            .pressed = true;
                        } else {
                            self.mouse.buttons[match btn {
                                MouseBtn::Left => 0,
                                MouseBtn::Right => 1,
                                MouseBtn::Middle => 2,
                            }]
                            .released = true;
                            self.mouse.buttons[match btn {
                                MouseBtn::Left => 0,
                                MouseBtn::Right => 1,
                                MouseBtn::Middle => 2,
                            }]
                            .held = false;
                        }
                    }
                    Events::MouseMove(x, y) => {
                        self.mouse.pos = (
                            (x / self.size.2 as f64).floor() as u32,
                            (y / self.size.2 as f64).floor() as u32,
                        );
                    }
                    Events::MouseWheel(dir) => {
                        self.mouse.wheel = dir;
                    }
                }
            }
            let r = (main_func)(self);
            if r.is_err() || r.as_ref().ok() == Some(&false) || force_exit {
                if let Err(e) = r {
                    if cfg!(debug_assertions) {
                        println!("Game Stopped:\n{:?}", e);
                    } else {
                        println!("Game Stopped:\n{}", e);
                    }
                }
                break 'mainloop;
            }
            self.update_frame();
        }
    }
    fn update_frame(&mut self) {
        let mut lock = self.screen_mutex.lock();
        lock.clone_from(&self.screen);
        std::mem::drop(lock);
        self.blocking.send(RenderBarrier).unwrap();
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

    pub fn get_mouse_btn(&self, btn: MouseBtn) -> Input {
        self.mouse.buttons[match btn {
            MouseBtn::Left => 0,
            MouseBtn::Right => 1,
            MouseBtn::Middle => 2,
        }]
    }
    pub fn get_mouse_location(&self) -> (u32, u32) {
        self.mouse.pos
    }
    pub fn get_mouse_wheel(&self) -> MouseWheel {
        self.mouse.wheel
    }
}
