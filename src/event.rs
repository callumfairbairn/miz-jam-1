use nannou::App;
use crate::Model;
use nannou::prelude::{WindowEvent, Key};
use nannou::event::WindowEvent::{KeyPressed, KeyReleased};

pub struct KeyDownStatus {
    pub w: bool,
    pub s: bool,
    pub a: bool,
    pub d: bool,
}

impl KeyDownStatus {
    pub fn new() -> KeyDownStatus {
        KeyDownStatus{ w: false, s: false, a: false, d: false}
    }
}

pub fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(key) => {
            update_key_down_status(key, model, true)
        }
        KeyReleased(key) => {
            update_key_down_status(key, model, false)
        }
        _ => {}
    }
}

fn update_key_down_status(key: Key, model: &mut Model, key_down: bool) {
    match key {
        Key::W => model.key_down_status.w = key_down,
        Key::S => model.key_down_status.s = key_down,
        Key::A => model.key_down_status.a = key_down,
        Key::D => model.key_down_status.d = key_down,
        Key::P => std::process::exit(0),
        _ => {}
    }
}