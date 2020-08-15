use super::{Entity};
use super::movement::{MovementState, Direction};
use crate::{
    action::ActiveActionState,
    action::ActionType,
    tile::Tile
};

use std::rc::Rc;

#[derive(Clone)]
pub struct InstanceAttributes {
    // HP?
}

pub struct Instance {
    class: Rc<Entity>,
    action: Option<ActiveActionState>,
    pub tile: Tile,
    pub movement: MovementState,

    pub state: InstanceAttributes,
}

impl Instance {
    pub fn new(from_entity: &Rc<Entity>, at_coords: (f64, f64), tile: Tile) -> Self {
        Self {
            class: from_entity.clone(),
            action: None,
            tile: tile,
            movement: MovementState::new(at_coords),

            state: from_entity.initial_state.clone()
        }
    }

    // TODO: grid and check
    pub fn movement_tick(&mut self, dirs: Direction) {
        self.movement.tick(self.class.movement_attrs(), dirs);
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
