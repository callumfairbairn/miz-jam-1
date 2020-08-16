mod level;
mod update;
mod event;
mod tile;
mod constants;

mod entity;
mod action;
mod environment;
mod rect;
mod animation;

use nannou::{
    prelude::*,
    image::open
};

use constants::{WINDOW_RES_X, WINDOW_RES_Y};
use tile::{Grid, Tile, from_internal_to_screen, from_internal_to_offset};
use event::event;
use update::update;
use level::{
    generate_level,
    generate_starting_position,
    hearts,
    Level
};
use entity::{
    Entity,
    EntityFactory,
    Instance
};
use environment::EnvironmentState;
use animation::{
    AnimationAction,
    AnimationState
};

pub struct Model {
    grid: Grid,
    level: Level,

    env: EnvironmentState,

    // graphics
    w_id: nannou::winit::window::WindowId,
    render_pipeline: wgpu::RenderPipeline,
    bind_group_0: wgpu::BindGroup,
    bind_group_layout_1: wgpu::BindGroupLayout,
}

impl Model {
    pub fn tick(&mut self) {
        self.env.player_tick(&self.level);

        // TODO: move the below into env.mob_tick
        let (active, mut dead): (Vec<Instance>, Vec<Instance>) = self.env.mobs.drain(..).partition(|mob| mob.state.is_active());
        self.env.mobs = active;
        
        for newly_dead in dead.iter_mut() {
            newly_dead.animations.push_back(AnimationState::new_opacity_change(1.0, 0.0, 100));
        }
        self.env.inactive.append(&mut dead);

        for mob in self.env.mobs.iter_mut() {
            // AI
            if let Some(a) = mob.animations.front_mut() {
                if a.tick() {
                    mob.animations.pop_front();
                }
            }
        }

        for inactive in self.env.inactive.iter_mut() {
            // AI
            if let Some(a) = inactive.animations.front_mut() {
                if a.tick() {
                    inactive.animations.pop_front();
                }
            }
        }
    }
}

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

