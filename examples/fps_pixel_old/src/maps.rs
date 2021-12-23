extern crate pixel_engine as engine;
extern crate ron;
extern crate serde;
//use engine::Keycode;
use engine::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct World {
    pub map: Map,
    pub objs: Vec<Objects>,
    pub tiles: std::collections::HashMap<char, Tile>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Map {
    pub map: String,
    pub w: u64,
    pub h: u64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Objects {
    #[serde(skip)]
    pub sprite: Option<Sprite>,
    pub sprite_path: String,
    pub x: f64,
    pub y: f64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tile {
    #[serde(skip)]
    pub sprite: Option<Sprite>,
    pub sprite_path: String,
    pub chr: char,
}
impl Tile {
    pub fn load(&mut self) -> Result<(), String> {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push(std::path::Path::new(&self.sprite_path));

        self.sprite = Some(Sprite::load_from_file(&path)?);
        Ok(())
    }
}

pub struct WorldConstructor {
    pub map: Vec<String>,
    pub objects: Vec<Objects>,
    pub tiles: std::collections::HashMap<char, Tile>,
}
impl WorldConstructor {
    pub fn new() -> WorldConstructor {
        WorldConstructor {
            map: Vec::new(),
            objects: Vec::new(),
            tiles: std::collections::HashMap::new(),
        }
    }
    pub fn load_file(path_str: String) -> Result<WorldConstructor, String> {
        use std::io::prelude::*;
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push(std::path::Path::new(&path_str));
        if path.exists() {
            let mut file = std::fs::File::open(path).map_err(|e| e.to_string())?;
            let mut data = String::new();
            file.read_to_string(&mut data).map_err(|e| e.to_string())?;
            let world = ron::de::from_str::<'_, World>(&data).map_err(|e| e.to_string())?;
            Ok(WorldConstructor::from_world(world))
        } else {
            Ok(WorldConstructor::new())
        }
    }
    pub fn from_world(world: World) -> WorldConstructor {
        //let mut temp_map = world.map.split();
        let mut cmap: Vec<String> = Vec::new();
        for chr in world.map.map.split("") {
            if cmap.len() > 0 {
                let last_index = cmap.len() - 1;
                if cmap[last_index].len() < world.map.w as usize {
                    cmap[last_index].push_str(chr)
                } else {
                    cmap.push(chr.to_owned())
                }
            } else {
                cmap.push(chr.to_owned());
            }
        }
        cmap.pop();
        WorldConstructor {
            map: cmap,
            tiles: world.tiles,
            objects: world.objs,
        }
    }
    pub fn to_world(&mut self) -> World {
        let mut w = 0;
        let h = self.map.len();
        let mut map: Vec<String> = self.map.clone();
        for r in &map {
            if r.len() > w {
                w = r.len();
            }
        }
        let mut index = 0;
        for row in &self.map {
            if index > h {
                break;
            }
            let mut r = row.clone();
            if r.len() < w {
                while r.len() < w {
                    r.push('.');
                }
            }
            map[index] = r.to_owned();
            index += 1;
        }
        World {
            map: Map {
                map: map.join(""), /* STRING */
                w: w as u64,       /* u64 */
                h: h as u64,       /* u64 */
            },
            tiles: self.tiles.clone(),
            objs: self.objects.clone(),
        }
    }
    pub fn map_set_y(&mut self, len: usize) {
        if len > self.map.len() {
            while len > self.map.len() {
                self.map.push(String::new());
            }
        }
    }
    pub fn map_set_x(&mut self, len: usize) {
        for row in &mut self.map {
            if len > row.len() {
                while len >= row.len() {
                    row.push_str(".");
                }
            }
        }
    }
    pub fn map_set(&mut self, x: usize, y: usize, chr: char) {
        self.map[y] = change_char(self.map[y].clone(), chr, x);
    }
}
pub fn change_char(source: String, chr: char, index: usize) -> String {
    let mut res = String::new();
    let mut c_index = 0_usize;
    for c in source.chars() {
        if c_index == index {
            res.push(chr);
        } else {
            res.push(c);
        }
        c_index += 1;
    }
    res
}

impl World {
    pub fn get_2d(&self, x: i64, y: i64) -> Option<char> {
        self.map
            .map
            .chars()
            .nth((y * self.map.w as i64 + x) as usize)
    }
    pub fn load_all(&mut self) -> Result<(), String> {
        for tile in &mut self.tiles {
            tile.1.load()?;
        }
        Ok(())
    }
}
