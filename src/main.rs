mod level;
mod update;
mod event;
mod tile;
mod constants;

mod entity;
mod action;
mod environment;
mod rect;

use nannou::{
    prelude::*,
    image::open
};

use constants::{WINDOW_RES_X, WINDOW_RES_Y};
use tile::{Grid, Tile};
use event::event;
use update::update;
use level::{
    generate_level,
    generate_starting_position,
    hearts
};
use entity::{
    Entity,
    EntityFactory
};
use environment::EnvironmentState;
use crate::level::Level;

pub struct Model {
    grid: Grid,
    tile_tex: nannou::wgpu::Texture,
    level: Level,

    env: EnvironmentState,
}

impl Model {
    pub fn tick(&mut self) {
        self.env.player.movement_tick(self.env.dirs, &self.level);

        self.env.player.action_tick(std::mem::replace(&mut self.env.player_action, None), &mut self.env.mobs);

        self.env.mobs = self.env.mobs.iter().filter(|mob| mob.state.current_hp > 0).cloned().collect::<Vec<_>>();
    }
}

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

fn model(app: &App) -> Model {
    app.new_window().size(WINDOW_RES_X as u32, WINDOW_RES_Y as u32).event(event).view(view).build().unwrap();

    let tile_image = open(app.assets_path().unwrap().join("tilesheet.png")).unwrap();
    let tile_tex = wgpu::Texture::from_image(app, &tile_image);

    let level = generate_level(hearts());
    let grid = Grid::new_from_level(&level, &tile_tex.size());
    let player_entity = EntityFactory::new(Entity::new_pawn());

    let start_pos = generate_starting_position(&level);
    let player = player_entity.spawn(start_pos, Tile::new(26, 7, &tile_tex.size()));

    let env = EnvironmentState::new(player, &level, &tile_tex.size());

    Model {
        grid,
        tile_tex,
        level,

        env
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    // Draw background...
    draw.sampler(sampler_desc())
        .translate(nannou::geom::Vector3::new(-model.env.player.movement.x_pos(), -model.env.player.movement.y_pos(), 0.0))
        .mesh().tris_textured(&model.tile_tex, model.grid.vertices.clone());

    // Draw player...
    draw.sampler(sampler_desc())
        .mesh().tris_textured(&model.tile_tex, model.env.player.tile.vertices.clone());

    // Draw mobs...
    for mob in &model.env.mobs {
        draw.sampler(sampler_desc())
        .translate(nannou::geom::Vector3::new(mob.movement.x_pos() - model.env.player.movement.x_pos(), mob.movement.y_pos() - model.env.player.movement.y_pos(), 0.0))
        .mesh().tris_textured(&model.tile_tex, mob.tile.vertices.clone());
    }

    // Finish
    draw.to_frame(app, &frame).unwrap();
}

const fn sampler_desc() -> nannou::wgpu::SamplerDescriptor {
    nannou::wgpu::SamplerDescriptor{
        address_mode_u: nannou::wgpu::AddressMode::Repeat,
        address_mode_v: nannou::wgpu::AddressMode::Repeat,
        address_mode_w: nannou::wgpu::AddressMode::Repeat,
        mag_filter: nannou::wgpu::FilterMode::Nearest,
        min_filter: nannou::wgpu::FilterMode::Nearest,
        mipmap_filter: nannou::wgpu::FilterMode::Nearest,
        lod_min_clamp: 1.0,
        lod_max_clamp: 1.0,
        compare_function: nannou::wgpu::CompareFunction::Never,
    }
}