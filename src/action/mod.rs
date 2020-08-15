pub mod attack;

use crate::{
    entity::{Instance, InstanceState}
};

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum ActionType {
    AttackA,
    AttackB,
    Consumable,
    SpecialMove
}

#[derive(Copy, Clone)]
enum ActionState {
    WindUp,
    Active,
    WindDown
}

#[derive(Clone)]
pub struct ActionAttributes {
    pub wind_up: u16,     // Num of ticks to wind up
    pub active: u16,      // Num of ticks active
    pub wind_down: u16,   // Num of ticks to wind down

    pub action: fn(&mut InstanceState, &mut [Instance])
}

pub struct ActiveActionState {
    state: ActionState,
    count: u16,   // Count to the next state

    attrs: ActionAttributes
}

impl ActiveActionState {
    pub fn new(attrs: &ActionAttributes) -> Self {
        Self {
            state: ActionState::WindUp,
            count: 0,

            attrs: attrs.clone(),
        }
    }

    // Returns true if it cancelled OK.
    pub fn cancel(&self) -> bool {
        match self.state {
            ActionState::WindUp => true,
            _ => false
        }
    }

    // Returns true if done.
    pub fn tick(&mut self, player: &mut InstanceState, mobs: &mut [Instance]) -> bool {
        self.count += 1;
        self.state = match self.state {
            ActionState::WindUp if self.count > self.attrs.wind_up => {
                self.count = 0;
                ActionState::Active
            },
            ActionState::Active if self.count > self.attrs.wind_up => {
                self.count = 0;
                (self.attrs.action)(player, mobs);
                ActionState::WindDown
            },
            ActionState::WindDown if self.count > self.attrs.wind_up => return true,
            other => other
        };

        false
    }
}