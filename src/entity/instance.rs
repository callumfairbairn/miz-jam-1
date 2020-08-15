use super::{Entity, PlayerEntity};
use super::environment::EnvironmentState;
use super::movement::MovementState;
use crate::tile::{Tile, IPoint2, TileInfo};
use nannou::prelude::Point2;
use nannou::App;

pub trait Instance {
    fn tick(&mut self, env: &EnvironmentState);
}

pub struct PlayerInstance {
    class: PlayerEntity,

    pub movement: MovementState,

    pub tile: Tile,
}

impl PlayerInstance {
    pub fn new(tile_info: &mut TileInfo, app: &App) -> Self {
        Self {
            class: PlayerEntity::new_pawn(),

            movement: MovementState::new(0, 0),

            tile: Tile::new(IPoint2{ x: 26, y: 7 }, Point2::new(0.0, 0.0), tile_info, app)
        }
    }
}

impl Instance for PlayerInstance {
    fn tick(&mut self, env: &EnvironmentState) {
        self.movement.tick(self.class.movement_attrs(), env.dirs);
    }
}