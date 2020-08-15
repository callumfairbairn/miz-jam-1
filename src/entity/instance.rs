use super::{Entity};
use super::movement::{MovementState, Direction};
use crate::{
    action::ActiveActionState,
    action::ActionType,
    tile::Tile
};

use std::rc::Rc;

pub struct InstanceState {
    pub movement: MovementState,

    // HP?
}

pub struct Instance {
    class: Rc<Entity>,
    action: Option<ActiveActionState>,
    pub tile: Tile,

    pub state: InstanceState,
}

impl Instance {
    pub fn new(from_entity: Rc<Entity>, at_coords: (usize, usize), tile: Tile) -> Self {
        Self {
            class: from_entity,
            action: None,
            tile: tile,

            state: InstanceState {
                movement: MovementState::new(at_coords.0, at_coords.1),
            }
        }
    }

    // TODO: grid and check
    pub fn movement_tick(&mut self, dirs: Direction) {
        self.state.movement.tick(self.class.movement_attrs(), dirs);
    }

    pub fn action_tick(&mut self, new_action: Option<ActionType>, mobs: &mut [Instance]) {
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
