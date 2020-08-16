use crate::constants::{TILE_RES, ZOOM, WINDOW_RES_X, WINDOW_RES_Y, GAP_BETWEEN_TILES};
use nannou::{
    prelude::*,
};
use crate::level::Level;

use super::Vertex;

pub struct Grid {
    pub num_vertices: u32,
    pub vertices: nannou::wgpu::Buffer
}

impl Grid {
    pub fn new_from_level(level: &Level, texture_size: &[u32; 2], device: &nannou::wgpu::Device) -> Self {
        let mut grid = Vec::new();
        let quad_size_x = (2.0 * TILE_RES * ZOOM) / WINDOW_RES_X;
        let quad_size_y = (2.0 * TILE_RES * ZOOM) / WINDOW_RES_Y;

        let tile_tex_size_x = TILE_RES / (texture_size[0] as f32);
        let tile_tex_size_y = TILE_RES / (texture_size[1] as f32);

        for (y, row) in level.floor.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {

                if let Some(tile_data) = tile {
                    let vertex_x_coord = quad_size_x * (x as f32) - 3.0;
                    let vertex_y_coord = quad_size_y * (y as f32) - 3.0;

                    let tilesheet_x_pix_coord = (tile_data.tile_coord.x as f32) * (TILE_RES + GAP_BETWEEN_TILES);
                    let tilesheet_y_pix_coord = (tile_data.tile_coord.y as f32) * (TILE_RES + GAP_BETWEEN_TILES);

                    let tilesheet_x_tex_coord = tilesheet_x_pix_coord / (texture_size[0] as f32);
                    let tilesheet_y_tex_coord = tilesheet_y_pix_coord / (texture_size[1] as f32);

                    grid.push(Vertex{
                        point: [vertex_x_coord, vertex_y_coord],
                        tex_coords: [tilesheet_x_tex_coord, tilesheet_y_tex_coord]
                    });
                    grid.push(Vertex{
                        point: [vertex_x_coord + quad_size_x, vertex_y_coord],
                        tex_coords: [tilesheet_x_tex_coord + tile_tex_size_x, tilesheet_y_tex_coord]
                    });
                    grid.push(Vertex{
                        point: [vertex_x_coord, vertex_y_coord + quad_size_y],
                        tex_coords: [tilesheet_x_tex_coord, tilesheet_y_tex_coord + tile_tex_size_y]
                    });
                    grid.push(Vertex{
                        point: [vertex_x_coord + quad_size_x, vertex_y_coord],
                        tex_coords: [tilesheet_x_tex_coord + tile_tex_size_x, tilesheet_y_tex_coord]
                    });
                    grid.push(Vertex{
                        point: [vertex_x_coord, vertex_y_coord + quad_size_y],
                        tex_coords: [tilesheet_x_tex_coord, tilesheet_y_tex_coord + tile_tex_size_y]
                    });
                    grid.push(Vertex{
                        point: [vertex_x_coord + quad_size_x, vertex_y_coord + quad_size_y],
                        tex_coords: [tilesheet_x_tex_coord + tile_tex_size_x, tilesheet_y_tex_coord + tile_tex_size_y]
                    });
                }
            }
        }

        /*grid.push(Vertex{
            point: [0.0, 0.0],
            tex_coords: [0.0, 0.0],
        });
        grid.push(Vertex{
            point: [1.0, 0.0],
            tex_coords: [1.0, 0.0],
        });
        grid.push(Vertex{
            point: [0.0, 1.0],
            tex_coords: [0.0, 1.0],
        });
        grid.push(Vertex{
            point: [1.0, 0.0],
            tex_coords: [1.0, 0.0],
        });
        grid.push(Vertex{
            point: [0.0, 1.0],
            tex_coords: [0.0, 1.0],
        });
        grid.push(Vertex{
            point: [1.0, 1.0],
            tex_coords: [1.0, 1.0],
        });*/

        let vertices_bytes: &[u8] = bytemuck::cast_slice(&grid);
        let vertex_buffer = device.create_buffer_mapped(vertices_bytes.len(), wgpu::BufferUsage::VERTEX).fill_from_slice(vertices_bytes);

        Self {
            num_vertices: grid.len() as u32,
            vertices: vertex_buffer
        }
    }
}
