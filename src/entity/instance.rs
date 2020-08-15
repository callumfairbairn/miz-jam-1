use super::{Entity, PlayerEntity};
use super::environment::EnvironmentState;
use super::movement::MovementState;
use crate::tile::{Tile, IPoint2};

pub trait Instance {
    fn tick(&mut self, env: &EnvironmentState);
}

pub struct PlayerInstance {
    class: PlayerEntity,

    pub movement: MovementState,

    pub tile: Tile,
}

impl PlayerInstance {
    pub fn new() -> Self {
        Self {
            class: PlayerEntity::new_pawn(),

            movement: MovementState::new(0, 0),

            tile: Tile::new(26, 7)
        }
    }
}

impl Instance for PlayerInstance {
    fn tick(&mut self, env: &EnvironmentState) {
        self.movement.tick(self.class.movement_attrs(), env.dirs);
    }
}