mod instance;
mod movement;

pub use instance::*;
pub use movement::*;

use crate::action::*;

use std::collections::HashMap;

/*pub trait Entity {
    fn movement_attrs<'a>(&'a self) -> &'a MovementAttributes;
}*/

pub struct Entity {
    move_attrs: MovementAttributes,
    actions: HashMap<ActionType, ActionAttributes>
}

impl Entity {
    pub fn new_pawn() -> Entity {
        let mut actions_map = HashMap::new();
        actions_map.insert(ActionType::AttackA, ActionAttributes{
            wind_up: 4,
            active: 2,
            wind_down: 2,
            action: |player, mobs| {
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

    pub fn movement_attrs<'a>(&'a self) -> &'a MovementAttributes {
        &self.move_attrs
    }
}
