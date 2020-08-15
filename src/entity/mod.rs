mod environment;
mod instance;
mod movement;

pub use environment::EnvironmentState;
pub use instance::*;
pub use movement::*;

pub trait Entity {
    fn movement_attrs<'a>(&'a self) -> &'a MovementAttributes;
}

pub struct PlayerEntity {
    move_attrs: MovementAttributes
}

impl PlayerEntity {
    pub fn new_pawn() -> PlayerEntity {
        Self {
            move_attrs: MovementAttributes {
                attack: 8.0,
                sustain: 10.0,
                release:  7.0
            }
        }
    }
}

impl Entity for PlayerEntity {
    fn movement_attrs<'a>(&'a self) -> &'a MovementAttributes {
        &self.move_attrs
    }
}