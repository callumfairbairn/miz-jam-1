use bitflags::bitflags;
use crate::level::Level;
use crate::constants::{WINDOW_RES_X, TILE_RES, ZOOM, WINDOW_RES_Y};

bitflags! {
    #[derive(Default)]
    pub struct Direction: u8 {
        const UP = 0b0001;
        const DOWN = 0b0010;
        const LEFT = 0b0100;
        const RIGHT = 0b1000;
    }
}
#[derive(Clone, Copy)]
enum YMove {
    None,
    Up,
    Down
}
#[derive(Clone, Copy)]
enum XMove {
    None,
    Left,
    Right
}

impl Direction {
    fn reduce(self) -> (XMove, YMove) {
        let y = match (self & (Direction::UP | Direction::DOWN)).bits() {
            0b00 | 0b11 => YMove::None,
            0b01 => YMove::Up,
            0b10 => YMove::Down,
            _ => unreachable!()
        };

        let x = match (self & (Direction::LEFT | Direction::RIGHT)).bits() >> 2 {
            0b00 | 0b11 => XMove::None,
            0b01 => XMove::Left,
            0b10 => XMove::Right,
            _ => unreachable!()
        };

        (x, y)
    }
}

// Defines the core attributes of an entity's movement.
pub struct MovementAttributes {
    pub attack: f64,  // 0->(sustain) in (attack) ticks
    pub sustain: f64, // (sustain) pixels per tick
    pub release: f64, // (sustain)->0 in (release) ticks
}

// Defines the state of a entity in space.
#[derive(Clone)]
pub struct MovementState {
    x: f64,
    y: f64,

    x_velo: f64,
    y_velo: f64,
}

impl MovementState {
    pub fn new((x, y): (f64, f64)) -> Self {
        Self {
            x: x as f64,
            y: y as f64,

            x_velo: 0.0,
            y_velo: 0.0
        }
    }

    pub fn tick(&mut self, attrs: &MovementAttributes, apply_direction: Direction, level: &Level) {
        let (x_move, y_move) = apply_direction.reduce();

        let (max_speed_x, max_speed_y) = match (x_move, y_move) {
            (XMove::None, YMove::None) => (0.0, 0.0),
            (_, YMove::None) => (attrs.sustain, 0.0),
            (XMove::None, _) => (0.0, attrs.sustain),
            (_, _) => {
                let max_speed = (attrs.sustain.powi(2) / 2.0).sqrt();
                (max_speed, max_speed)
            }
        };
        let decel = attrs.sustain / attrs.release;
        let accel = attrs.sustain / attrs.attack;

        let x_velo = match x_move {
            XMove::None => if self.x_velo > 0.0 {   // Release while travelling right
                let new_x_velo = self.x_velo - decel;
                if new_x_velo < 0.0 {0.0} else {new_x_velo}
            } else if self.x_velo < 0.0 {               // Release while travelling left
                let new_x_velo = self.x_velo + decel;
                if new_x_velo > 0.0 {0.0} else {new_x_velo}
            } else {
                0.0
            },
            XMove::Left => if self.x_velo > 0.0 {   // Release when travelling right
                self.x_velo - decel
            } else if self.x_velo > -max_speed_x {  // Accelerate left
                let new_x_velo = self.x_velo - accel;
                if new_x_velo < -max_speed_x {-max_speed_x} else {new_x_velo}
            } else if self.x_velo < -max_speed_x {  // Release when above max speed
                self.x_velo + accel
            } else {    // Maintain max speed
                -max_speed_x
            },
            XMove::Right => if self.x_velo < 0.0 {
                self.x_velo + decel
            } else if self.x_velo < max_speed_x {  // Accelerate right
                let new_x_velo = self.x_velo + accel;
                if new_x_velo > max_speed_x {max_speed_x} else {new_x_velo}
            } else if self.x_velo > max_speed_x {  // Release when above max speed
                self.x_velo - accel
            } else {
                max_speed_x
            },
        };

        let y_velo = match y_move {
            YMove::None => if self.y_velo > 0.0 {   // Release while travelling down
                let new_y_velo = self.y_velo - decel;
                if new_y_velo < 0.0 {0.0} else {new_y_velo}
            } else if self.y_velo < 0.0 {               // Release while travelling up
                let new_y_velo = self.y_velo + accel;
                if new_y_velo > 0.0 {0.0} else {new_y_velo}
            } else {
                0.0
            },
            YMove::Up => if self.y_velo < 0.0 {
                self.y_velo + decel
            } else if self.y_velo < max_speed_y {
                let new_y_velo = self.y_velo + accel;
                if new_y_velo > max_speed_y {max_speed_y} else {new_y_velo}
            } else if self.y_velo > max_speed_y {
                self.y_velo - accel
            } else {
                max_speed_y
            },
            YMove::Down => if self.y_velo > 0.0 {
                self.y_velo - decel
            } else if self.y_velo > -max_speed_y {
                let new_y_velo = self.y_velo - accel;
                if new_y_velo < -max_speed_y {-max_speed_y} else {new_y_velo}
            } else if self.y_velo < -max_speed_y {
                self.y_velo + accel
            } else {
                -max_speed_y
            },
        };

        let new_x = self.x + x_velo;
        let new_y = self.y + y_velo;
        let new_x_velo = x_velo;
        let new_y_velo = y_velo;

        let player_rect = crate::rect::Rect{
            pos: (new_x as f32, new_y as f32),
            size: (32.0, 32.0)
        };

        let mut collision = false;

        for (y, row) in level.floor.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if tile.is_some() {
                    if tile.as_ref().unwrap().solid {
                        let quad_size = TILE_RES * ZOOM;
                        let vertex_x = quad_size * (x as f32) - (WINDOW_RES_X / 2.0);
                        let vertex_y = quad_size * (y as f32) - (WINDOW_RES_Y / 2.0);

                        let tile_rect = crate::rect::Rect {
                            pos: (vertex_x, vertex_y),
                            size: (quad_size, quad_size),
                        };
                        if player_rect.collides_with(&tile_rect) {
                            collision = true;
                        }
                    }
                }
            }
        }

        if !collision {
            self.x = new_x;
            self.y = new_y;
            self.x_velo = new_x_velo;
            self.y_velo = new_y_velo;
        } else {
            self.x_velo = 0.0;
            self.y_velo = 0.0;

        }
    }

    pub fn x_pos(&self) -> f32 {
        self.x as f32
    }

    pub fn y_pos(&self) -> f32 {
        self.y as f32
    }
}
