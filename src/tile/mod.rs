mod grid;
mod tile;

use nannou::wgpu;

pub use grid::Grid;
pub use tile::Tile;

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

/*impl From<Vertex> for (Point3, Point2) {
    fn from(v: Vertex) -> Self {
        (v.point, v.tex_coords)
    }
}*/

#[derive(Eq, Ord, PartialOrd, PartialEq, Clone, Hash, Copy, Deserialize, Debug)]
pub struct IPoint2 {
    pub x: usize,
    pub y: usize
}
