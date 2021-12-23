use std::collections::HashMap;
extern crate pixel_engine as px;
extern crate world_transform;
use px::{
    traits::*,
    vector2::{Vf2d, Vu2d},
    Color,
};

struct FloatRange {
    start: f32,
    end: f32,
    step: f32,
    current: f32,
}

impl FloatRange {
    pub fn new(start: f32, end: f32, step: f32) -> FloatRange {
        //assert that start && end && step != NaN && start < end && step > 0
        assert!(start < end);
        assert!(start.is_finite());
        assert!(end.is_finite());
        assert!(step.is_finite());
        assert!(step > 0f32);
        FloatRange {
            start,
            end,
            step,
            current: start,
        }
    }
}

type ShapeID = u16;
struct ShapeIDGen {
    current: u16,
}

impl ShapeIDGen {
    fn new() -> Self {
        Self { current: 0 }
    }
    fn get_one(&mut self) -> ShapeID {
        self.current += 1;
        self.current
    }
}

impl Iterator for FloatRange {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let ret = Some(self.current);
            self.current += self.step;
            ret
        } else {
            None
        }
    }
}

pub trait Shape: std::fmt::Debug {
    fn draw_youself(&self, game: &mut px::Engine, transform: &world_transform::Transform);
    fn draw_nodes(&self, game: &mut px::Engine, transform: &world_transform::Transform);
    fn max_node(&self) -> u8;
    fn set_current_node(&mut self, pos: Vf2d, parent_id: ShapeID) -> bool;
    fn hit_node(&mut self, pos: Vf2d) -> Option<&mut Node>;
}
#[derive(Clone, Debug)]
struct BoxShape {
    nodes: Vec<Node>,
    col: Color,
    shape_id: ShapeID,
}
impl Shape for BoxShape {
    fn draw_youself(&self, game: &mut px::Engine, transform: &world_transform::Transform) {
        if self.nodes.len() >= 2 {
            let topleft = transform.world_to_screen(self.nodes[0].pos);
            let bottom_right = transform.world_to_screen(self.nodes[1].pos);

            game.draw_rect(topleft, bottom_right - topleft, self.col);
        }
    }

    fn draw_nodes(&self, game: &mut px::Engine, transform: &world_transform::Transform) {
        for node in &self.nodes {
            game.fill_circle(
                transform.world_to_screen(node.pos),
                (transform.scale().x * 3.0) as u32,
                Color::RED,
            );
        }
    }

    fn max_node(&self) -> u8 {
        2
    }

    fn set_current_node(&mut self, pos: Vf2d, parent_id: ShapeID) -> bool {
        
    }

    fn hit_node(&mut self, pos: Vf2d) -> Option<&mut Node> {
        for node in &mut self.nodes {
            if (pos - node.pos).mag() <= 0.01 {
                return Some(node);
            }
        }
        return None;
    }
}
#[derive(Clone, Debug)]
struct LineShape {
    nodes: Vec<Node>,
    col: Color,
    shape_id: ShapeID,
}

impl Shape for LineShape {
    fn draw_youself(&self, game: &mut px::Engine, transform: &world_transform::Transform) {
        if self.nodes.len() >= 2 {
            let start = transform.world_to_screen(self.nodes[0].pos);
            let end = transform.world_to_screen(self.nodes[1].pos);

            game.draw_line(start, end, self.col);
        }
    }

    fn max_node(&self) -> u8 {
        2
    }

    fn hit_node(&mut self, pos: Vf2d) -> Option<&mut Node> {
        for node in &mut self.nodes {
            if (pos - node.pos).mag() <= 0.01 {
                return Some(node);
            }
        }
        return None;
    }

    fn draw_nodes(&self, game: &mut px::Engine, transform: &world_transform::Transform) {
        for node in &self.nodes {
            game.fill_circle(
                transform.world_to_screen(node.pos),
                (transform.scale().x * 3.0) as u32,
                Color::RED,
            );
        }
    }
}

#[derive(Debug, Clone)]
enum ShapeTypes {
    Circle,
    Box,
    Spline,
    Line,
}

#[derive(Clone, Debug)]
pub struct Node {
    parent: ShapeID,
    pos: Vf2d,
}

