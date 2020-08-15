use crate::constants::{TILE_RES, ZOOM, WINDOW_RES_X, WINDOW_RES_Y, GAP_BETWEEN_TILES};
use nannou::image::imageops::{FilterType};
use nannou::image::{DynamicImage};
use nannou::prelude::{Point3, Point2, wgpu};
use nannou::{App, Frame};
use nannou::wgpu::Texture;
use nannou::geom::Tri;
use std::collections::HashMap;

use super::Vertex;

#[derive(Clone)]
pub struct Tile {
    pub vertices: Vec<Tri<(Point3, Point2)>>
}

impl Tile {
    pub fn new(x: usize, y: usize, texture_size: &[u32; 2]) -> Self {
        let quad_size_x = TILE_RES * ZOOM;
        let quad_size_y = TILE_RES * ZOOM;

        let tile_tex_size_x = TILE_RES / (texture_size[0] as f32);
        let tile_tex_size_y = TILE_RES / (texture_size[1] as f32);

        let tilesheet_x_pix_coord = (x as f32) * (TILE_RES + GAP_BETWEEN_TILES);
        let tilesheet_y_pix_coord = (y as f32) * (TILE_RES + GAP_BETWEEN_TILES);

        let tilesheet_x_tex_coord = tilesheet_x_pix_coord / (texture_size[0] as f32);
        let tilesheet_y_tex_coord = tilesheet_y_pix_coord / (texture_size[1] as f32);

        let tri_a = Tri([
            Vertex{
                point: Point3{x: 0.0, y: 0.0, z: 0.0},
                tex_coords: Point2{x: tilesheet_x_tex_coord, y: tilesheet_y_tex_coord + tile_tex_size_y}
            }.into(),
            Vertex{
                point: Point3{x: quad_size_x, y: 0.0, z: 0.0},
                tex_coords: Point2{x: tilesheet_x_tex_coord + tile_tex_size_x, y: tilesheet_y_tex_coord + tile_tex_size_y}
            }.into(),
            Vertex{
                point: Point3{x: 0.0, y: quad_size_y, z: 0.0},
                tex_coords: Point2{x: tilesheet_x_tex_coord, y: tilesheet_y_tex_coord}
            }.into()
        ]);
        let tri_b = Tri([
            Vertex{
                point: Point3{x: quad_size_x, y: 0.0, z: 0.0},
                tex_coords: Point2{x: tilesheet_x_tex_coord + tile_tex_size_x, y: tilesheet_y_tex_coord + tile_tex_size_y}
            }.into(),
            Vertex{
                point: Point3{x: 0.0, y: quad_size_y, z: 0.0},
                tex_coords: Point2{x: tilesheet_x_tex_coord, y: tilesheet_y_tex_coord}
            }.into(),
            Vertex{
                point: Point3{x: quad_size_x, y: quad_size_y, z: 0.0},
                tex_coords: Point2{x: tilesheet_x_tex_coord + tile_tex_size_x, y: tilesheet_y_tex_coord}
            }.into()
        ]);

        Self {
            vertices: vec![tri_a, tri_b]
        }
    }
}