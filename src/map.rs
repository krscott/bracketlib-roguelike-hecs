use bracket_lib::prelude::*;
use specs::World;
use std::cmp::{max, min};

use crate::color;
use crate::glyph;
use crate::rect::Rect;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

impl TileType {
    fn fg(&self, is_visible: bool) -> RGB {
        match (self, is_visible) {
            (TileType::Wall, true) => color::wall_fg(),
            (TileType::Wall, false) => color::wall_fog_fg(),
            (TileType::Floor, true) => color::floor_fg(),
            (TileType::Floor, false) => color::floor_fog_fg(),
        }
    }

    fn bg(&self, _is_visible: bool) -> RGB {
        color::bg()
    }

    fn glyph(&self) -> FontCharType {
        match self {
            TileType::Wall => glyph::wall(),
            TileType::Floor => glyph::floor(),
        }
    }

    fn is_opaque(&self) -> bool {
        match self {
            TileType::Wall => true,
            TileType::Floor => false,
        }
    }
}

pub struct Map {
    tiles: Vec<TileType>,
    rooms: Vec<Rect>,
    width: i32,
    height: i32,
    revealed_tiles: Vec<bool>,
    visible_tiles: Vec<bool>,
}

impl Map {
    fn blank(width: i32, height: i32, tile_type: TileType) -> Self {
        let num_tiles = width as usize * height as usize;

        Self {
            tiles: vec![tile_type; num_tiles],
            rooms: Vec::new(),
            width,
            height,
            revealed_tiles: vec![false; num_tiles],
            visible_tiles: vec![false; num_tiles],
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

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn get_player_starting_position(&self) -> (i32, i32) {
        match self.rooms.first() {
            Some(room) => room.center(),
            None => (self.width / 2, self.height / 2),
        }
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Option<&TileType> {
        self.get_index(x, y).and_then(|index| self.tiles.get(index))
    }

    pub fn set_tile_revealed(&mut self, x: i32, y: i32) {
        if let Some(index) = self.get_index(x, y) {
            self.revealed_tiles[index] = true;
        }
    }

    pub fn set_tile_visible(&mut self, x: i32, y: i32) {
        if let Some(index) = self.get_index(x, y) {
            self.visible_tiles[index] = true;
        }
    }

    pub fn clear_visible_tiles(&mut self) {
        for x in self.visible_tiles.iter_mut() {
            *x = false;
        }
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
}

impl BaseMap for Map {
    fn is_opaque(&self, index: usize) -> bool {
        if let Some(tile) = self.tiles.get(index) {
            tile.is_opaque()
        } else {
            // Tile is out of bounds
            assert!(false);
            true
        }
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

pub fn draw_map(ecs: &World, context: &mut BTerm) {
    let map = ecs.fetch::<Map>();

    for (i, tile) in map.tiles.iter().enumerate() {
        if map.revealed_tiles[i] {
            let (x, y) = map.get_coords(i);
            let is_vis = map.visible_tiles[i];
            context.set(x, y, tile.fg(is_vis), tile.bg(is_vis), tile.glyph());
        }
    }
}
