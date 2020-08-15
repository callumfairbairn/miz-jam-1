use super::{Entity, PlayerEntity};
use super::movement::MovementState;
use crate::{
    action::ActiveActionState,
    environment::EnvironmentState,
    tile::Tile
};

pub trait Instance {
    fn tick(&mut self, env: &mut EnvironmentState);
}

pub struct PlayerInstance {
    class: PlayerEntity,

    pub movement: MovementState,
    action: Option<ActiveActionState>,

    pub tile: Tile,
}

impl PlayerInstance {
    pub fn new(tile: Tile) -> Self {
        Self {
            class: PlayerEntity::new_pawn(),

            movement: MovementState::new(0, 0),
            action: None,

            tile: tile
        }
    }
}

impl Instance for PlayerInstance {
    fn tick(&mut self, env: &mut EnvironmentState) {
        self.movement.tick(self.class.movement_attrs(), env.dirs);

        // TODO: Check any incoming actions first...
        if let Some(action) = std::mem::replace(&mut env.player_action, None) {
            if self.action.as_ref().map(|a| a.cancel()).unwrap_or(true) {
                self.action = self.class.actions.get(&action).map(|attrs| ActiveActionState::new(attrs));
            }
        }

        if let Some(action) = &mut self.action {
            if action.tick(env) {
                self.action = None;
            }
        }
    }
}