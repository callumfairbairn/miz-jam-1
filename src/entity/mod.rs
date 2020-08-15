mod instance;
mod movement;

pub use instance::*;
pub use movement::*;

use crate::action::*;

use std::collections::HashMap;

pub trait Entity {
    fn movement_attrs<'a>(&'a self) -> &'a MovementAttributes;
}

pub struct PlayerEntity {
    move_attrs: MovementAttributes,
    actions: HashMap<ActionType, ActionAttributes>
}

impl PlayerEntity {
    pub fn new_pawn() -> PlayerEntity {
        let mut actions_map = HashMap::new();
        actions_map.insert(ActionType::AttackA, ActionAttributes{
            wind_up: 8,
            active: 10,
            wind_down: 6,
            action: |env| {
                println!("Action triggered!");
            }
        });

        Self {
            move_attrs: MovementAttributes {
                attack: 8.0,
                sustain: 10.0,
                release:  7.0
            },

            actions: actions_map,
        }
    }
}

impl Entity for PlayerEntity {
    fn movement_attrs<'a>(&'a self) -> &'a MovementAttributes {
        &self.move_attrs
    }
}