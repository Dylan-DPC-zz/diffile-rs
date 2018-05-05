use std::fs;
use std::io::{BufReader, Read, Error};
use std::path::PathBuf;
use std::str::FromStr;
use std::collections::HashMap;
use std::convert::From;

#[derive(Clone, Debug, Hash)]
pub struct File {
   path: PathBuf
}

impl File {
    pub fn new(path: &PathBuf) -> File {
        File { path: path.clone() }
    }

    pub fn read(&self) -> Result<LineAwareFile, Error> {
        let file = fs::File::open(&self.path)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        LineAwareFile::from_str(contents.as_str())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LineAwareFile {
    pub contents: HashMap<usize, String>
}

impl LineAwareFile {
    pub fn new() -> LineAwareFile {
        LineAwareFile { contents: HashMap::new() }
    }
}

impl FromStr for LineAwareFile {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<String> = s.split('\n').map(|word| String::from(word) ).collect();
        Ok(LineAwareFile::from(lines))
    }
}

impl From<HashMap<usize,String>> for LineAwareFile {
    fn from(s: HashMap<usize, String>) -> Self {
        LineAwareFile { contents: s}
    }
}

impl From<Vec<String>> for LineAwareFile {
    fn from(s: Vec<String>) -> Self {
        let mut map:HashMap<usize, String> = HashMap::new();
        s.into_iter().enumerate().for_each(| (number, line)| {
            map.insert(number, line);
        });

        LineAwareFile { contents: map}
    }
}


#[test]
pub fn test_read_file() {
    let file = File::new(&PathBuf::from("tests/foo"));
    let output = file.read();
    println!("{:?}", output.unwrap().contents);
}