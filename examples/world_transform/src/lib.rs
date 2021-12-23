extern crate pixel_engine as px;
use px::vector2::*;
pub struct Transform {
    scale: Vf2d,
    offset: Vf2d,
    start_pan: Vf2d,
    world_top_left: Vf2d,
    world_bottom_right: Vf2d,
}

impl Transform {
    pub fn new(game: &px::Engine, scale: Vf2d) -> Self {
        Self {
            scale,
            offset: Vu2d::from(game.size()).cast_f32() / -2.0,
            start_pan: Vf2d { x: 1.0, y: 1.0 },
            world_top_left: (0.0, 0.0).into(),
            world_bottom_right: Vu2d::from(game.size()).cast_f32(),
        }
    }

    pub fn world_to_screen(&self, world: Vf2d) -> Vi2d {
        ((world - self.offset) * self.scale).cast_i32()
    }
    pub fn screen_to_world(&self, screen: Vi2d) -> Vf2d {
        screen.cast_f32() / self.scale + self.offset
    }

    pub fn handle_pan<F1, F2>(&mut self, game: &px::Engine, func_start: F1, func_continue: F2)
    where
        F1: FnOnce(&px::Engine) -> bool,
        F2: FnOnce(&px::Engine) -> bool,
    {
        let (mouse_x, mouse_y) = game.get_mouse_location();
        let mouse: Vf2d = (mouse_x as f32, mouse_y as f32).into();
        if func_start(game) {
            self.start_pan = mouse;
        }

        if func_continue(game) {
            self.offset -= (mouse - self.start_pan) / self.scale;
            self.start_pan = mouse;
        }

        self.world_top_left = self.screen_to_world((0, 0).into());
        self.world_bottom_right = self.screen_to_world(Vu2d::from(game.size()).cast_i32());
    }

    pub fn handle_zoom<F1, F2>(&mut self, game: &px::Engine, func_plus: F1, func_minus: F2)
    where
        F1: FnOnce(&px::Engine) -> bool,
        F2: FnOnce(&px::Engine) -> bool,
    {
        let (mouse_x, mouse_y) = game.get_mouse_location();
        let mouse: Vf2d = (mouse_x as f32, mouse_y as f32).into();

        let mouse_before = self.screen_to_world(mouse.cast_i32());

        if func_plus(game) {
            self.scale *= 1.01;
        }
        if func_minus(game) {
            self.scale *= 0.99;
        }

        let mouse_after = self.screen_to_world(mouse.cast_i32());
        self.offset += mouse_before - mouse_after;

        self.world_top_left = self.screen_to_world((0, 0).into());
        self.world_bottom_right = self.screen_to_world(Vu2d::from(game.size()).cast_i32());
    }

    pub fn get_mouse_location(&self, game: &px::Engine) -> Vf2d {
        let (mouse_x, mouse_y) = game.get_mouse_location();
        let mouse: Vi2d = (mouse_x as i32, mouse_y as i32).into();
        let mouse_after = self.screen_to_world(mouse);
        mouse_after
    }

    pub fn is_visible(&self, point: Vf2d) -> bool {
        point.y >= self.world_top_left.y
            && point.y <= self.world_bottom_right.y
            && point.x >= self.world_top_left.x
            && point.x <= self.world_bottom_right.x
    }
    pub fn is_visible_y(&self, y: f32) -> bool {
        y >= self.world_top_left.y && y <= self.world_bottom_right.y
    }

    pub fn is_visible_x(&self, x: f32) -> bool {
        x >= self.world_top_left.x && x <= self.world_bottom_right.x
    }

    /// Get a reference to the transform's world bottom right.
    pub fn world_bottom_right(&self) -> &Vf2d {
        &self.world_bottom_right
    }

    /// Get a reference to the transform's world top left.
    pub fn world_top_left(&self) -> &Vf2d {
        &self.world_top_left
    }

    /// Get a reference to the transform's start pan.
    pub fn start_pan(&self) -> &Vf2d {
        &self.start_pan
    }

    /// Get a reference to the transform's scale.
    pub fn scale(&self) -> &Vf2d {
        &self.scale
    }

    /// Get a reference to the transform's offset.
    pub fn offset(&self) -> &Vf2d {
        &self.offset
    }

    pub fn round_corner(&mut self) {
        self.world_top_left.x = self.world_top_left.x.floor();
        self.world_top_left.y = self.world_top_left.y.floor();
        self.world_bottom_right.x = self.world_bottom_right.x.ceil();
        self.world_bottom_right.y = self.world_bottom_right.y.ceil();
    }
}
