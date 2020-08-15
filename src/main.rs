mod level;
mod update;
mod event;
mod tile;
mod grid;
mod constants;

use nannou::prelude::*;
use crate::constants::{WINDOW_RES_X, WINDOW_RES_Y};
use nannou::image::open;
use crate::grid::Grid;
use crate::tile::{Tile, IPoint2, TileInfo};
use crate::event::{event, KeyDownStatus};
use crate::update::update;
use std::collections::HashMap;
use crate::level::read_level_from_file;

pub struct Model {
    player: Tile,
    grid: Grid,
    key_down_status: KeyDownStatus,
    tile_info: TileInfo
}

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

fn model(app: &App) -> Model {
    app.new_window().size(WINDOW_RES_X as u32, WINDOW_RES_Y as u32).event(event).view(view).build().unwrap();

    let tile_sheet = open(app.assets_path().unwrap().join("tilesheet.png")).unwrap();
    let coord_texture_map = HashMap::new();
    let mut tile_info = TileInfo{ tile_sheet, coord_texture_map };
    let level = read_level_from_file("levels/lvl1.json").ok().unwrap();
    // let grid = Grid::new_from_level(level, &mut tile_info, app);
    let grid = Grid::_new_from_tile(IPoint2{x: 5, y: 0}, &mut tile_info, app);

    Model {
        player: Tile::new(IPoint2{x: 25, y: 7}, Point2::new(4.0, 4.0), &mut tile_info, app),
        grid,
        key_down_status: KeyDownStatus::new(),
        tile_info
    }
}



fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);
    model.grid.draw_background(app, &frame, &model.tile_info.coord_texture_map);
    model.player.draw_tile(app, &frame, &model.tile_info.coord_texture_map)
}