use crate::constants::{TILE_RES, ZOOM, WINDOW_RES_X, WINDOW_RES_Y, GAP_BETWEEN_TILES};
use nannou::image::imageops::{FilterType};
use nannou::image::{DynamicImage};
use nannou::prelude::{Point2, wgpu};
use nannou::{App, Frame};
use nannou::wgpu::Texture;
use std::collections::HashMap;
use serde::Deserialize;
use crate::entity::PlayerInstance;

#[derive(Eq, Ord, PartialOrd, PartialEq, Clone, Hash, Copy, Deserialize, Debug)]
pub struct IPoint2 {
    pub x: usize,
    pub y: usize
}

#[derive(Clone)]
pub struct Tile(pub IPoint2);

impl Tile {
    pub fn new(x: usize, y: usize) -> Tile {
        Tile(IPoint2{x: x, y: y})
    }
}