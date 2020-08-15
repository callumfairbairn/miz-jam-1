use crate::tile::IPoint2;
use serde::Deserialize;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct Level {
    pub level: Vec<Vec<IPoint2>>
}

pub fn read_level_from_file<P: AsRef<Path>>(path: P) -> Result<Level, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let level = serde_json::from_reader(reader)?;
    Ok(level)
}
