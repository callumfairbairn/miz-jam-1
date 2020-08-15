use crate::constants::{TILE_RES, ZOOM, WINDOW_RES_X, WINDOW_RES_Y, GAP_BETWEEN_TILES};
use nannou::image::imageops::{FilterType};
use nannou::image::{DynamicImage};
use nannou::prelude::{Point2, wgpu};
use nannou::{App, Frame};
use nannou::wgpu::Texture;
use std::collections::HashMap;
use serde::Deserialize;

pub struct TileInfo {
    pub coord_texture_map: HashMap<IPoint2, Texture>,
    pub tile_sheet: DynamicImage
}

#[derive(Eq, Ord, PartialOrd, PartialEq, Clone, Hash, Copy, Deserialize, Debug)]
pub struct IPoint2 {
    pub x: i32,
    pub y: i32
}

#[derive(Clone)]
pub struct Tile {
    pub tile_coord: IPoint2,
    pub location: Point2
}

impl Tile {
    pub fn new(tile_coord: IPoint2, game_location: Point2, tile_info: &mut TileInfo, app: &App) -> Tile {
        // The first time we use a tile that we haven't used before, add its texture to coord_texture_map.
        // This is so that we only have to load it once, as wgpu::Texture::from_image() is expensive
        add_texture_to_map_if_needed(tile_coord, tile_info, app); // If we have already loaded a texture, this function does nothing
        return Tile {
            tile_coord: tile_coord.clone(),
            location: game_location
        }
    }

    pub fn draw_tile(&self, app: &App, frame: &Frame, coord_texture_map: &HashMap<IPoint2, Texture>) {
        let draw = app.draw();
        draw.texture(get_texture(&self.tile_coord, coord_texture_map))
            .x_y(-WINDOW_RES_X/2.0 + ((self.location.x + 0.5 ) * TILE_RES * ZOOM), WINDOW_RES_Y/2.0 - ((self.location.y + 0.5) * TILE_RES * ZOOM) );
        draw.to_frame(app, frame).unwrap();
    }

    pub fn draw_tiles(tiles: Vec<Tile>, app: &App, frame: &Frame, coord_texture_map: &HashMap<IPoint2, Texture>) {
        let draw = app.draw();
        for tile in tiles {
            draw.texture(get_texture(&tile.tile_coord, coord_texture_map))
                .x_y(-WINDOW_RES_X/2.0 + ((tile.location.x as f32 + 0.5 ) * TILE_RES * ZOOM), WINDOW_RES_Y/2.0 - ((tile.location.y as f32 + 0.5) * TILE_RES * ZOOM) );
        }
        draw.to_frame(app, frame).unwrap();
    }
}

fn new_texture_from_coord(coord: IPoint2, tile_sheet: &DynamicImage, app: &App) -> Texture {
    wgpu::Texture::from_image(
        app,
        &tile_sheet.crop_imm(coord.x as u32 * (TILE_RES as u32 + GAP_BETWEEN_TILES), coord.y as u32 * (TILE_RES as u32 + GAP_BETWEEN_TILES), TILE_RES as u32, TILE_RES as u32)
            .resize( (TILE_RES * ZOOM) as u32, (TILE_RES * ZOOM) as u32, FilterType::Nearest)
    )
}

fn get_texture<'a>(coord: &'a IPoint2, coord_texture_map: &'a HashMap<IPoint2, Texture>) -> &'a Texture {
    let texture = coord_texture_map.get(coord);
    return if texture.is_some() { texture.unwrap() } else { panic!("Texture not found in model.coord_texture_map") }
}

fn add_texture_to_map_if_needed(coord: IPoint2, tile_info: &mut TileInfo, app: &App) {
    let texture = tile_info.coord_texture_map.get(&coord);
    if texture.is_none() { tile_info.coord_texture_map.insert(coord, new_texture_from_coord(coord, &tile_info.tile_sheet, app)); }
}
