extern crate pixel_engine as px;
#[macro_use]
extern crate rust_embed;
use px::graphics::Color;
use px::inputs::Keycodes;
use px::traits::*;
use px::vector2::*;
fn main() {
    px::launch(game());
}

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/tiles"]
struct Assets;

#[derive(Debug, Clone, Copy)]
struct Vec3d {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3d {
    const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}
impl std::ops::Mul<f32> for Vec3d {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl std::ops::Add<Self> for Vec3d {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.y,
        }
    }
}
impl std::ops::Sub<Self> for Vec3d {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.y,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Cube {
    vertex: [Vec3d; 8],
}

/*
unitCube[0] = { 0.0f, 0.0f, 0.0f };
unitCube[1] = { fScale, 0.0f, 0.0f };
unitCube[2] = { fScale, -fScale, 0.0f };
unitCube[3] = { 0.0f, -fScale, 0.0f };
unitCube[4] = { 0.0f, 0.0f, fScale };
unitCube[5] = { fScale, 0.0f, fScale };
unitCube[6] = { fScale, -fScale, fScale };
unitCube[7] = { 0.0f, -fScale, fScale };
*/

impl Cube {
    #[rustfmt::skip]
    const UNIT: Cube = Cube {
        vertex: [
            Vec3d::new(0.0, 0.0, 0.0),
            Vec3d::new(1.0, 0.0, 0.0),
            Vec3d::new(1.0,-1.0, 0.0),
            Vec3d::new(0.0,-1.0, 0.0),
            Vec3d::new(0.0, 0.0, 1.0),
            Vec3d::new(1.0, 0.0, 1.0),
            Vec3d::new(1.0,-1.0, 1.0),
            Vec3d::new(0.0,-1.0, 1.0),
        ],
    };

    fn scale(&self, scale: f32) -> Self {
        Self {
            vertex: self.vertex.map(|v| v * scale),
        }
    }

    fn displace(&self, direction: Vec3d) -> Self {
        Self {
            vertex: self.vertex.map(|v| v + direction),
        }
    }

    fn rotate_x_axis(&self, angle: f32) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self {
            vertex: self.vertex.map(|v| Vec3d {
                x: v.x * cos + v.z * sin,
                y: v.y,
                z: v.x * sin + v.z * cos,
            }),
        }
    }
    fn rotate_y_axis(&self, angle: f32) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self {
            vertex: self.vertex.map(|v| Vec3d {
                x: v.x,
                y: v.y * cos + v.z * sin,
                z: v.y * sin + v.z * cos,
            }),
        }
    }

    fn project(&self, dimention: Vf2d) -> Self {
        Self {
            vertex: self.vertex.map(|v| Vec3d {
                x: v.x + dimention.x * 0.5,
                y: v.y + dimention.y * 0.5,
                z: v.z,
            }),
        }
    }

    fn get_projected_cube(
        cell: Vf2d,
        angle: f32,
        pitch: f32,
        scale: f32,
        camera: Vec3d,
        dimention: Vf2d,
    ) -> Self {
        Cube::UNIT
            .scale(scale)
            .displace(Vec3d {
                x: cell.x * scale - camera.x,
                y: -camera.y,
                z: cell.y * scale - camera.z,
            })
            .rotate_x_axis(angle)
            .rotate_y_axis(pitch)
            .project(dimention)
    }
}

#[derive(Clone, Copy)]
struct CellData {
    texture_id: [Vi2d; 6],
}
#[derive(Clone, Copy)]
enum Cell {
    Floor(CellData),
    Wall(CellData),
}

impl Cell {
    fn get_data(&self) -> &CellData {
        match self {
            Cell::Floor(d) | Cell::Wall(d) => d,
        }
    }
}

#[rustfmt::skip]
#[repr(u8)]
enum Sides {
    Floor = 0,
    North = 1,
    East  = 2,
    South = 3,
    West  = 4,
    Top   = 5,
}

#[derive(Debug, Clone)]
struct Quad {
    points: [Vec3d; 4],
    tile_id: Vi2d,
}

struct World<const W: usize, const H: usize> {
    cells: [[Cell; W]; H],
}

impl<const W: usize, const H: usize> World<W, H> {
    fn new() -> Self {
        Self {
            cells: [[Cell::Floor(CellData {
                texture_id: [Vi2d { x: 0, y: 0 }; 6],
            }); W]; H],
        }
    }

    fn size() -> Vi2d {
        (W as i32, H as i32).into()
    }

    fn enumerate_cell(&self) -> impl Iterator<Item = (Vu2d, &Cell)> {
        self.cells
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().zip(std::iter::repeat(y as u32)))
            .map(|((x, cell), y)| (Vu2d { x: x as u32, y }, cell))
    }
    fn enumerate_cell_mut(&mut self) -> impl Iterator<Item = (Vu2d, &mut Cell)> {
        self.cells
            .iter_mut()
            .enumerate()
            .flat_map(|(y, row)| row.iter_mut().enumerate().zip(std::iter::repeat(y as u32)))
            .map(|((x, cell), y)| (Vu2d { x: x as u32, y }, cell))
    }
}

