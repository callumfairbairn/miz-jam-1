use crate::tile::IPoint2;
use serde::Deserialize;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::error::Error;
use rand::Rng;
use crate::constants::{LEVEL_DIM};
use rand::rngs::ThreadRng;

#[derive(Deserialize, Debug)]
pub struct Level {
    pub level: Vec<Vec<Option<IPoint2>>>
}

pub struct Suit {
    pub floor_tiles: Vec<IPoint2>,
    pub wall_tiles: Vec<IPoint2>,
}

pub fn hearts() -> Suit {
    Suit {
        floor_tiles: vec![IPoint2{x: 5, y: 0}, IPoint2{x: 6, y: 0}, IPoint2{x: 7, y: 0}],
        wall_tiles: vec![IPoint2{x: 0, y: 1}, IPoint2{x: 0, y: 2}, IPoint2{x: 1, y: 1}]
    }
}

pub fn _spades() -> Suit {
    Suit {
        floor_tiles: vec![IPoint2{x: 19, y: 1}],
        wall_tiles: vec![IPoint2{x: 10, y: 17}, IPoint2{x: 10, y: 18}, IPoint2{x: 11, y: 18}]
    }
}

pub fn _read_level_from_file<P: AsRef<Path>>(path: P) -> Result<Level, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let level = serde_json::from_reader(reader)?;
    Ok(level)
}

pub fn generate_level(suit: Suit) -> Level {
    let rng = rand::thread_rng();
    let tiles_per_row = LEVEL_DIM;
    let tiles_per_column = LEVEL_DIM;
    let mut grid = Vec::new();
    for y in 0..tiles_per_column {
        let mut row = Vec::new();
        for x in 0..tiles_per_row {
            if (x == 0 || y == 0) || (x == &tiles_per_row - 1 || y == &tiles_per_column - 1) {
                row.push(Some(get_tile(&suit.wall_tiles, rng)))
            } else {
                row.push(Some(get_tile(&suit.floor_tiles, rng)))
            }
        }
        grid.push(row);
    }
    Level{ level: grid }
}

fn get_tile(possible_tiles: &Vec<IPoint2>, mut rng: ThreadRng) -> IPoint2 {
    possible_tiles[rng.gen_range(0, possible_tiles.len())]
}
