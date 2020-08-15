use super::{Entity, PlayerEntity};
use super::environment::EnvironmentState;
use super::movement::MovementState;
use crate::tile::Tile;
use crate::level::Level;
use rand::Rng;
use crate::constants::{TILE_RES, ZOOM, WINDOW_RES_X, WINDOW_RES_Y};

pub trait Instance {
    fn tick(&mut self, env: &EnvironmentState);
}

pub struct PlayerInstance {
    class: PlayerEntity,

    pub movement: MovementState,

    pub tile: Tile,
}

impl PlayerInstance {
    pub fn new(tile: Tile, level: &Level) -> Self {
        Self {
            class: PlayerEntity::new_pawn(),

            movement: MovementState::new(generate_starting_position(level)),

            tile: tile
        }
    }
}

fn generate_starting_position(level: &Level) -> (f64, f64) {
    let mut rng = rand::thread_rng();

    let mut x;
    let mut y;
    loop {
        x = rng.gen_range(0, level.floor.len());
        y = rng.gen_range(0, level.floor[x].len());
        if level.floor[x][y].is_some() && !level.floor[x][y].as_ref().unwrap().solid {
            let quad_size_x = TILE_RES * ZOOM;
            let quad_size_y = TILE_RES * ZOOM;

            let vertex_x_coord = quad_size_x * (x as f32) - (WINDOW_RES_X / 2.0);
            let vertex_y_coord = quad_size_y * (y as f32) - (WINDOW_RES_Y / 2.0);
            return (vertex_x_coord as f64, vertex_y_coord as f64)
        }
    }
}

impl Instance for PlayerInstance {
    fn tick(&mut self, env: &EnvironmentState) {
        self.movement.tick(self.class.movement_attrs(), env.dirs);
    }
}