fn create_face_quads(
    cell_index: Vf2d,
    cell: &Cell,
    angle: f32,
    pitch: f32,
    scale: f32,
    camera: Vec3d,
    dimention: Vf2d,
    visible_faces: &[bool; 6],
    quads: &mut Vec<Quad>,
) {
    let proj = Cube::get_projected_cube(cell_index, angle, pitch, scale, camera, dimention);
    let mut make_face =
        |v1: usize, v2: usize, v3: usize, v4: usize, face: Sides, data: &CellData| {
            quads.push(Quad {
                points: [
                    proj.vertex[v1],
                    proj.vertex[v2],
                    proj.vertex[v3],
                    proj.vertex[v4],
                ],
                tile_id: cell.get_data().texture_id[face as usize],
            })
        };

    match cell {
        Cell::Floor(d) => {
            if visible_faces[Sides::Floor as usize] {
                make_face(4, 0, 1, 5, Sides::Floor, d);
            }
        }
        Cell::Wall(d) => {
            if visible_faces[Sides::South as usize] {
                make_face(3, 0, 1, 2, Sides::South, d);
            }
            if visible_faces[Sides::North as usize] {
                make_face(6, 5, 4, 7, Sides::North, d);
            }
            if visible_faces[Sides::East as usize] {
                make_face(7, 4, 0, 3, Sides::East, d);
            }
            if visible_faces[Sides::West as usize] {
                make_face(2, 1, 5, 6, Sides::West, d);
            }
            if visible_faces[Sides::Top as usize] {
                make_face(7, 3, 2, 6, Sides::Top, d);
            }
        }
    }
}

async fn game() {
    let mut wrapper = px::EngineWrapper::new("Decal Dungeon".to_string(), (512, 512, 2)).await;
    println!("{:?}", Assets::iter().collect::<Vec<_>>());

    let selected_spr = px::Sprite::load_image_bytes(
        &Assets::get("selected.png")
            .expect("No selected.png sprite found")
            .data,
    )
    .unwrap();
    let selected_decal = wrapper.create_decal(&selected_spr);
    //drop(selected_spr);

    let tilesheet_spr = px::Sprite::load_image_bytes(
        &Assets::get("tilesheet.gif")
            .expect("No tilesheet.gif sprite found")
            .data,
    )
    .unwrap();
    let tilesheet_decal = wrapper.create_decal(&tilesheet_spr);
    let tile_size: Vf2d = Vf2d { x: 32.0, y: 32.0 };
    //drop(tilesheet_spr);

    let mut camera_pos = Vec3d::new(0.0, 10.0, 0.0);
    let mut camera_angle = 0.0;
    let mut camera_pitch = 5.5;
    let mut camera_zoom = 16.0;

    let mut quads: Vec<Quad> = Vec::with_capacity(512);

    let mut visible_faces = [true; 6];

    let mut world: World<32, 32> = World::new();
    let selected_cell = Cell::Wall(CellData {
        texture_id: [(1, 1).into(); 6],
    });
    wrapper.clear(Color::BLACK);
    wrapper.run(move |engine: &mut px::Engine| {
        engine.draw_warped_decal(
            [(0.0, 0.0), (512.0, 0.0), (512.0, 512.0), (0.0, 512.0)],
            &tilesheet_decal,
        );
        //engine.clear(Color::BLACK);
        for (index, cell) in world.enumerate_cell() {
            let index = index.cast_f32();
            create_face_quads(
                index,
                cell,
                camera_angle,
                camera_pitch,
                camera_zoom,
                camera_pos,
                engine.size().cast_f32(),
                &visible_faces,
                &mut quads,
            );
        }

        create_face_quads(
            (0.0, 0.0).into(),
            &selected_cell,
            camera_angle,
            camera_pitch,
            camera_zoom,
            camera_pos,
            engine.size().cast_f32(),
            &visible_faces,
            &mut quads,
        );

        quads.sort_by(|lhs, rhs| {
            let z1 = lhs.points[0].z + lhs.points[1].z + lhs.points[2].z + lhs.points[3].z;
            let z2 = rhs.points[0].z + rhs.points[1].z + rhs.points[2].z + rhs.points[3].z;
            z1.total_cmp(&z2)
        });
        if engine.get_key(Keycodes::Space).pressed {
            dbg!(&quads);
        }

        for quad in quads.drain(..) {
            break;
            engine.draw_warped_decal(
                quad.points.map(|v| Vf2d { x: v.x, y: v.y }),
                //quad.tile_id.cast_f32() * tile_size,
                //tile_size,
                &tilesheet_decal,
            );
        }

        if engine.get_key(Keycodes::Escape).any() {
            // engine.destroy_decal(tilesheet_decal);
            // engine.destroy_decal(selected_decal);
            return Ok(false);
        }
        Ok(true)
    });
}
