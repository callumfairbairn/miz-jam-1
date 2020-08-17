mod instance;
mod movement;

pub use instance::*;
pub use movement::*;

use crate::{
    action::*,
    tile::Tile
};

use std::collections::HashMap;
use std::rc::Rc;

pub struct EntityFactory(Rc<Entity>);

impl EntityFactory {
    pub fn new(entity: Entity) -> Self {
        Self(Rc::new(entity))
    }

    pub fn spawn(&self, at_coords: (f64, f64), with_tile: Tile) -> Instance {
        Instance::new(&self.0, at_coords, with_tile)
    }
}


pub struct Entity {
    move_attrs: MovementAttributes,
    actions: HashMap<ActionType, ActionAttributes>,
    initial_state: InstanceAttributes
}

impl Entity {
    pub fn new_pawn() -> Entity {
        let mut actions_map = HashMap::new();
        actions_map.insert(ActionType::AttackA, ActionAttributes{
            wind_up: 4,
            active: 2,
            wind_down: 2,
            action: crate::action::attack::quick_attack
        });

        Self {
            move_attrs: MovementAttributes {
                attack: 8.0,
                sustain: 10.0 / 32.0,
                release:  7.0
            },

            actions: actions_map,

            initial_state: InstanceAttributes::new(10)
        }
    }

    pub fn card_2() -> Entity {
        Self {
            move_attrs: MovementAttributes {
                attack: 3.0,
                sustain: 5.0 / 32.0,
                release:  7.0
            },

            actions: HashMap::new(),

            initial_state: InstanceAttributes::new(3)
        }
    }
}

impl Entity {
    pub fn movement_attrs<'a>(&'a self) -> &'a MovementAttributes {
        &self.move_attrs
    }
}
