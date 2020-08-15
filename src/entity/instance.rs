use super::{Entity, PlayerEntity};
use super::environment::EnvironmentState;
use super::movement::MovementState;

pub trait Instance {
    fn tick(&mut self, env: &EnvironmentState);
}

pub struct PlayerInstance {
    class: PlayerEntity,

    pub movement: MovementState,
}

impl PlayerInstance {
    pub fn new() -> Self {
        Self {
            class: PlayerEntity::new_pawn(),

            movement: MovementState::new(0, 0)
        }
    }
}

impl Instance for PlayerInstance {
    fn tick(&mut self, env: &EnvironmentState) {
        self.movement.tick(self.class.movement_attrs(), env.dirs);
    }
}