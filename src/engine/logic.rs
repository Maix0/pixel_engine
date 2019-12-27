use crate::keyboard;
use crate::keyboard::KeySet;
use crate::ScreenHandle;
use crate::{GLEvents, GLHandle};
type GameLogic = &'static (dyn Fn(&mut Engine));

/**
 *  Bone of the Engine, join everything;
 *  
 *  ## Working window:
 *  ```
 *  use pixel_engin_gl as engine;
 *  fn main() {
 *      let game = engine::Engine::new(String::new("A window title"), (10,10,10),&game_logic);
 *      game.start();
 *      game.destroy();
 *  }
 *  fn game_logic(game:&mut engine::Engine) {
 *      // Code run before everything, only once
 *      while game.new_frame() {
 *          // Your game code, run every frame
 *      }
 *      // Code run after everything
 *  }
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
    /// Game's core, defining what the window will do
    pub main: GameLogic,
    /// Game's screen manager, let you draw on the screen
    pub screen: ScreenHandle,
    //handler: GlHandler,
    handle: GLHandle,
    k_pressed: std::collections::HashSet<keyboard::Key>,
    k_held: std::collections::HashSet<keyboard::Key>,
    k_released: std::collections::HashSet<keyboard::Key>,
}
impl std::fmt::Debug for Engine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Engine")
            .field("title", &self.title)
            .field("size", &self.size)
            .field("elapsed", &self.elapsed)
            .field("main", &"main function")
            .field("screen", &self.screen)
            .finish()
    }
}

impl Engine {
    /// Create a new [Engine]
    pub fn new(title: String, size: (u32, u32, u32), func: GameLogic) -> Self {
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
            main: func,
            screen: ScreenHandle::spawn_thread(size),
            handle: GLHandle::new(size),
            k_pressed: std::collections::HashSet::new(),
            k_held: std::collections::HashSet::new(),
            k_released: std::collections::HashSet::new(),
        }
    }
    /// Run the engine;
    pub fn run(&mut self) {
        (self.main)(self);
    }
    /// Destroy everything tied to the engine
    pub fn stop(&mut self) {
        self.handle.destroy();
        self.screen.destroy();
    }
    /// Do all sort of things needed to update the engine. Will return a [bool] if the game
    /// needs to be shut down
    pub fn new_frame(&mut self) -> bool {
        /* FRAME STUFF */
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
            self.handle
                .update_title(format!("{} - {}fps", self.title, self.frame_count));
            self.frame_count = 0;
        }

        self.handle.request_events();
        self.k_held = self.k_pressed.drain().collect();
        self.k_released.drain();
        while let Ok(event) = self
            .handle
            .receiver
            .recv_timeout(std::time::Duration::from_millis(10))
        {
            match event {
                GLEvents::Keyboard { inp } => {
                    if inp.state == glutin::ElementState::Released {
                        self.k_pressed
                            .remove(&(keyboard::Key::from(inp.clone())).clone());
                        self.k_held
                            .remove(&(keyboard::Key::from(inp.clone())).clone());
                        self.k_released
                            .insert((keyboard::Key::from(inp.clone())).clone());
                    } else {
                        if !self.k_held.has(inp.virtual_keycode.unwrap()) {
                            self.k_pressed
                                .insert((keyboard::Key::from(inp.clone())).clone());
                        }
                    }
                }
            }
            println!("{:?}", event);
        }

        /* FRAME UPDATING */

        if self.screen.updated {
            if let Some(img) = self.screen.get_image() {
                self.handle.update_frame(img);
            }
        }
        true
    }
    /// Know if key is pressed
    pub fn is_pressed(&self, keycode: keyboard::Keycodes) -> bool {
        self.k_pressed.has(keycode)
    }
    /// Know if key is held
    pub fn is_held(&self, keycode: keyboard::Keycodes) -> bool {
        self.k_held.has(keycode)
    }
    /// Know if key is released
    pub fn is_released(&self, keycode: keyboard::Keycodes) -> bool {
        self.k_released.has(keycode)
    }
    /// Know if a key is pressed or held
    pub fn get_key(&self, keycode: keyboard::Keycodes) -> Option<&keyboard::Key> {
        if self.is_pressed(keycode) {
            for k in &self.k_pressed {
                if k.key == keycode {
                    return Some(k);
                }
            }
        } else if self.is_held(keycode) {
            for k in &self.k_held {
                if k.key == keycode {
                    return Some(k);
                }
            }
        }
        None
    }
}
