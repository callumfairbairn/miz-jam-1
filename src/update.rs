use nannou::App;
use crate::Model;
use nannou::prelude::Update;

pub fn update(_app: &App, model: &mut Model, _update: Update) {
    model.tick();
}