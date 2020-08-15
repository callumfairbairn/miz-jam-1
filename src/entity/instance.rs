use super::{Entity};
use super::movement::{MovementState, Direction};
use crate::{
    action::ActiveActionState,
    action::ActionType,
    tile::Tile
};

pub struct InstanceState {
    pub movement: MovementState,

    // HP?
}

pub struct Instance {
    class: Entity,
    action: Option<ActiveActionState>,
    pub tile: Tile,

    pub state: InstanceState,
}

impl Instance {
    pub fn new(tile: Tile) -> Self {
        Self {
            class: Entity::new_pawn(),
            action: None,
            tile: tile,

            state: InstanceState {
                movement: MovementState::new(0, 0),
            }
        }
    }

    // TODO: grid and check
    pub fn movement_tick(&mut self, dirs: Direction) {
        self.state.movement.tick(self.class.movement_attrs(), dirs);
    }

    pub fn action_tick(&mut self, new_action: Option<ActionType>, mobs: &mut [Instance]) {
        // TODO: Check any incoming actions first...
        if let Some(action) = new_action {
            if self.action.as_ref().map(|a| a.cancel()).unwrap_or(true) {
                self.action = self.class.actions.get(&action).map(|attrs| ActiveActionState::new(attrs));
            }
        }

        if let Some(action) = &mut self.action {
            if action.tick(&mut self.state, mobs) {
                self.action = None;
            }
        }
    }
}

/*impl Instance for PlayerInstance {
    fn tick(&mut self, env: &mut EnvironmentState) {
        self.movement.tick(self.class.movement_attrs(), env.dirs);

        // TODO: Check any incoming actions first...
        if let Some(action) = std::mem::replace(&mut env.player_action, None) {
            if self.action.as_ref().map(|a| a.cancel()).unwrap_or(true) {
                self.action = self.class.actions.get(&action).map(|attrs| ActiveActionState::new(attrs));
            }
        }

        if let Some(action) = &mut self.action {
            if action.tick(self, env) {
                self.action = None;
            }
        }
    }
}*/