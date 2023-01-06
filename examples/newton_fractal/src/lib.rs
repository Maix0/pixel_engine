extern crate nalgebra as na;
extern crate pixel_engine as px;
#[macro_use]
extern crate pixel_engine_console as pxc;

use pxc::{ConsoleEngine, GameWrapper};

mod expr;
pub mod threaded;

struct Game {}

#[cfg(target_arch = "wasm32")]
extern crate wasm_bindgen;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

impl px::Game for Game {
    fn create(_engine: &mut px::Engine) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Game {})
    }

    fn update(&mut self, engine: &mut px::Engine) -> Result<bool, Box<dyn std::error::Error>> {
        if engine.get_key(px::inputs::Keycodes::Space).pressed {
            engine.open_console(px::inputs::Keycodes::Escape, false);
        }
        Ok(!(engine.get_key(px::inputs::Keycodes::Escape).pressed && !engine.is_console_opened()))
    }
}

impl pxc::ConsoleGame for Game {
    fn receive_console_input(&mut self, engine: &mut px::Engine, mut input: String) {
        input.make_ascii_lowercase();
        let commands = shlex::split(&input).unwrap_or_default();

        match commands.get(0).map(AsRef::as_ref).unwrap_or_default() {
            "help" => {
                if let Some(cmd) = commands.get(1) {
                    match cmd.as_str() {
                        "equation" => {
                            cinfo!("Command 'equation':");
                            cinfo!("Usage: 'equation set <equation>'");
                            cinfo!("\tSet the equation to use");
                            cinfo!("Usage: 'equation get'");
                            cinfo!("\tGet the equation used");
                        }
                        "size" => {
                            cinfo!("Command 'size':");
                            cinfo!("Usage: 'size set <x> <y>'");
                            cinfo!("\tSet the render size");
                            cinfo!("Usage: 'size get'");
                            cinfo!("\tGet the render size");
                        }
                        "render" => {
                            cinfo!("Command 'render':");
                            cinfo!("Usage: 'render'");
                            cinfo!("\tRender with the current parameters");
                        }
                        "roots" => {
                            cinfo!("Command 'roots':");
                            cinfo!("Usage: 'size solve <trys>'");
                            cinfo!("\tSolves for the roots of the current equation with a number of trys");
                            cinfo!("Usage: 'size set (<root1>, <root2>...)'");
                            cinfo!("\tSet the roots to the list of values");
                        }
                        _ => {
                            cerror!("Command '{cmd}' not found");
                            cerror!("Type 'help' to get a list of commands");
                        }
                    }
                } else {
                    cinfo!("List of commands:");
                    cinfo!("\t- help     => Show this message");
                    cinfo!("\t- equation => Manipulate the equation");
                    cinfo!("\t- size     => Manipulate the render size");
                    cinfo!("\t- render   => Renders the fractal");
                    cinfo!("\t- roots    => Manipulate the equation's roots");
                    cinfo!("Use 'help <cmd>' to get more detail on a command");
                }
            }
            _ => {
                cerror!("Type 'help' to get a list of commands");
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn run() {
    px::start::<GameWrapper<Game>>("Newton Fractal", (500, 500), 2);
}
