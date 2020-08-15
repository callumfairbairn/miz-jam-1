mod level;
mod update;
mod event;
mod tile;
mod grid;
mod constants;

mod entity;

use nannou::{
    prelude::*,
    image::open
};

use std::collections::HashMap;

use constants::{WINDOW_RES_X, WINDOW_RES_Y};
use grid::Grid;
use tile::{Tile, IPoint2, TileInfo};
use event::event;
use update::update;
use level::{generate_level, hearts};
use entity::{
    PlayerInstance,
    Instance,
    EnvironmentState
};

pub struct Model {
    grid: Grid,
    tile_info: TileInfo,

    player: PlayerInstance,
    env: EnvironmentState,
}

impl Model {
    pub fn tick(&mut self) {
        self.player.tick(&self.env);
    }
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

    let level = generate_level(hearts());
    let grid = Grid::new_from_level(level, &mut tile_info, app);
    // let grid = Grid::_new_from_tile(IPoint2{x: 5, y: 0}, &mut tile_info, app);
    let player = PlayerInstance::new(&mut tile_info, app);

    let env = EnvironmentState::new();

    Model {
        grid,
        tile_info,

        player,
        env
    }
}



fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);
    model.grid.draw_background(app, &frame, &model.tile_info.coord_texture_map, &model.player);
    Tile::draw_player(app, &frame, &model.tile_info.coord_texture_map, &model.player)
}