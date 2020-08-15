use crate::constants::{TILE_RES, ZOOM, WINDOW_RES_X, WINDOW_RES_Y};
use nannou::prelude::*;
use std::ops::{Index, IndexMut};
use crate::tile::{Tile, IPoint2};
use crate::{TileInfo};
use std::collections::HashMap;
use nannou::wgpu::Texture;
use crate::level::Level;
use crate::entity::PlayerInstance;

pub(crate) struct Grid(Vec<Vec<Option<Tile>>>);

impl Grid {
    pub fn new_from_level(level: Level, tile_info: &mut TileInfo, app: &App) -> Grid {
        let mut grid = Vec::new();

        for x in 0..level.level.len() {
            let mut row = Vec::new();
            for y in 0..level.level.len() {
                if level.level[y][x].is_some() {
                    row.push(Some(Tile::new(level.level[y][x].unwrap(), Point2::new(x as f32, y as f32), tile_info, app)))
                } else {
                    row.push(None)
                }
            }
            grid.push(row);
        }
        Grid(grid)
    }

    pub fn draw_background(&self, app: &App, frame: &Frame, coord_texture_map: &HashMap<IPoint2, Texture>, player: &PlayerInstance) {
        let tile_coords = self.unique_tile_coords_in_grid();
        let Grid(vec) = self;
        let view = Rect::from_x_y_w_h(
            player.movement.x_pos(),
            player.movement.y_pos(),
            WINDOW_RES_X,
            WINDOW_RES_Y
        );

        for tile_coord in tile_coords {
            let mut tiles_with_coord = vec![];
            for row in vec {
                for tile in row {
                    if tile.is_some() {
                        if tile_coord == tile.as_ref().unwrap().tile_coord && is_tile_in_view(&tile.as_ref().unwrap(), view) {
                            tiles_with_coord.push(tile.as_ref().unwrap().clone());
                        }
                    }
                }
            }
            Tile::draw_tiles(tiles_with_coord, app, frame, coord_texture_map, player);
        }
    }

    fn unique_tile_coords_in_grid(&self) -> Vec<IPoint2> {
        //ssc = tile sheet coord
        let mut sscs = vec![];
        let Grid(vec) = self;
        for row in vec {
            for tile in row {
                if tile.is_some() {
                    let ssc = &tile.as_ref().unwrap().tile_coord;
                    if !sscs.contains(ssc) {
                        sscs.push(ssc.clone());
                    }
                }
            }
        }
        sscs
    }
}

fn is_tile_in_view(tile: &Tile, view: Rect) -> bool {
    let x_loc = -WINDOW_RES_X/2.0 + (tile.location.x as f32 + 0.5 ) * TILE_RES * ZOOM;
    let y_loc = WINDOW_RES_Y/2.0 - (tile.location.y as f32 + 0.5) * TILE_RES * ZOOM;
    let buffer = TILE_RES + 10.0;
    if x_loc + buffer < view.left() || x_loc - buffer > view.right() || y_loc + buffer < view.bottom() || y_loc - buffer > view.top() {
        return false
    }
    true
}

impl Index<usize> for Grid {
    type Output = Vec<Option<Tile>>;
    fn index(&self, index: usize) -> &Vec<Option<Tile>> {
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