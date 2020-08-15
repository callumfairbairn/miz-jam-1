use super::tile::IPoint2;
use rand::Rng;
use crate::constants::{LEVEL_DIM};
use rand::rngs::ThreadRng;

pub struct Level {
    pub floor: Vec<Vec<Option<TileAttributes>>>,
}

pub struct TileAttributes {
    pub tile_coord: IPoint2,
    pub solid: bool
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

pub fn generate_level(suit: Suit) -> Level {
    let rng = rand::thread_rng();
    let tiles_per_row = LEVEL_DIM;
    let tiles_per_column = LEVEL_DIM;
    let mut grid = Vec::new();
    for y in 0..tiles_per_column {
        let mut row = Vec::new();
        for x in 0..tiles_per_row {
            if (x == 0 || y == 0) || (x == &tiles_per_row - 1 || y == &tiles_per_column - 1) {
                row.push(Some(TileAttributes { tile_coord: get_tile_coord(&suit.wall_tiles, rng), solid: true }))
            } else {
                row.push(Some(TileAttributes { tile_coord: get_tile_coord(&suit.floor_tiles, rng), solid: false }))
            }
        }
        grid.push(row);
    }
    Level{
        floor: grid,
    }
}

fn get_tile_coord(possible_tiles: &Vec<IPoint2>, mut rng: ThreadRng) -> IPoint2 {
    possible_tiles[rng.gen_range(0, possible_tiles.len())]
}
