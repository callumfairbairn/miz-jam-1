use crate::{
    entity::*,
    action::ActionType,
    tile::Tile,
    level::{Level, generate_starting_position}
};

pub struct EnvironmentState {
    pub player: Instance,
    pub mobs: Vec<Instance>,

    pub dirs:           Direction,
    pub player_action:  Option<ActionType>
}

impl EnvironmentState {
    pub fn new(player: Instance, level: &Level, size: &[u32; 2]) -> Self {
        // FOR DEMO PURPOSES:
        let card2_spawner = EntityFactory::new(Entity::card_2());
        let card2_instance = card2_spawner.spawn(generate_starting_position(level), Tile::new(21, 16, size));

        Self {
            player: player,
            mobs: vec![card2_instance],

            dirs: Default::default(),

            player_action: None,
        }
    }
}