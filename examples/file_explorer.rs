extern crate pixel_engine as engine;
use engine::traits::*;
fn main() {
    println!("[WIP] THIS EXAMPLE IS NOT WORKING AS INTENDED");
    let game = engine::EngineWrapper::new("Path".to_owned(), (500, 500, 1));
    let mut pw = PathView::new(PathBuf::from("."));
    use engine::inputs::Keycodes::{Down, Escape, Left, Right, Up};
    game.run(move |game: &mut engine::Engine| {
        game.screen.clear([0x00, 0x00, 0x00].into());
        if game.get_key(Escape).pressed {
            return Ok(false);
        }
        if game.get_key(Up).pressed {
            pw.select_prev();
        }
        if game.get_key(Down).pressed {
            pw.select_next();
        }
        if game.get_key(Left).pressed {
            pw.goto_parent();
        }
        if game.get_key(Right).pressed {
            pw.goto_select();
        }
        let current_filename = match pw.current_path.file_name() {
            Some(filename) => filename.to_str().unwrap().to_string(),
            None => String::from("."),
        };
        game.screen
            .draw_text(0, 0, 2, [1.0, 1.0, 1.0].into(), &current_filename);
        for i in 0..pw.child.len() {
            if i == pw.child_index {
                //game.screen.draw_line()
                game.screen.draw_text(
                    200,
                    20 * i as u32,
                    2,
                    [0.0, 1.0, 1.0].into(),
                    match &pw.get_child_name()[i] {
                        Some(name) => name,
                        None => "",
                    },
                );
            } else {
                game.screen.draw_text(
                    200,
                    20 * i as u32,
                    2,
                    [1.0, 1.0, 1.0].into(),
                    match &pw.get_child_name()[i] {
                        Some(name) => name,
                        None => "",
                    },
                );
            }
        }
        Ok(true)
    });
}
use std::path::PathBuf;

struct PathView {
    current_path: PathBuf,
    child: Vec<PathBuf>,
    child_index: usize,
}

impl PathView {
    pub fn new(path: PathBuf) -> Self {
        let mut pw = PathView {
            current_path: path,
            child: Vec::new(),
            child_index: 0,
        };
        pw.populate_child();
        pw
    }
    pub fn populate_child(&mut self) {
        let dirs = self.current_path.read_dir();
        if None == self.current_path.parent() {
            return;
        }
        match dirs {
            Err(_) => {
                self.current_path = match self.current_path.parent() {
                    None => self.current_path.clone(),
                    Some(f) => f.to_path_buf(),
                };
                self.populate_child();
            }
            Ok(iters) => {
                for entry in iters {
                    if let Ok(entry) = entry {
                        self.child.push(entry.path());
                    }
                }
            }
        }
    }
    pub fn select_next(&mut self) {
        if self.child_index + 1 < self.child.len() {
            self.child_index += 1;
        }
    }
    pub fn select_prev(&mut self) {
        if self.child_index > 0 {
            self.child_index -= 1;
        }
    }
    pub fn goto_select(&mut self) {
        if self.child_index >= self.child.len() && !self.child.is_empty() {
            self.child_index = self.child.len() - 1;
        }
        if self.child.is_empty() {
            return;
        }
        self.current_path = self.child[self.child_index].clone();
        self.child.clear();
        self.populate_child();
        self.child_index = 0;
    }

    pub fn get_child_name(&self) -> Vec<Option<String>> {
        self.child
            .iter()
            .map(|x| match x.file_name() {
                Some(file) => Some(file.to_str().unwrap().to_string()),
                None => None,
            })
            .collect()
    }
    pub fn goto_parent(&mut self) {
        if let Some(parent) = self.current_path.parent() {
            self.current_path = parent.to_path_buf();
            self.populate_child();
            self.child_index = 0;
        }
    }
}
