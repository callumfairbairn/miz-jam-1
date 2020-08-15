use nannou::{
    App,
    prelude::{WindowEvent, Key}
};
use crate::{
    Model,
    entity::Direction,
    action::ActionType
};

pub fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        WindowEvent::KeyPressed(k) => match k {
            Key::W => model.env.dirs.insert(Direction::UP),
            Key::A => model.env.dirs.insert(Direction::LEFT),
            Key::S => model.env.dirs.insert(Direction::DOWN),
            Key::D => model.env.dirs.insert(Direction::RIGHT),
            Key::Space => model.env.player_action = Some(ActionType::AttackA),
            _ => ()
        },
        WindowEvent::KeyReleased(k) => match k {
            Key::W => model.env.dirs.remove(Direction::UP),
            Key::A => model.env.dirs.remove(Direction::LEFT),
            Key::S => model.env.dirs.remove(Direction::DOWN),
            Key::D => model.env.dirs.remove(Direction::RIGHT),
            _ => ()
        },
        _ => (),
    }
}
