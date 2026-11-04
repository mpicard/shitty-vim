use std::fs;
use std::io;

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn load(filename: &str) -> Result<Self, io::Error> {
        let contents = fs::read_to_string(filename)?;
        let lines = contents.lines().map(String::from).collect();
        Ok(Self { lines })
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}
