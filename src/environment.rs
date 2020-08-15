use crate::{
    entity::Direction,
    action::ActionType
};

pub struct EnvironmentState {
    pub dirs:           Direction,
    pub player_action:  Option<ActionType>
}

impl EnvironmentState {
    pub fn new() -> Self {
        Self {
            dirs: Default::default(),

            player_action: None,
        }
    }
}