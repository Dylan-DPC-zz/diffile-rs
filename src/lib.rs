#[macro_use]
extern crate serde_json;
extern crate serde;
mod file_handler;
mod differentiator;
use std::path::PathBuf;
use file_handler::{File, LineAwareFile};

pub struct Diffile {
    path: PathBuf,
    contents: LineAwareFile
}

impl Diffile {
    pub fn new(path: PathBuf) -> Diffile {
        Diffile { path, contents: LineAwareFile::new()}
    }

    pub fn step(mut self) -> Diffile {
        self.contents = File::new(&self.path).read().unwrap();

        self
    }

    pub fn diff(&self) {

    }
}
