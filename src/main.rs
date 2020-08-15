mod level;
mod update;
mod event;
mod tile;
mod constants;

mod entity;

use nannou::{
    prelude::*,
    image::open
};

use constants::{WINDOW_RES_X, WINDOW_RES_Y};
use tile::{Grid, Tile};
use event::event;
use update::update;
use level::{generate_level, hearts};
use entity::{
    PlayerInstance,
    Instance,
    EnvironmentState
};

pub struct Model {
    grid: Grid,
    tile_tex: nannou::wgpu::Texture,

    player: PlayerInstance,
    env: EnvironmentState,
}

impl Model {
    pub fn tick(&mut self) {
        self.player.tick(&self.env);
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
    let grid = Grid::new_from_level(level, &tile_tex.size());
    let player = PlayerInstance::new(Tile::new(26, 7, &tile_tex.size()));

    let env = EnvironmentState::new();

    Model {
        grid,
        tile_tex,

        player,
        env
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    // Draw background...
    draw.sampler(nannou::wgpu::SamplerDescriptor{
        address_mode_u: nannou::wgpu::AddressMode::Repeat,
        address_mode_v: nannou::wgpu::AddressMode::Repeat,
        address_mode_w: nannou::wgpu::AddressMode::Repeat,
        mag_filter: nannou::wgpu::FilterMode::Nearest,
        min_filter: nannou::wgpu::FilterMode::Nearest,
        mipmap_filter: nannou::wgpu::FilterMode::Nearest,
        lod_min_clamp: 1.0,
        lod_max_clamp: 1.0,
        compare_function: nannou::wgpu::CompareFunction::Never,
    }).translate(nannou::geom::Vector3::new(-model.player.movement.x_pos(), -model.player.movement.y_pos(), 0.0))
        .mesh().tris_textured(&model.tile_tex, model.grid.vertices.clone());

    // Draw player...
    draw.sampler(nannou::wgpu::SamplerDescriptor{
        address_mode_u: nannou::wgpu::AddressMode::Repeat,
        address_mode_v: nannou::wgpu::AddressMode::Repeat,
        address_mode_w: nannou::wgpu::AddressMode::Repeat,
        mag_filter: nannou::wgpu::FilterMode::Nearest,
        min_filter: nannou::wgpu::FilterMode::Nearest,
        mipmap_filter: nannou::wgpu::FilterMode::Nearest,
        lod_min_clamp: 1.0,
        lod_max_clamp: 1.0,
        compare_function: nannou::wgpu::CompareFunction::Never,
    }).mesh().tris_textured(&model.tile_tex, model.player.tile.vertices.clone());

    // Finish
    draw.to_frame(app, &frame).unwrap();
}