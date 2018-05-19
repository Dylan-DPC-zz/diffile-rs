#[macro_use]
extern crate serde_json;
extern crate serde;
mod file_handler;
mod differentiator;
use std::path::PathBuf;
use file_handler::{File, LineAwareFile};
use differentiator::{Differ, Changes};
use std::error::Error;
use std::fmt::{Display, Result as FmtResult, Formatter};

pub struct Diffile {
    file: File,
    contents: Vec<LineAwareFile>
}

impl Diffile {
    pub fn new(path: &PathBuf) -> Diffile {
        Diffile { file: File::new(path), contents: vec![] }
    }

    pub fn step(mut self) -> Diffile {
        self.contents.push(self.file.read().unwrap());

        self
    }

    pub fn diff(self) -> Result<Changes, DiffError> {
        let length = self.contents.capacity();

        match length {
            len if len < 2 => Err(DiffError),
            _ => Ok(Differ::new(self.contents.get(length - 2).unwrap(), self.contents.last().unwrap()).diff())
        }
    }

    pub fn rollback(&self) {
        self.file.apply_changes(self.contents.last().unwrap())
        self.file.write(self.contents.last().unwrap())
    }
}

#[derive(Debug)]
pub struct DiffError;

impl Error for DiffError {
    fn description(&self) -> &str {
        "file hasn't been revisioned enough"
    }
}

impl Display for DiffError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "file hasn't been revisioned enough")
    }

}

#[test]
pub fn test_revision_a_file() {
    use std::collections::HashMap;

    let contents: HashMap<usize, String> = [(1, "foo".to_string())].iter().cloned().collect();
    let first_edition = LineAwareFile::from(contents);
    let contents: HashMap<usize, String> = [(1, "foo".to_string()), (2, "baz".to_string())].iter().cloned().collect();
    let second_edition = LineAwareFile::from(contents);
    let changes = Diffile { file: File::new(&PathBuf::from("_")), contents: vec![first_edition, second_edition] }.diff();
    println!("{:?}", changes);
}