async fn init() {
    let game = px::EngineWrapper::new("World Transform".to_owned(), (800, 450, 2)).await;
    let mut transform = world_transform::Transform::new(&game, (10f32, 10f32).into());

    let grid_size = 1.0;
    let mut shapes: HashMap<ShapeID, Box<dyn Shape>> = HashMap::new();
    let mut current_shape: Option<(ShapeID, Box<dyn Shape>)> = None;
    let mut new_shape_type = ShapeTypes::Line;
    let mut shape_id_gen = ShapeIDGen::new();
    let mut current_node: Option<&mut Node> = None;
    game.run(move |game| {
        let mouse: Vu2d = game.get_mouse_location().into();
        transform.handle_pan(
            game,
            |game| game.get_mouse_btn(px::inputs::MouseBtn::Middle).pressed,
            |game| game.get_mouse_btn(px::inputs::MouseBtn::Middle).held,
        );

        transform.handle_zoom(
            game,
            |game| {
                game.get_mouse_wheel() == px::inputs::MouseWheel::Up
                    || game.get_key(px::inputs::Keycodes::A).any()
            },
            |game| {
                game.get_mouse_wheel() == px::inputs::MouseWheel::Down
                    || game.get_key(px::inputs::Keycodes::E).any()
            },
        );

        transform.round_corner();

        let cursor_snaped = {
            let mut mouse_after = transform.get_mouse_location(game);
            mouse_after.x = ((mouse_after.x + 0.5) * grid_size).floor();
            mouse_after.y = ((mouse_after.y + 0.5) * grid_size).floor();
            mouse_after
        };

        if game.get_mouse_btn(px::inputs::MouseBtn::Left).pressed {
            if current_shape.is_none() {
                let new_id = shape_id_gen.get_one();
                current_shape = Some((
                    new_id,
                    match new_shape_type {
                        ShapeTypes::Box => Box::new(BoxShape {
                            nodes: Vec::with_capacity(2),
                            shape_id: new_id,
                            col: Color::WHITE,
                        }),
                        ShapeTypes::Circle => {
                            todo!()
                        }
                        ShapeTypes::Spline => {
                            todo!()
                        }
                        ShapeTypes::Line => Box::new(LineShape {
                            nodes: Vec::with_capacity(2),
                            shape_id: new_id,
                            col: Color::WHITE,
                        }),
                    },
                ));
            }
        }

        if let Some((id, mut shape)) = current_shape {
            current_node = shape.get_next_node(cursor_snaped);
        }

        for shape in shapes.values() {
            shape.draw_youself(game, &mut transform);
            shape.draw_nodes(game, &mut transform);
        }

        if let Some((_, shape)) = &current_shape {
            print!(".");
            shape.draw_youself(game, &mut transform);
            shape.draw_nodes(game, &mut transform);
        }

        game.clear(px::Color::VERY_DARK_BLUE);

        for x in FloatRange::new(
            transform.world_top_left().x,
            transform.world_bottom_right().x,
            grid_size,
        ) {
            for y in FloatRange::new(
                transform.world_top_left().y,
                transform.world_bottom_right().y,
                grid_size,
            ) {
                let point = transform.world_to_screen((x, y).into());
                game.draw(point, Color::BLUE);
            }
        }

        let top_middle = transform.world_to_screen((0f32, transform.world_top_left().y).into());
        let bot_middle = transform.world_to_screen((0f32, transform.world_bottom_right().y).into());

        game.draw_line_dotted(top_middle, bot_middle, Color::GREY, 0xF0F0F0F0);

        let left_middle = transform.world_to_screen((transform.world_top_left().x, 0f32).into());
        let right_middle =
            transform.world_to_screen((transform.world_bottom_right().x, 0f32).into());

        game.draw_line_dotted(left_middle, right_middle, Color::GREY, 0xF0F0F0F0);

        game.draw_circle(transform.world_to_screen(cursor_snaped), 3, Color::YELLOW);

        game.draw_text(
            (-8, 0),
            1,
            Color::YELLOW,
            &format!(" X:{}\nY:{}", cursor_snaped.x, cursor_snaped.y),
        );

        Ok(true)
    });
}
pub fn main() {
    px::launch(init())
}
