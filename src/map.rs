use bracket_lib::prelude::*;
use std::cmp::{max, min};

use super::consts::{MAP_HEIGHT, MAP_WIDTH, PLAYER_START_X, PLAYER_START_Y};

// ==================
// =     Types      =
// ==================

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

impl TileType {
    fn fg(&self) -> RGB {
        match self {
            TileType::Wall => RGB::from_f32(0.0, 1.0, 0.0),
            TileType::Floor => RGB::from_f32(0.5, 0.5, 0.5),
        }
    }

    fn bg(&self) -> RGB {
        RGB::from_f32(0.0, 0.0, 0.0)
    }

    fn glyph(&self) -> u16 {
        let char = match self {
            TileType::Wall => '#',
            TileType::Floor => '.',
        };

        to_cp437(char)
    }
}

pub type GameMap = Vec<TileType>;

// ==================
// =   Functions    =
// ==================

pub fn get_map_index(x: i32, y: i32) -> usize {
    let x = min(MAP_WIDTH, max(0, x as usize));
    let y = min(MAP_HEIGHT, max(0, y as usize));

    (y * MAP_WIDTH) + x
}

pub fn get_map_coords(index: usize) -> (i32, i32) {
    let x = (index % MAP_WIDTH) as i32;
    let y = (index / MAP_WIDTH) as i32;
    (x, min(y, MAP_HEIGHT as i32))
}

/// Create a randomized map of tiles
pub fn create_random_map() -> GameMap {
    let mut map = vec![TileType::Floor; MAP_WIDTH * MAP_HEIGHT];

    // Map edge walls
    for x in 0..(MAP_WIDTH as i32) {
        map[get_map_index(x, 0)] = TileType::Wall;
        map[get_map_index(x, MAP_HEIGHT as i32 - 1)] = TileType::Wall;
    }
    for y in 0..(MAP_HEIGHT as i32) {
        map[get_map_index(0, y)] = TileType::Wall;
        map[get_map_index(MAP_WIDTH as i32 - 1, y)] = TileType::Wall;
    }

    // Random walls
    let mut rng = RandomNumberGenerator::new();

    let player_index = get_map_index(PLAYER_START_X, PLAYER_START_Y);
    for _ in 0..400 {
        let x = rng.roll_dice(1, MAP_WIDTH as i32 - 1);
        let y = rng.roll_dice(1, MAP_HEIGHT as i32 - 1);
        let i = get_map_index(x, y);

        if i != player_index {
            map[i] = TileType::Wall;
        }
    }

    map
}

/// Draw a GameMap
pub fn draw_map(map: &[TileType], context: &mut BTerm) {
    for (i, tile) in map.iter().enumerate() {
        let (x, y) = get_map_coords(i);
        context.set(x, y, tile.fg(), tile.bg(), tile.glyph())
    }
}
