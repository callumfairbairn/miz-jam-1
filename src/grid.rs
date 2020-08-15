use crate::constants::{TILE_RES, ZOOM, WINDOW_RES_X, WINDOW_RES_Y};
use nannou::prelude::*;
use std::ops::{Index, IndexMut};
use crate::tile::{Tile, IPoint2};
use crate::{TileInfo};
use std::collections::HashMap;
use nannou::wgpu::Texture;
use crate::level::Level;

pub(crate) struct Grid(Vec<Vec<Tile>>);

impl Grid {
    pub fn _new_from_tile(tile_coord: IPoint2, tile_info: &mut TileInfo, app: &App) -> Grid {
        let tiles_per_row = (WINDOW_RES_X / (TILE_RES * ZOOM)) as usize;
        let tiles_per_column = (WINDOW_RES_Y / (TILE_RES * ZOOM)) as usize;
        let mut grid = Vec::new();
        for x in 0..tiles_per_row {
            let mut row = Vec::new();
            for y in 0..tiles_per_column {
                row.push(Tile::new(tile_coord, Point2::new(x as f32, y as f32), tile_info, app))
            }
            grid.push(row);
        }
        Grid(grid)
    }

    pub fn new_from_level(level: Level, tile_info: &mut TileInfo, app: &App) -> Grid {
        let tiles_per_row = (WINDOW_RES_X / (TILE_RES * ZOOM)) as usize;
        let tiles_per_column = (WINDOW_RES_Y / (TILE_RES * ZOOM)) as usize;
        let mut grid = Vec::new();

        if tiles_per_row != level.level[0].len() {
            println!("Number of rows in level: {}, should be {}", level.level[0].len(), tiles_per_row);
            panic!("Level has incorrect dimensions, closing...")
        }

        if tiles_per_column != level.level.len() {
            println!("Number of columns in level: {}, should be {}", level.level.len(), tiles_per_column);
            panic!("Level has incorrect dimensions, closing...")
        }

        for x in 0..tiles_per_row {
            let mut row = Vec::new();
            for y in 0..tiles_per_column {
                row.push(Tile::new(level.level[y][x], Point2::new(x as f32, y as f32), tile_info, app))
            }
            grid.push(row);
        }
        Grid(grid)
    }

    pub fn len(&self) -> usize {
        let Grid(vec) = self;
        vec.len()
    }

    pub fn draw_background(&self, app: &App, frame: &Frame, coord_texture_map: &HashMap<IPoint2, Texture>) {
        let tile_coords = self.unique_tile_coords_in_grid();
        let Grid(vec) = self;

        for tile_coord in tile_coords {
            let mut tiles_with_coord = vec![];
            for row in vec {
                for tile in row {
                    if tile_coord == tile.tile_coord {
                        tiles_with_coord.push(tile.clone());
                    }
                }
            }
            Tile::draw_tiles(tiles_with_coord, app, frame, coord_texture_map);
        }
    }

    fn unique_tile_coords_in_grid(&self) -> Vec<IPoint2> {
        //ssc = tile sheet coord
        let mut sscs = vec![];
        let Grid(vec) = self;
        for row in vec {
            for tile in row {
                let ssc = &tile.tile_coord;
                if !sscs.contains(ssc) {
                    sscs.push(ssc.clone());
                }
            }
        }
        sscs
    }

    // Replaces tile in grid that has the same location as the one provided
    pub fn _add_tile(&mut self, tile: Tile) {
        self[tile.location.x as usize][tile.location.y as usize] = tile.clone();
    }

    pub fn _add_tiles(&mut self, tiles: Vec<Tile>) {
        for tile in tiles {
            self[tile.location.x as usize][tile.location.y as usize] = tile.clone();
        }
    }
}



impl Index<usize> for Grid {
    type Output = Vec<Tile>;
    fn index(&self, index: usize) -> &Vec<Tile> {
        let Grid(vec) = self;
        &vec[index]
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let Grid(vec) = self;
        &mut vec[index]
    }
}