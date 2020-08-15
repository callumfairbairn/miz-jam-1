use crate::{
    entity::{Instance, Direction},
    action::ActionType
};

pub struct EnvironmentState {
    pub player: Instance,
    pub mobs: Vec<Instance>,

    pub dirs:           Direction,
    pub player_action:  Option<ActionType>
}

impl EnvironmentState {
    pub fn new(player: Instance) -> Self {
        Self {
            player: player,
            mobs: Vec::new(),

            dirs: Default::default(),

            player_action: None,
        }
    }
}