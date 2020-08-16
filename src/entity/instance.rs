use super::{Entity};
use super::movement::{MovementState, Direction};
use crate::{
    action::ActiveActionState,
    action::ActionType,
    animation::AnimationState,
    tile::Tile
};

use std::rc::Rc;
use std::collections::VecDeque;
use crate::level::Level;
use crate::entity::random_direction;
use crate::constants::{AI_IDLE_WAIT_TIME, AI_IDLE_MOVEMENT_TIME};

pub struct InstanceState<'a> {
    pub pos: (f32, f32),
    pub attrs: &'a mut InstanceAttributes
}

#[derive(Clone)]
pub struct InstanceAttributes {
    max_hp: isize,
    current_hp: isize,

    is_active: bool,

    tick_tracker: i32,
    direction: Direction,
}

impl InstanceAttributes {
    pub fn new(hp: isize) -> Self {
        Self {
            max_hp: hp,
            current_hp: hp,

            is_active: true,

            tick_tracker: 0,
            direction: Default::default(),
        }
    }

    pub fn modify_hp(&mut self, delta: isize) {
        self.current_hp += delta;
        if self.current_hp <= 0 {
            self.is_active = false;
        }
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn increment_tick_tracker(&mut self) {
        self.tick_tracker += 1;
    }

    pub fn reset_tick_tracker(&mut self) {
        self.tick_tracker = 0;
    }

    pub fn set_direction(&mut self, dirs: Direction) {
        self.direction = dirs;
    }
}

pub struct Instance {
    class: Rc<Entity>,
    action: Option<ActiveActionState>,
    pub tile: Tile,
    pub movement: MovementState,
    pub animations: VecDeque<AnimationState>,

    pub state: InstanceAttributes,
}

impl Instance {
    pub fn new(from_entity: &Rc<Entity>, at_coords: (f64, f64), tile: Tile) -> Self {
        Self {
            class: from_entity.clone(),
            action: None,
            tile: tile,
            movement: MovementState::new(at_coords),
            animations: VecDeque::new(),

            state: from_entity.initial_state.clone()
        }
    }

    // TODO: grid and check
    pub fn movement_tick(&mut self, dirs: Direction, level: &Level) {
        if self.state.is_active() {
            self.movement.tick(self.class.movement_attrs(), dirs, level);
        }
    }

    pub fn action_tick(&mut self, new_action: Option<ActionType>, mobs: &mut [Instance]) {
        if self.state.is_active() {
            if let Some(action) = new_action {
                if self.action.as_ref().map(|a| a.cancel()).unwrap_or(true) {
                    self.action = self.class.actions.get(&action).map(|attrs| ActiveActionState::new(attrs));
                }
            }
    
            if let Some(action) = &mut self.action {
                let mut state = InstanceState {
                    pos: (self.movement.x_pos(), self.movement.y_pos()),
                    attrs: &mut self.state
                };
                if action.tick(&mut state, mobs) {
                    self.action = None;
                }
            }
        }
    }

    fn ai_idle(&mut self) {
        if self.state.tick_tracker >= AI_IDLE_WAIT_TIME {
            if self.state.tick_tracker == AI_IDLE_WAIT_TIME {
                self.state.set_direction(random_direction());
            }

            if self.state.tick_tracker > AI_IDLE_WAIT_TIME + AI_IDLE_MOVEMENT_TIME {
                self.state.reset_tick_tracker();
                self.state.set_direction(Default::default());
            }
        }
    }


    pub fn ai_tick(&mut self, level: &Level) {
        if self.state.is_active() {
            self.ai_idle();
            self.movement.tick(self.class.movement_attrs(), self.state.direction, level);
            self.state.increment_tick_tracker();
        }
    }
}
