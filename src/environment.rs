use crate::{
    entity::*,
    action::ActionType,
    tile::Tile,
    level::{Level, generate_starting_position}
};

pub struct EnvironmentState {
    pub player: Instance,
    pub mobs: Vec<Instance>,
    pub inactive: Vec<Instance>,

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
            inactive: Vec::new(),

            dirs: Default::default(),

            player_action: None,
        }
    }

    pub fn player_tick(&mut self) {
        self.player.movement_tick(self.dirs);
        self.player.action_tick(std::mem::replace(&mut self.player_action, None), &mut self.mobs);
    }
}