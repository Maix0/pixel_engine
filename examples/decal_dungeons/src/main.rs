use engine::vector2::{Vf2d, Vi2d, Vu2d};
use pixel_engine as engine;
use pixel_engine::{inputs, traits::*, Color, EngineWrapper, Sprite};

#[allow(unused_assignments)]

struct World {
    size: Vu2d,
    cells: Vec<Cell>,
}

impl World {
    fn new(x: u32, y: u32) -> Self {
        Self {
            size: Vu2d { x, y },
            cells: Vec::with_capacity((x * y) as _),
        }
    }

    fn get_cell(&mut self, c: Vu2d) -> Option<&mut Cell> {
        self.cells.get_mut((c.x * c.y) as usize)
    }
}

struct Cell {
    wall: bool,
    id: [Vf2d; 6],
}

#[derive(Clone, Copy, Debug)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}
impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}
impl From<(f32, f32, f32)> for Vec3 {
    fn from(s: (f32, f32, f32)) -> Self {
        Self::new(s.0, s.1, s.2)
    }
}
impl From<(f64, f64, f64)> for Vec3 {
    fn from(s: (f64, f64, f64)) -> Self {
        Self::new(s.0 as f32, s.1 as f32, s.2 as f32)
    }
}

fn create_select_cube() -> Sprite {
    let mut spr = Sprite::new(32, 32);
    let alpha = [255, 128 + 64, 128, 64];
    for x in 0..spr.width() {
        for i in 0..4usize {
            spr.set_pixel(x, i as u32, [255, 255, 0, alpha[i]].into());
            spr.set_pixel(x, spr.height() - 1 - i as u32, [255, 255, 0, alpha[i]].into());
        }
    }
    spr
}

#[derive(Debug)]
struct Quad {
    points: [Vec3; 4],
    tile: Vf2d,
}

trait ToVec3 {
    fn vec3(&self) -> Vec3;
}
impl ToVec3 for Vf2d {
    fn vec3(&self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: 0.0,
        }
    }
}

#[repr(C)]
enum Face {
    Floor = 0,
    North = 1,
    East = 2,
    South = 3,
    West = 4,
    Top = 5,
}

fn create_cube(
    cell: Vf2d,
    angle: f32,
    pitch: f32,
    scale: f32,
    camera: &Vec3,
    screen_size: Vf2d,
    //viewport: (Vf2d, f32, f32),
) -> [Vec3; 8] {
    // Unit _cube
    let mut unit_cube = [Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    }; 8];
    let mut rot_cube = unit_cube.clone();
    let mut world_cube = unit_cube.clone();
    let mut proj_cube = unit_cube.clone();
    unit_cube[0] = (0.0, 0.0, 0.0).into();
    unit_cube[1] = (scale, 0.0, 0.0).into();
    unit_cube[2] = (scale, -scale, 0.0).into();
    unit_cube[3] = (0.0, -scale, 0.0).into();
    unit_cube[4] = (0.0, 0.0, scale).into();
    unit_cube[5] = (scale, 0.0, scale).into();
    unit_cube[6] = (scale, -scale, scale).into();
    unit_cube[7] = (0.0, -scale, scale).into();

    // Translate _cube in X-Z Plane
    for i in 0..8 {
        unit_cube[i].x += cell.x * scale - camera.x;
        unit_cube[i].y += -camera.y;
        unit_cube[i].z += cell.y * scale - camera.z;
    }

    // Rotate _cube in Y-Axis around origin
    let (s, c) = angle.sin_cos();

    for i in 0..8 {
        rot_cube[i].x = unit_cube[i].x * c + unit_cube[i].z * s;
        rot_cube[i].y = unit_cube[i].y;
        rot_cube[i].z = unit_cube[i].x * -s + unit_cube[i].z * c;
    }

    // Rotate _cube in X-Axis around origin (tilt slighly overhead)
    let (s, c) = pitch.sin_cos();
    for i in 0..8 {
        world_cube[i].x = rot_cube[i].x;
        world_cube[i].y = rot_cube[i].y * c - rot_cube[i].z * s;
        world_cube[i].z = rot_cube[i].y * s + rot_cube[i].z * c;
    }
    /*
    let (center, size_h, size_v) = viewport;
    let left = center.x - size_h;
    let right = center.x + size_h;
    let top = center.y + size_v;
    let bottom = center.y - size_v;
    let near = 0.1;
    let far = 100.0;
    for i in 0..8 {
        proj_cube[i].x =
            (2.0 / (right - left)) * world_cube[i].x - ((right + left) / (right - left));
        proj_cube[i].y =
            (2.0 / (top - bottom)) * world_cube[i].y - ((top + bottom) / (top - bottom));
        proj_cube[i].z = (2.0 / (far - near)) * world_cube[i].z - ((far + near) / (far - near));
        proj_cube[i].x *= -right;
        proj_cube[i].y *= -top;
        proj_cube[i].x += right;
        proj_cube[i].y += top;
    }
    */
    for i in 0..8 {
        proj_cube[i].x = world_cube[i].x + screen_size.x * 0.5;
        proj_cube[i].y = world_cube[i].y + screen_size.y * 0.5;
        proj_cube[i].z = world_cube[i].z;
    }
    proj_cube
}
fn calculate_visible_faces(cube: [Vec3; 8]) -> [bool; 6] {
    let check_normal = |v1: usize, v2: usize, v3: usize| {
        let a: Vf2d = (cube[v1].x, cube[v1].y).into();
        let b: Vf2d = (cube[v2].x, cube[v2].y).into();
        let c: Vf2d = (cube[v3].x, cube[v3].y).into();
        return (b - a).cross(&(c - a)) > 0.0;
    };

    [
        check_normal(4, 0, 1),
        check_normal(3, 0, 1),
        check_normal(6, 5, 4),
        check_normal(7, 4, 0),
        check_normal(2, 1, 5),
        check_normal(7, 3, 2),
    ]
}

