use bracket_lib::prelude::*;
use std::cmp::{max, min};

use crate::rect::Rect;

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

pub struct Map {
    tiles: Vec<TileType>,
    rooms: Vec<Rect>,
    width: i32,
    height: i32,
}

impl Map {
    fn blank(width: i32, height: i32, tile_type: TileType) -> Self {
        Self {
            tiles: vec![tile_type; width as usize * height as usize],
            rooms: Vec::new(),
            width,
            height,
        }
    }

    pub fn rooms_and_cooridors(width: i32, height: i32) -> Self {
        let mut map = Self::blank(width, height, TileType::Wall);

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, map.width - w - 1) - 1;
            let y = rng.roll_dice(1, map.height - h - 1) - 1;
            let new_room = Rect::new(x, y, w, h);

            let intersects_existing_room = map.rooms.iter().any(|other| new_room.intersect(other));

            if !intersects_existing_room {
                map.apply_rect(&new_room, TileType::Floor);

                if let Some(prev_room) = map.rooms.last() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = prev_room.center();

                    if rng.range(0, 2) == 1 {
                        map.apply_horizontal_line(prev_x, new_x, prev_y, TileType::Floor);
                        map.apply_vertical_line(prev_y, new_y, new_x, TileType::Floor);
                    } else {
                        map.apply_vertical_line(prev_y, new_y, prev_x, TileType::Floor);
                        map.apply_horizontal_line(prev_x, new_x, new_y, TileType::Floor);
                    }
                }

                map.rooms.push(new_room);
            }
        }

        map
    }

    pub fn get_player_starting_position(&self) -> (i32, i32) {
        match self.rooms.first() {
            Some(room) => room.center(),
            None => (self.width / 2, self.height / 2),
        }
    }

    // pub fn get_tiles(&self) -> &[TileType] {
    //     &self.tiles
    // }

    // pub fn get_rooms(&self) -> &[Rect] {
    //     &self.rooms
    // }

    // pub fn get_width(&self) -> i32 {
    //     self.width
    // }

    // pub fn get_height(&self) -> i32 {
    //     self.height
    // }

    pub fn get_tile(&self, x: i32, y: i32) -> Option<&TileType> {
        self.get_index(x, y).and_then(|index| self.tiles.get(index))
    }

    fn get_index(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return None;
        }

        let index = (y as usize * self.width as usize) + x as usize;

        assert!(index < self.tiles.len());

        Some(index)
    }

    fn get_coords(&self, index: usize) -> (i32, i32) {
        assert!(index < self.tiles.len());

        let x = (index % self.width as usize) as i32;
        let y = (index / self.width as usize) as i32;
        (x, min(y, self.height))
    }

    fn apply_rect(&mut self, rect: &Rect, tile_type: TileType) {
        for y in (rect.y1 + 1)..=rect.y2 {
            for x in (rect.x1 + 1)..=rect.x2 {
                if let Some(i) = self.get_index(x, y) {
                    self.tiles[i] = tile_type;
                }
            }
        }
    }

    fn apply_horizontal_line(&mut self, x1: i32, x2: i32, y: i32, tile_type: TileType) {
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(i) = self.get_index(x, y) {
                self.tiles[i] = tile_type;
            }
        }
    }

    fn apply_vertical_line(&mut self, y1: i32, y2: i32, x: i32, tile_type: TileType) {
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(i) = self.get_index(x, y) {
                self.tiles[i] = tile_type;
            }
        }
    }

    pub fn draw_to_context(&self, context: &mut BTerm) {
        for (i, tile) in self.tiles.iter().enumerate() {
            let (x, y) = self.get_coords(i);
            context.set(x, y, tile.fg(), tile.bg(), tile.glyph())
        }
    }
}
