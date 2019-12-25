use crate::handler;
use crate::keyboard;
use crate::keyboard::KeySet;
use crate::screen;
type GameLogic = &'static (dyn Fn(&mut Engine));

pub struct Engine {
    /* FRONTEND */
    pub title: String,
    pub size: (u32, u32, u32),

    /* TIME */
    pub elapsed: f64,
    timer: std::time::SystemTime,
    frame_count: u64,
    frame_timer: f64,

    /* BACKEND */
    pub main: GameLogic,
    pub screen: screen::ScreenHandle,
    //handler: GlHandler,
    handle: handler::GLHandle,
    k_pressed: std::collections::HashSet<keyboard::Key>,
    k_held: std::collections::HashSet<keyboard::Key>,
    k_released: std::collections::HashSet<keyboard::Key>,
}

impl Engine {
    pub fn new(title: String, size: (u32, u32, u32), func: GameLogic) -> Self {
        let (sender_handle, receiver_handler) = std::sync::mpsc::channel();
        let (sender_handler, receiver_handle) = std::sync::mpsc::channel();
        handler::GlHandler::spawn_thread(size, receiver_handler, sender_handler);
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
            screen: crate::screen::ScreenHandle::spawn_thread(size),
            handle: handler::GLHandle::new(sender_handle, receiver_handle),
            k_pressed: std::collections::HashSet::new(),
            k_held: std::collections::HashSet::new(),
            k_released: std::collections::HashSet::new(),
        }
    }
    pub fn run(&mut self) -> Result<(), String> {
        (self.main)(self);
        Ok(())
    }
    pub fn stop(&mut self) {
        self.handle.destroy();
        self.screen.destroy();
    }

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
                handler::GLEvents::Keyboard { inp } => {
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

        if Some(true) == self.screen.updated() {
            if let Some(img) = self.screen.get_image() {
                self.handle.update_frame(img);
            }
        }
        true
    }

    pub fn is_pressed(&self, keycode: keyboard::Keycodes) -> bool {
        self.k_pressed.has(keycode)
    }
    pub fn is_held(&self, keycode: keyboard::Keycodes) -> bool {
        self.k_held.has(keycode)
    }
    pub fn is_released(&self, keycode: keyboard::Keycodes) -> bool {
        self.k_released.has(keycode)
    }

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