fn get_face_quad(
    cell: Vf2d,
    angle: f32,
    pitch: f32,
    scale: f32,
    camera: &Vec3,
    render: &mut Vec<Quad>,
    screen_size: Vf2d,
    world: &mut World,
    visible: &[bool; 6],
) {
    let proj_cube = create_cube(cell, angle, pitch, scale, camera, screen_size);

    let cell = {
        let c = world.get_cell((cell.x as u32, cell.y as u32).into());
        if c.is_none() {
            return;
        } else {
            c.unwrap()
        }
    };
    let mut make_face = |v1: usize, v2: usize, v3: usize, v4: usize, f: Face| {
        render.push(Quad {
            points: [proj_cube[v1], proj_cube[v2], proj_cube[v3], proj_cube[v4]],
            tile: cell.id[f as usize],
        });
    };
    if !cell.wall {
        if visible[Face::Floor as usize] {
            make_face(4, 0, 1, 5, Face::Floor);
        }
    } else {
        if visible[Face::South as usize] {
            make_face(3, 0, 1, 2, Face::South);
        }
        if visible[Face::North as usize] {
            make_face(6, 5, 4, 7, Face::North);
        }
        if visible[Face::East as usize] {
            make_face(7, 4, 0, 3, Face::East);
        }
        if visible[Face::West as usize] {
            make_face(2, 1, 5, 6, Face::West);
        }
        if visible[Face::Top as usize] {
            make_face(7, 3, 2, 6, Face::Top);
        }
    }
}