fn model(app: &App) -> Model {
    let w_id = app.new_window().size(WINDOW_RES_X as u32, WINDOW_RES_Y as u32).event(event).view(view).build().unwrap();
    let window = app.window(w_id).unwrap();
    let device = window.swap_chain_device();

    // GRAPHICS
    let tile_image = open(app.assets_path().unwrap().join("tilesheet.png")).unwrap();
    let tile_tex = wgpu::Texture::from_image(app, &tile_image);

    let vs_mod = wgpu::shader_from_spirv_bytes(device, include_bytes!("shaders/vert.spv"));
    let fs_mod = wgpu::shader_from_spirv_bytes(device, include_bytes!("shaders/frag.spv"));

    let bind_group_layout_0 = wgpu::BindGroupLayoutBuilder::new()
        .sampled_texture(
            wgpu::ShaderStage::FRAGMENT,
            false,
            wgpu::TextureViewDimension::D2
        )
        .sampler(wgpu::ShaderStage::FRAGMENT)
        .build(device);
    let bind_group_0 = wgpu::BindGroupBuilder::new()
        .texture_view(&tile_tex.view().build())
        .sampler(&wgpu::SamplerBuilder::new()
            .mag_filter(wgpu::FilterMode::Nearest)
            .min_filter(wgpu::FilterMode::Nearest)
            .build(device)
        )
        .build(device, &bind_group_layout_0);
    let bind_group_layout_1 = wgpu::BindGroupLayoutBuilder::new()
        .uniform_buffer(wgpu::ShaderStage::VERTEX, false)
        .uniform_buffer(wgpu::ShaderStage::FRAGMENT, false)
        .build(device);

    let pipeline_layout = device.create_pipeline_layout(
        &wgpu::PipelineLayoutDescriptor{
            bind_group_layouts: &[&bind_group_layout_0, &bind_group_layout_1]
        }
    );
    let render_pipeline = wgpu::RenderPipelineBuilder::from_layout(&pipeline_layout, &vs_mod)
        .fragment_shader(&fs_mod)
        .color_format(Frame::TEXTURE_FORMAT)
        .add_vertex_buffer::<tile::Vertex>()
        .sample_count(window.msaa_samples())
        .primitive_topology(wgpu::PrimitiveTopology::TriangleList)
        .build(device);

    let level = generate_level(hearts());
    let grid = Grid::new_from_level(&level, &tile_tex.size(), device);
    let player_entity = EntityFactory::new(Entity::new_pawn());

    let start_pos = generate_starting_position(&level);
    let player = player_entity.spawn(start_pos, Tile::new(26, 7, &tile_tex.size()));

    let env = EnvironmentState::new(player, &level, &tile_tex.size());

    Model {
        grid,
        level,

        env,

        w_id,
        render_pipeline,
        bind_group_0,
        bind_group_layout_1,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let window = app.window(model.w_id).unwrap();
    let device = window.swap_chain_device();

    let mut encoder = frame.command_encoder();
    let mut render_pass = wgpu::RenderPassBuilder::new()
        .color_attachment(frame.texture_view(), |colour| colour.clear_color(wgpu::Color::BLACK))
        .begin(&mut encoder);

    render_pass.set_bind_group(0, &model.bind_group_0, &[]);
    render_pass.set_pipeline(&model.render_pipeline);

    let (s_x, s_y) = from_internal_to_screen(model.env.player.movement.x_pos(), model.env.player.movement.y_pos());

    // DRAW BACKGROUND
    let bind_group_1 = create_bind_group_1(device, &model.bind_group_layout_1,
        (-s_x, -s_y),
        wgpu::Color::WHITE
    );
    render_pass.set_bind_group(1, &bind_group_1, &[]);
    render_pass.set_vertex_buffers(0, &[(&model.grid.vertices, 0)]);
    render_pass.draw(0..model.grid.num_vertices, 0..1);

    // DRAW PLAYER
    let bind_group_1 = create_bind_group_1(device, &model.bind_group_layout_1,
        (0.0, 0.0),
        wgpu::Color::WHITE
    );
    render_pass.set_bind_group(1, &bind_group_1, &[]);
    render_pass.set_vertex_buffers(0, &[(&model.env.player.tile.make_buffer(device), 0)]);
    render_pass.draw(0..6, 0..1);

    // DRAW MOBS
    for mob in &model.env.mobs {
        let col = if let Some(a) = mob.animations.front() {
            match a.current_action {
                Some(AnimationAction::Colour(c)) => c,
                Some(AnimationAction::Opacity(a)) => wgpu::Color{r: 1.0, g: 1.0, b: 1.0, a},
                None => wgpu::Color::WHITE
            }
        } else {
            wgpu::Color::WHITE
        };

        let bind_group_1 = create_bind_group_1(device, &model.bind_group_layout_1,
            from_internal_to_offset(mob.movement.x_pos() - model.env.player.movement.x_pos(), mob.movement.y_pos() - model.env.player.movement.y_pos()),
            col
        );
        render_pass.set_bind_group(1, &bind_group_1, &[]);
        render_pass.set_vertex_buffers(0, &[(&mob.tile.make_buffer(device), 0)]);
        render_pass.draw(0..6, 0..1);
    }

    for mob in &model.env.inactive {
        let col = if let Some(a) = mob.animations.front() {
            match a.current_action {
                Some(AnimationAction::Colour(c)) => c,
                Some(AnimationAction::Opacity(a)) => wgpu::Color{r: 1.0, g: 1.0, b: 1.0, a},
                None => wgpu::Color::WHITE
            }
        } else {
            wgpu::Color::WHITE
        };

        let bind_group_1 = create_bind_group_1(device, &model.bind_group_layout_1,
            from_internal_to_offset(mob.movement.x_pos() - model.env.player.movement.x_pos(), mob.movement.y_pos() - model.env.player.movement.y_pos()),
            col
        );
        render_pass.set_bind_group(1, &bind_group_1, &[]);
        render_pass.set_vertex_buffers(0, &[(&mob.tile.make_buffer(device), 0)]);
        render_pass.draw(0..6, 0..1);
    }
}

fn create_bind_group_1(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, transform: (f32, f32), color: wgpu::Color) -> wgpu::BindGroup {
    let transform_buffer_data = [transform.0, transform.1];
    let transform_buffer_bytes: &[u8] = bytemuck::cast_slice(&transform_buffer_data);
    let transform_buffer = device.create_buffer_mapped(transform_buffer_bytes.len(), wgpu::BufferUsage::UNIFORM).fill_from_slice(transform_buffer_bytes);

    let color_buffer_data = [color.r as f32, color.g as f32, color.b as f32, color.a as f32];
    let color_buffer_bytes: &[u8] = bytemuck::cast_slice(&color_buffer_data);
    let color_buffer = device.create_buffer_mapped(color_buffer_bytes.len(), wgpu::BufferUsage::UNIFORM).fill_from_slice(color_buffer_bytes);

    wgpu::BindGroupBuilder::new()
        .buffer_bytes(&transform_buffer, 0..(transform_buffer_bytes.len() as u64))
        .buffer_bytes(&color_buffer, 0..(color_buffer_bytes.len() as u64))
        .build(device, layout)
}