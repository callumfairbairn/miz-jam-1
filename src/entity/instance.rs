use super::{Entity, PlayerEntity};
use super::environment::EnvironmentState;
use super::movement::MovementState;
use crate::tile::Tile;

pub trait Instance {
    fn tick(&mut self, env: &EnvironmentState);
}

pub struct PlayerInstance {
    class: PlayerEntity,

    pub movement: MovementState,

    pub tile: Tile,
}

impl PlayerInstance {
    pub fn new(tile: Tile) -> Self {
        Self {
            class: PlayerEntity::new_pawn(),

            movement: MovementState::new(0, 0),

            tile: tile
        }
    }
}

impl Instance for PlayerInstance {
    fn tick(&mut self, env: &EnvironmentState) {
        self.movement.tick(self.class.movement_attrs(), env.dirs);
    }
}