use nannou::wgpu;

// Number of tiles per edge = WINDOW_RES / (TILE_RES * ZOOM))
pub(crate) const GAP_BETWEEN_TILES: f32 = 1.0;

pub(crate) const TILE_RES: f32 = 16.0;
pub(crate) const ZOOM: f32 = 2.0; // This must be a power of 2
pub(crate) const WINDOW_RES_X: f32 = 832.0; // This must be a multiple of TILE_RES
pub(crate) const WINDOW_RES_Y: f32 = 640.0; // This must be a multiple of TILE_RES

pub(crate) const CHUNK_SIZE: usize = 15;
pub(crate) const LAYOUT_DIM: usize = 5;
pub(crate) const CHUNK_NUM: usize = 2;
pub(crate) const EROSION_CHANCE: f64 = 0.25;
pub(crate) const EROSION_TIMES: i32 = 2;

pub(crate) const COLLISION_MULTIPLIER: f64 = 0.5;

pub(crate) const AI_IDLE_WAIT_TIME: i32 = 40;
pub(crate) const AI_IDLE_MOVEMENT_TIME: i32 = 6;
pub(crate) const AI_CHASE_DISTANCE_MAX: f32 = 5.0;
pub(crate) const AI_ATTACK_DISTANCE: f32 = 1.0;

pub(crate) const HEARTS_BACKGROUND_COLOUR: wgpu::Color = wgpu::Color{
    r: 0.02,
    g: 0.0,
    b: 0.0,
    a: 0.0
};

pub(crate) const DIAMONDS_BACKGROUND_COLOUR: wgpu::Color = wgpu::Color{
    r: 0.02,
    g: 0.0,
    b: 0.02,
    a: 0.0
};

pub(crate) const CLUBS_BACKGROUND_COLOUR: wgpu::Color = wgpu::Color{
    r: 0.0,
    g: 0.0,
    b: 0.02,
    a: 0.0
};

pub(crate) const SPADES_BACKGROUND_COLOUR: wgpu::Color = wgpu::Color::BLACK;

