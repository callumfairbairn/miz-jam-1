mod grid;
mod tile;

use nannou::prelude::{Point3, Point2};

pub use grid::Grid;
pub use tile::Tile;

use serde::Deserialize;

#[derive(Clone)]
struct Vertex {
    point: Point3,
    tex_coords: Point2,
}

impl From<Vertex> for (Point3, Point2) {
    fn from(v: Vertex) -> Self {
        (v.point, v.tex_coords)
    }
}

#[derive(Eq, Ord, PartialOrd, PartialEq, Clone, Hash, Copy, Deserialize, Debug)]
pub struct IPoint2 {
    pub x: usize,
    pub y: usize
}
