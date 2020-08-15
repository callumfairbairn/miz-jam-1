// Number of tiles per edge = WINDOW_RES / (TILE_RES * ZOOM))
pub(crate) const GAP_BETWEEN_TILES: u32 = 1;

pub(crate) const TILE_RES: f32 = 16.0;
pub(crate) const ZOOM: f32 = 4.0; // This must be a power of 2
pub(crate) const WINDOW_RES_X: f32 = 832.0; // This must be a multiple of TILE_RES
pub(crate) const WINDOW_RES_Y: f32 = 640.0; // This must be a multiple of TILE_RES