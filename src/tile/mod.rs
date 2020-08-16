mod grid;
mod tile;

use nannou::wgpu;

pub use grid::Grid;
pub use tile::Tile;

use crate::constants::{TILE_RES, ZOOM, WINDOW_RES_X, WINDOW_RES_Y};

use serde::Deserialize;

#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct Vertex {
    point: [f32; 2],
    tex_coords: [f32; 2],
}

unsafe impl bytemuck::Zeroable for Vertex {}
unsafe impl bytemuck::Pod for Vertex {}

impl wgpu::VertexDescriptor for Vertex {
    const STRIDE: wgpu::BufferAddress = 16;
    const ATTRIBUTES: &'static [wgpu::VertexAttributeDescriptor] = &[
        wgpu::VertexAttributeDescriptor {
            format: wgpu::VertexFormat::Float2,
            offset: 0,
            shader_location: 0,
        },
        wgpu::VertexAttributeDescriptor {
            format: wgpu::VertexFormat::Float2,
            offset: 4 * 2,
            shader_location: 1,
        },
    ];
}

// Coords
pub fn from_internal_to_screen(internal_x: f32, internal_y: f32) -> (f32, f32) {
    const SCALE_X: f32 = 2.0 * TILE_RES * ZOOM / WINDOW_RES_X;
    const SCALE_Y: f32 = 2.0 * TILE_RES * ZOOM / WINDOW_RES_Y;
    (
        (internal_x * SCALE_X) - 1.0,
        (internal_y * SCALE_Y) - 1.0,
    )
}

pub fn from_internal_to_offset(internal_x: f32, internal_y: f32) -> (f32, f32) {
    const SCALE_X: f32 = 2.0 * TILE_RES * ZOOM / WINDOW_RES_X;
    const SCALE_Y: f32 = 2.0 * TILE_RES * ZOOM / WINDOW_RES_Y;
    (
        (internal_x * SCALE_X),
        (internal_y * SCALE_Y),
    )
}

#[derive(Eq, Ord, PartialOrd, PartialEq, Clone, Hash, Copy, Deserialize, Debug)]
pub struct IPoint2 {
    pub x: usize,
    pub y: usize
}
