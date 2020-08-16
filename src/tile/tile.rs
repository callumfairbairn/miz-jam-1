use crate::constants::*;
use super::Vertex;

#[derive(Clone)]
pub struct Tile {
    pub vertices: Vec<Vertex>
}

impl Tile {
    pub fn new(x: usize, y: usize, texture_size: &[u32; 2]) -> Self {
        let mut vertices = Vec::new();

        let quad_size_x = (2.0 * TILE_RES * ZOOM) / WINDOW_RES_X;
        let quad_size_y = (2.0 * TILE_RES * ZOOM) / WINDOW_RES_Y;

        let tile_tex_size_x = TILE_RES / (texture_size[0] as f32);
        let tile_tex_size_y = TILE_RES / (texture_size[1] as f32);

        let tilesheet_x_pix_coord = (x as f32) * (TILE_RES + GAP_BETWEEN_TILES);
        let tilesheet_y_pix_coord = (y as f32) * (TILE_RES + GAP_BETWEEN_TILES);

        let tilesheet_x_tex_coord = tilesheet_x_pix_coord / (texture_size[0] as f32);
        let tilesheet_y_tex_coord = tilesheet_y_pix_coord / (texture_size[1] as f32);

        vertices.push(Vertex{
            point: [0.0, 0.0],
            tex_coords: [tilesheet_x_tex_coord, tilesheet_y_tex_coord]
        });
        vertices.push(Vertex{
            point: [quad_size_x, 0.0],
            tex_coords: [tilesheet_x_tex_coord + tile_tex_size_x, tilesheet_y_tex_coord]
        });
        vertices.push(Vertex{
            point: [0.0, quad_size_y],
            tex_coords: [tilesheet_x_tex_coord, tilesheet_y_tex_coord + tile_tex_size_y]
        });
        vertices.push(Vertex{
            point: [quad_size_x, 0.0],
            tex_coords: [tilesheet_x_tex_coord + tile_tex_size_x, tilesheet_y_tex_coord]
        });
        vertices.push(Vertex{
            point: [0.0, quad_size_y],
            tex_coords: [tilesheet_x_tex_coord, tilesheet_y_tex_coord + tile_tex_size_y]
        });
        vertices.push(Vertex{
            point: [quad_size_x, quad_size_y],
            tex_coords: [tilesheet_x_tex_coord + tile_tex_size_x, tilesheet_y_tex_coord + tile_tex_size_y]
        });

        Self {
            vertices
        }
    }

    pub fn make_buffer(&self, device: &nannou::wgpu::Device) -> nannou::wgpu::Buffer {
        let vertices_bytes: &[u8] = bytemuck::cast_slice(&self.vertices);
        device.create_buffer_mapped(vertices_bytes.len(), nannou::wgpu::BufferUsage::VERTEX).fill_from_slice(vertices_bytes)
    }
}