use nannou::App;
use crate::Model;
use nannou::prelude::Update;

pub fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.key_down_status.w {
        if model.player.location.y > 0.0 { model.player.location.y = model.player.location.y - 1.0 }
    }
    if model.key_down_status.s {
        if model.player.location.y < model.grid[0].len() as f32 - 1.0 { model.player.location.y = model.player.location.y + 1.0 }
    }
    if model.key_down_status.a {
        if model.player.location.x > 0.0 { model.player.location.x = model.player.location.x - 1.0 }
    }
    if model.key_down_status.d {
        if model.player.location.x < model.grid.len() as f32 - 1.0 { model.player.location.x = model.player.location.x + 1.0  }
    }
}