async fn init() {
    let mut game = EngineWrapper::new("Decal Dungeons".to_owned(), (255, 255, 2)).await;
    let mut world = World::new(16, 16);
    let tile_size: Vf2d = (32.0, 32.0).into();
    let mut base_wall_id: [Vf2d; 6] = [
        (0f32, 1f32).into(),
        (3f32, 10f32).into(),
        (3f32, 10f32).into(),
        (3f32, 10f32).into(),
        (3f32, 10f32).into(),
        (2f32, 10f32).into(),
    ];
    for w in &mut base_wall_id {
        *w = *w * tile_size;
    }
    for _ in 0..world.cells.capacity() {
        world.cells.push(Cell {
            wall: false,
            id: base_wall_id,
        });
    }

    let spr = Sprite::load_from_file(&std::path::Path::new("./tiles/dg_features32.gif"))
        .expect("Unable to load tiles");
    let tile_map = game.create_decal(&spr);
    let selected_cube = game.create_decal(&create_select_cube());

    let mut camera_angle = 0f32;
    let mut camera_target = camera_angle;
    let mut camera_pitch = 5.5f32;
    let mut camera_zoom = 16f32;

    let mut cursor: Vf2d = (0.0, 0.0).into();
    let mut tile_cursor: Vf2d = (32.0, 32.0).into();
    let mut vec_quad: Vec<Quad> = Vec::with_capacity(100);
    game.run(move |game| {
        game.clear(Color::BLACK);
        let mouse: Vu2d = game.get_mouse_location().into();
        let mouse: Vf2d = (mouse.x as f32, mouse.y as f32).into();
        {
            use pixel_engine::inputs::Keycodes;
            if game.get_key(Keycodes::Tab).held {
                game.draw_sprite((0, 0), 1, &spr, (false, false));
                game.draw_rect(
                    {
                        let t = tile_cursor * tile_size;
                        Vi2d {
                            x: t.x as i32,
                            y: t.y as i32,
                        }
                    },
                    Vi2d {
                        x: tile_size.x as i32,
                        y: tile_size.y as i32,
                    },
                    Color::WHITE,
                );
                if game
                    .get_mouse_btn(pixel_engine::inputs::MouseBtn::Right)
                    .pressed
                {
                    tile_cursor = mouse / tile_size;
                    return Ok(true);
                }
                return Ok(true);
            }
            if game.get_key(Keycodes::Z).any() {
                camera_pitch += game.elapsed as f32;
            }
            if game.get_key(Keycodes::S).any() {
                camera_pitch -= game.elapsed as f32;
            }
            if game.get_key(Keycodes::D).any() {
                camera_target += game.elapsed as f32;
            }
            if game.get_key(Keycodes::Q).any() {
                camera_target -= game.elapsed as f32;
            }
            if game.get_key(Keycodes::X).any() {
                camera_zoom += 5f32 * game.elapsed as f32;
            }
            if game.get_key(Keycodes::W).any() {
                camera_zoom -= 5f32 * game.elapsed as f32;
            }

            if game.get_key(Keycodes::Numpad2).pressed {
                camera_target = std::f32::consts::PI * 0.0f32;
            }
            if game.get_key(Keycodes::Numpad1).pressed {
                camera_target = std::f32::consts::PI * 0.25f32;
            }
            if game.get_key(Keycodes::Numpad4).pressed {
                camera_target = std::f32::consts::PI * 0.5f32;
            }
            if game.get_key(Keycodes::Numpad7).pressed {
                camera_target = std::f32::consts::PI * 0.75f32;
            }
            if game.get_key(Keycodes::Numpad8).pressed {
                camera_target = std::f32::consts::PI * 1.0f32;
            }
            if game.get_key(Keycodes::Numpad9).pressed {
                camera_target = std::f32::consts::PI * 1.25f32;
            }
            if game.get_key(Keycodes::Numpad6).pressed {
                camera_target = std::f32::consts::PI * 1.5f32;
            }
            if game.get_key(Keycodes::Numpad3).pressed {
                camera_target = std::f32::consts::PI * 1.75f32;
            }

            if game.get_key(Keycodes::Key1).pressed {
                world
                    .get_cell((cursor.x as u32, cursor.y as u32).into())
                    .unwrap()
                    .id[Face::North as usize] = tile_cursor * tile_size;
            }
            if game.get_key(Keycodes::Key2).pressed {
                world
                    .get_cell((cursor.x as u32, cursor.y as u32).into())
                    .unwrap()
                    .id[Face::East as usize] = tile_cursor * tile_size;
            }
            if game.get_key(Keycodes::Key3).pressed {
                world
                    .get_cell((cursor.x as u32, cursor.y as u32).into())
                    .unwrap()
                    .id[Face::South as usize] = tile_cursor * tile_size;
            }
            if game.get_key(Keycodes::Key4).pressed {
                world
                    .get_cell((cursor.x as u32, cursor.y as u32).into())
                    .unwrap()
                    .id[Face::West as usize] = tile_cursor * tile_size;
            }
            if game.get_key(Keycodes::Key5).pressed {
                world
                    .get_cell((cursor.x as u32, cursor.y as u32).into())
                    .unwrap()
                    .id[Face::Floor as usize] = tile_cursor * tile_size;
            }
            if game.get_key(Keycodes::Key6).pressed {
                world
                    .get_cell((cursor.x as u32, cursor.y as u32).into())
                    .unwrap()
                    .id[Face::Top as usize] = tile_cursor * tile_size;
            }

            if game.get_key(Keycodes::Left).pressed {
                cursor.x -= 1.0;
            }
            if game.get_key(Keycodes::Right).pressed {
                cursor.x += 1.0;
            }
            if game.get_key(Keycodes::Up).pressed {
                cursor.y -= 1.0;
            }
            if game.get_key(Keycodes::Down).pressed {
                cursor.y += 1.0;
            }
            if cursor.x < 0.0 {
                cursor.x = 0.0;
            }
            if cursor.y < 0.0 {
                cursor.y = 0.0;
            }
            if cursor.x >= world.size.x as f32 {
                cursor.x = world.size.x as f32 - 1.0;
            }
            if cursor.y >= world.size.y as f32 {
                cursor.y = world.size.y as f32 - 1.0;
            }
            if camera_pitch < 1.0 {
                camera_pitch = 1.0;
            }
        }
        camera_angle += (camera_target - camera_angle) * 10.0 * game.elapsed as f32;
        let camera_pos = (cursor + Vf2d { x: 0.5, y: 0.5 }) * camera_zoom;
        let cull_cube = create_cube(
            (0.0, 0.0).into(),
            camera_angle,
            camera_pitch,
            camera_zoom,
            &camera_pos.vec3(),
            (game.size.0 as f32, game.size.1 as f32).into(),
        );
        let visible = calculate_visible_faces(cull_cube);
        println!("{:?}", visible);
        vec_quad.clear();
        for x in 0..world.size.x {
            for y in 0..world.size.y {
                get_face_quad(
                    (x as f32, y as f32).into(),
                    camera_angle,
                    camera_pitch,
                    camera_zoom,
                    &(camera_pos.x, 0f32, camera_pos.y).into(),
                    &mut vec_quad,
                    (game.size.0 as f32, game.size.1 as f32).into(),
                    &mut world,
                    &visible,
                );
            }
        }
        vec_quad.sort_by(|q1, q2| {
            use std::cmp::Ordering;
            let z1 = (q1.points[0].z + q1.points[1].z + q1.points[2].z + q1.points[3].z) * 0.25;
            let z2 = (q2.points[0].z + q2.points[1].z + q2.points[2].z + q2.points[3].z) * 0.25;
            if z1 < z2 {
                Ordering::Less
            } else if z1 == z2 {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        });
        for q in &vec_quad {
            game.draw_warped_partial_decal(
                [
                    (q.points[0].x, q.points[0].y),
                    (q.points[1].x, q.points[1].y),
                    (q.points[2].x, q.points[2].y),
                    (q.points[3].x, q.points[3].y),
                ],
                (q.tile.x, q.tile.y),
                (tile_size.x, tile_size.y),
                &tile_map,
            );
        }
        vec_quad.clear();
        get_face_quad(
            cursor,
            camera_angle,
            camera_pitch,
            camera_zoom,
            &(camera_pos.x, 0.0, camera_pos.y).into(),
            &mut vec_quad,
            (game.size.0 as f32, game.size.1 as f32).into(),
            &mut world,
            &visible,
        );
        println!("{:?}", vec_quad.len());
        for q in &vec_quad {
            //dbg!(&q);
            game.draw_warped_decal(
                [
                    (q.points[0].x, q.points[0].y),
                    (q.points[1].x, q.points[1].y),
                    (q.points[2].x, q.points[2].y),
                    (q.points[3].x, q.points[3].y),
                ],
                &selected_cube,
            );
        }
        game.draw_partial_decal(
            Vf2d::from((10.0, 10.0)),
            &tile_map,
            tile_cursor * tile_size,
            tile_size,
        );

        //game.draw_decal_scaled(cursor, &tile_map, (1.0, 1.0).into());

        Ok(!game.get_key(inputs::Keycodes::Escape).any())
    });
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        use std::panic;
        panic::set_hook(Box::new(pixel_engine::console_error_panic_hook::hook));
        pixel_engine::wasm_bindgen_futures::spawn_local(init());
    };
    #[cfg(not(target_arch = "wasm32"))]
    pixel_engine::futures::executor::block_on(init());
}
