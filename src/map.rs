use bracket_lib::prelude::*;
use std::cmp::{max, min};

use crate::rect::Rect;

use super::consts::{MAP_HEIGHT, MAP_WIDTH};

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

/// Create a blank map
fn create_fill_map(tile_type: TileType) -> GameMap {
    vec![tile_type; MAP_WIDTH * MAP_HEIGHT]
}

/// Create a randomized map of tiles
#[allow(dead_code)]
pub fn create_test_map() -> GameMap {
    let mut map = create_fill_map(TileType::Floor);

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

    for _ in 0..400 {
        let x = rng.roll_dice(1, MAP_WIDTH as i32 - 1);
        let y = rng.roll_dice(1, MAP_HEIGHT as i32 - 1);
        let i = get_map_index(x, y);

        map[i] = TileType::Wall;
    }

    map
}

fn apply_rect_to_map(map: &mut [TileType], rect: &Rect, tile_type: TileType) {
    for y in (rect.y1 + 1)..=rect.y2 {
        for x in (rect.x1 + 1)..=rect.x2 {
            map[get_map_index(x, y)] = tile_type;
        }
    }
}

fn apply_horizontal_line(map: &mut [TileType], x1: i32, x2: i32, y: i32, tile_type: TileType) {
    for x in min(x1, x2)..=max(x1, x2) {
        let i = get_map_index(x, y);
        map[i] = tile_type;
    }
}

fn apply_vertical_line(map: &mut [TileType], y1: i32, y2: i32, x: i32, tile_type: TileType) {
    for y in min(y1, y2)..=max(y1, y2) {
        let i = get_map_index(x, y);
        map[i] = tile_type;
    }
}

pub fn create_rooms_and_corridors_map() -> (GameMap, Vec<Rect>) {
    let mut map = create_fill_map(TileType::Wall);

    let mut rooms = Vec::new();

    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: i32 = 6;
    const MAX_SIZE: i32 = 10;

    let mut rng = RandomNumberGenerator::new();

    for _ in 0..MAX_ROOMS {
        let w = rng.range(MIN_SIZE, MAX_SIZE);
        let h = rng.range(MIN_SIZE, MAX_SIZE);
        let x = rng.roll_dice(1, MAP_WIDTH as i32 - w - 1) - 1;
        let y = rng.roll_dice(1, MAP_HEIGHT as i32 - h - 1) - 1;
        let new_room = Rect::new(x, y, w, h);

        let intersects_existing_room = rooms.iter().any(|other| new_room.intersect(other));

        if !intersects_existing_room {
            apply_rect_to_map(&mut map, &new_room, TileType::Floor);

            if let Some(prev_room) = rooms.last() {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = prev_room.center();

                if rng.range(0, 2) == 1 {
                    apply_horizontal_line(&mut map, prev_x, new_x, prev_y, TileType::Floor);
                    apply_vertical_line(&mut map, prev_y, new_y, new_x, TileType::Floor);
                } else {
                    apply_vertical_line(&mut map, prev_y, new_y, prev_x, TileType::Floor);
                    apply_horizontal_line(&mut map, prev_x, new_x, new_y, TileType::Floor);
                }
            }

            rooms.push(new_room);
        }
    }

    (map, rooms)
}

/// Draw a GameMap
pub fn draw_map(map: &[TileType], context: &mut BTerm) {
    for (i, tile) in map.iter().enumerate() {
        let (x, y) = get_map_coords(i);
        context.set(x, y, tile.fg(), tile.bg(), tile.glyph())
    }
}
