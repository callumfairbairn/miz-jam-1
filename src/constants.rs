// Number of tiles per edge = WINDOW_RES / (TILE_RES * ZOOM))
pub(crate) const GAP_BETWEEN_TILES: f32 = 1.0;

pub(crate) const TILE_RES: f32 = 16.0;
pub(crate) const ZOOM: f32 = 1.0; // This must be a power of 2
pub(crate) const WINDOW_RES_X: f32 = 832.0; // This must be a multiple of TILE_RES
pub(crate) const WINDOW_RES_Y: f32 = 640.0; // This must be a multiple of TILE_RES

pub(crate) const CHUNK_SIZE: usize = 15;
pub(crate) const LAYOUT_DIM: usize = 5;
pub(crate) const CHUNK_NUM: usize = 8;
pub(crate) const EROSION_CHANCE: f64 = 0.25;
pub(crate) const EROSION_TIMES: i32 = 2;

pub(crate) const COLLISION_MULTIPLIER: f64 = 0.75;
