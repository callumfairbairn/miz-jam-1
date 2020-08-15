use crate::constants::{TILE_RES, ZOOM, WINDOW_RES_X, WINDOW_RES_Y, GAP_BETWEEN_TILES};
use nannou::{
    prelude::*,
    geom::Tri
};
use crate::level::Level;

use super::Vertex;

#[derive(Clone)]
pub struct Grid {
    pub vertices: Vec<Tri<(Point3, Point2)>>
}

impl Grid {
    pub fn new_from_level(level: Level, texture_size: &[u32; 2]) -> Self {
        let mut grid = Vec::new();

        let quad_size_x = TILE_RES * ZOOM;
        let quad_size_y = TILE_RES * ZOOM;

        let tile_tex_size_x = TILE_RES / (texture_size[0] as f32);
        let tile_tex_size_y = TILE_RES / (texture_size[1] as f32);

        for (y, row) in level.floor.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {

                if let Some(tile_data) = tile {
                    let vertex_x_coord = quad_size_x * (x as f32) - (WINDOW_RES_X / 2.0);
                    let vertex_y_coord = quad_size_y * (y as f32) - (WINDOW_RES_Y / 2.0);

                    let tilesheet_x_pix_coord = (tile_data.tile_coord.x as f32) * (TILE_RES + GAP_BETWEEN_TILES);
                    let tilesheet_y_pix_coord = (tile_data.tile_coord.y as f32) * (TILE_RES + GAP_BETWEEN_TILES);

                    let tilesheet_x_tex_coord = tilesheet_x_pix_coord / (texture_size[0] as f32);
                    let tilesheet_y_tex_coord = tilesheet_y_pix_coord / (texture_size[1] as f32);

                    let tri_a = Tri([
                        Vertex{
                            point: Point3{x: vertex_x_coord, y: vertex_y_coord, z: 0.0},
                            tex_coords: Point2{x: tilesheet_x_tex_coord, y: tilesheet_y_tex_coord + tile_tex_size_y}
                        }.into(),
                        Vertex{
                            point: Point3{x: vertex_x_coord + quad_size_x, y: vertex_y_coord, z: 0.0},
                            tex_coords: Point2{x: tilesheet_x_tex_coord + tile_tex_size_x, y: tilesheet_y_tex_coord + tile_tex_size_y}
                        }.into(),
                        Vertex{
                            point: Point3{x: vertex_x_coord, y: vertex_y_coord + quad_size_y, z: 0.0},
                            tex_coords: Point2{x: tilesheet_x_tex_coord, y: tilesheet_y_tex_coord}
                        }.into()
                    ]);
                    let tri_b = Tri([
                        Vertex{
                            point: Point3{x: vertex_x_coord + quad_size_x, y: vertex_y_coord, z: 0.0},
                            tex_coords: Point2{x: tilesheet_x_tex_coord + tile_tex_size_x, y: tilesheet_y_tex_coord + tile_tex_size_y}
                        }.into(),
                        Vertex{
                            point: Point3{x: vertex_x_coord, y: vertex_y_coord + quad_size_y, z: 0.0},
                            tex_coords: Point2{x: tilesheet_x_tex_coord, y: tilesheet_y_tex_coord}
                        }.into(),
                        Vertex{
                            point: Point3{x: vertex_x_coord + quad_size_x, y: vertex_y_coord + quad_size_y, z: 0.0},
                            tex_coords: Point2{x: tilesheet_x_tex_coord + tile_tex_size_x, y: tilesheet_y_tex_coord}
                        }.into()
                    ]);
                    grid.push(tri_a);
                    grid.push(tri_b);
                }
                
            }
        }

        Self {
            vertices: grid
        }
    }
}
