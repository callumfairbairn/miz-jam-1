use super::tile::IPoint2;
use rand::Rng;
use crate::constants::{CHUNK_SIZE, LAYOUT_DIM, CHUNK_NUM};
use rand::rngs::ThreadRng;

pub struct Level {
    pub floor: Vec<Vec<Option<TileAttributes>>>,
}

pub struct TileAttributes {
    pub tile_coord: IPoint2,
    pub solid: bool
}

pub struct Suit {
    pub floor_tiles: Vec<IPoint2>,
    pub wall_tiles: Vec<IPoint2>,
}

pub fn hearts() -> Suit {
    Suit {
        floor_tiles: vec![IPoint2{x: 5, y: 0}, IPoint2{x: 6, y: 0}, IPoint2{x: 7, y: 0}],
        wall_tiles: vec![IPoint2{x: 0, y: 1}, IPoint2{x: 0, y: 2}, IPoint2{x: 1, y: 1}]
    }
}

pub fn _spades() -> Suit {
    Suit {
        floor_tiles: vec![IPoint2{x: 19, y: 1}],
        wall_tiles: vec![IPoint2{x: 10, y: 17}, IPoint2{x: 10, y: 18}, IPoint2{x: 11, y: 18}]
    }
}

fn generate_floor_layout(mut rng: ThreadRng) -> Vec<Vec<bool>> {
    let mut layout = vec![vec![false; LAYOUT_DIM]; LAYOUT_DIM];

    //turn middle chunk on
    let centre_x = (layout.len()/2) as usize;
    let centre_y = (layout[0].len()/2) as usize;
    layout[centre_x][centre_y] = true;

    let mut number_of_chunks_turned_on = get_on_chunks(&layout).len();
    while number_of_chunks_turned_on < CHUNK_NUM {
        let on_chunks = get_on_chunks(&layout);

        let mut random_on_chunk = on_chunks[rng.gen_range(0, on_chunks.len())];
        let mut off_neighbors = get_off_neighbours(&layout, &random_on_chunk);
        while off_neighbors.len() == 0 {
            random_on_chunk = on_chunks[rng.gen_range(0, on_chunks.len())];
            off_neighbors = get_off_neighbours(&layout, &random_on_chunk);
        }

        let random_off_neighbor = off_neighbors[rng.gen_range(0, off_neighbors.len())];
        layout[random_off_neighbor.x][random_off_neighbor.y] = true;
        number_of_chunks_turned_on = get_on_chunks(&layout).len();
    }

    layout
}

fn get_off_neighbours(layout: &Vec<Vec<bool>>, on_chunk: &IPoint2) -> Vec<IPoint2> {
    let mut neighbours = Vec::new();
    if on_chunk.y > 0 { if !layout[on_chunk.x][on_chunk.y - 1] { neighbours.push(IPoint2{ x: on_chunk.x, y: on_chunk.y -1 }) } }

    if on_chunk.y < layout[0].len() - 1 { if !layout[on_chunk.x][on_chunk.y + 1] { neighbours.push(IPoint2{ x: on_chunk.x, y: on_chunk.y + 1 }) } }

    if on_chunk.x > 0 { if !layout[on_chunk.x - 1][on_chunk.y] { neighbours.push(IPoint2{ x: on_chunk.x - 1, y: on_chunk.y }) } }

    if on_chunk.x < layout.len() - 1 { if !layout[on_chunk.x + 1][on_chunk.y] { neighbours.push(IPoint2{ x: on_chunk.x + 1, y: on_chunk.y }) } }

    neighbours
}

fn get_on_chunks(layout: &Vec<Vec<bool>>) -> Vec<IPoint2> {
    let mut chunk_list = Vec::new();
    for (x, row) in layout.iter().enumerate() {
        for (y, tile) in row.iter().enumerate() {
            if *tile { chunk_list.push(IPoint2{ x, y  }); }
        }
    }
    chunk_list
}

fn generate_floor(rng: ThreadRng) -> Vec<Vec<Option<&'static str>>> {
    let layout = generate_floor_layout(rng);
    let mut floor = vec![vec![None; LAYOUT_DIM * CHUNK_SIZE]; LAYOUT_DIM * CHUNK_SIZE];
    for chunk_x in 0..LAYOUT_DIM {
        for chunk_y in 0..LAYOUT_DIM {
            if layout[chunk_x][chunk_y] {
                for x in chunk_x * CHUNK_SIZE..(chunk_x + 1) * CHUNK_SIZE {
                    for y in chunk_y * CHUNK_SIZE..(chunk_y + 1) * CHUNK_SIZE {
                        floor[x][y] = Some("floor");
                    }
                }
            }
        }
    }

    for (x, row) in floor.clone().iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            if floor[x][y].is_some() {
                if  x == 0 || y == 0 || x == floor.len() - 1 || y == floor[0].len() - 1
                    || floor[x + 1][y].is_none()
                    || floor[x - 1][y].is_none()
                    || floor[x][y + 1].is_none()
                    || floor[x][y - 1].is_none()
                {
                    floor[x][y] = Some("wall");
                }
            }
        }
    }

    floor
}

pub fn generate_level(suit: Suit) -> Level {
    let rng = rand::thread_rng();
    let mut grid = Vec::new();
    let floor = generate_floor(rng);

    for y in 0..floor.len() {
        let mut row = Vec::new();
        for x in 0..floor[0].len() {
            if floor[x][y].is_some() {
                match floor[x][y].unwrap() {
                    "floor" => row.push(Some(TileAttributes { tile_coord: get_tile_coord(&suit.floor_tiles, rng), solid: false })),
                    "wall" => row.push(Some(TileAttributes { tile_coord: get_tile_coord(&suit.wall_tiles, rng), solid: true })),
                    _ => row.push(None)
                }
            } else {
            }
        }
        grid.push(row);
    }
    Level{
        floor: grid,
    }
}

fn get_tile_coord(possible_tiles: &Vec<IPoint2>, mut rng: ThreadRng) -> IPoint2 {
    possible_tiles[rng.gen_range(0, possible_tiles.len())]
}
