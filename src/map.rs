use bracket_lib::prelude::*;
use hecs::{Entity, World};
use itertools::Itertools;
use std::{
    cmp::{max, min},
    vec,
};

use crate::{
    components::{Position, Renderable},
    config::Config,
    rect::Rect,
};

const EMPTY_ENTITY_ARRAY: &'static [Entity] = &[];

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

impl TileType {
    fn fg(&self, config: &Config, is_visible: bool) -> RGB {
        match (self, is_visible) {
            (TileType::Wall, true) => config.wall.fg,
            (TileType::Wall, false) => config.wall.fog_fg,
            (TileType::Floor, true) => config.floor.fg,
            (TileType::Floor, false) => config.floor.fog_fg,
        }
    }

    fn bg(&self, config: &Config, is_visible: bool) -> RGB {
        match (self, is_visible) {
            (TileType::Wall, true) => config.wall.bg,
            (TileType::Wall, false) => config.wall.fog_bg,
            (TileType::Floor, true) => config.floor.bg,
            (TileType::Floor, false) => config.floor.fog_bg,
        }
    }

    fn glyph(&self, config: &Config) -> FontCharType {
        match self {
            TileType::Wall => config.wall.glyph,
            TileType::Floor => config.floor.glyph,
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
    blocked_tiles: Vec<bool>,
    tile_content: Vec<Vec<Entity>>,
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
            blocked_tiles: vec![false; num_tiles],
            tile_content: vec![Vec::new(); num_tiles],
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

    pub fn get_rooms(&self) -> &[Rect] {
        &self.rooms
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

    pub fn set_tile_blocked(&mut self, x: i32, y: i32, is_blocked: bool) {
        if let Some(index) = self.get_index(x, y) {
            self.blocked_tiles[index] = is_blocked;
        }
    }

    pub fn reset_blocked_tiles(&mut self) {
        for (i, is_blocked) in self.blocked_tiles.iter_mut().enumerate() {
            *is_blocked = match self.tiles[i] {
                TileType::Wall => true,
                TileType::Floor => false,
            }
        }
    }

    pub fn get_entities_on_tile(&self, x: i32, y: i32) -> &[Entity] {
        if let Some(index) = self.get_index(x, y) {
            &self.tile_content[index]
        } else {
            EMPTY_ENTITY_ARRAY
        }
    }

    pub fn add_entity_to_tile_content(&mut self, x: i32, y: i32, entity: Entity) {
        if let Some(index) = self.get_index(x, y) {
            self.tile_content[index].push(entity);
        }
    }

    pub fn clear_content_index(&mut self) {
        for content in self.tile_content.iter_mut() {
            content.clear();
        }
    }

    // pub fn is_tile_revealed(&self, x: i32, y: i32) -> bool {
    //     if let Some(index) = self.get_index(x, y) {
    //         self.revealed_tiles[index]
    //     } else {
    //         false
    //     }
    // }

    pub fn is_tile_visible(&self, x: i32, y: i32) -> bool {
        if let Some(index) = self.get_index(x, y) {
            self.visible_tiles[index]
        } else {
            false
        }
    }

    pub fn is_tile_blocked(&self, x: i32, y: i32) -> bool {
        if let Some(index) = self.get_index(x, y) {
            self.blocked_tiles[index]
        } else {
            true
        }
    }

    pub fn get_index(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return None;
        }

        let index = (y as usize * self.width as usize) + x as usize;

        assert!(index < self.tiles.len());

        Some(index)
    }

    pub fn get_coords(&self, index: usize) -> (i32, i32) {
        assert!(index < self.tiles.len());

        let x = (index % self.width as usize) as i32;
        let y = (index / self.width as usize) as i32;
        (x, min(y, self.height))
    }

    fn apply_tile(&mut self, x: i32, y: i32, tile_type: TileType) {
        if let Some(i) = self.get_index(x, y) {
            self.tiles[i] = tile_type;
        }
    }

    fn apply_rect(&mut self, rect: &Rect, tile_type: TileType) {
        for y in (rect.y1 + 1)..=rect.y2 {
            for x in (rect.x1 + 1)..=rect.x2 {
                self.apply_tile(x, y, tile_type);
            }
        }
    }

    fn apply_horizontal_line(&mut self, x1: i32, x2: i32, y: i32, tile_type: TileType) {
        for x in min(x1, x2)..=max(x1, x2) {
            self.apply_tile(x, y, tile_type);
        }
    }

    fn apply_vertical_line(&mut self, y1: i32, y2: i32, x: i32, tile_type: TileType) {
        for y in min(y1, y2)..=max(y1, y2) {
            self.apply_tile(x, y, tile_type);
        }
    }

    fn is_valid_exit(&self, x: i32, y: i32) -> bool {
        !self.is_tile_blocked(x, y)
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

    fn get_pathing_distance(&self, index1: usize, index2: usize) -> f32 {
        let p1 = Point::from_tuple(self.get_coords(index1));
        let p2 = Point::from_tuple(self.get_coords(index2));
        DistanceAlg::Pythagoras.distance2d(p1, p2)
    }

    fn get_available_exits(&self, index: usize) -> SmallVec<[(usize, f32); 10]> {
        let (original_x, original_y) = self.get_coords(index);

        (-1..=1)
            .cartesian_product(-1..=1)
            .filter(|(dx, dy)| *dx != 0 || *dy != 0)
            .map(|(dx, dy)| {
                (
                    original_x + dx,
                    original_y + dy,
                    if i32::abs(dx) == i32::abs(dy) {
                        1.45_f32
                    } else {
                        1.0_f32
                    },
                )
            })
            .filter(|(x, y, _cost)| self.is_valid_exit(*x, *y))
            .filter_map(|(x, y, cost)| self.get_index(x, y).map(|i| (i, cost)))
            .map(|(i, cost)| (i, cost))
            .collect()
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

pub fn draw_map(context: &mut BTerm, world: &World, config: &Config, map_entity: Entity) {
    let map = world.get::<Map>(map_entity).unwrap();

    for (i, tile) in map.tiles.iter().enumerate() {
        if map.revealed_tiles[i] {
            let (x, y) = map.get_coords(i);
            let is_visible = map.visible_tiles[i];
            context.set(
                x,
                y,
                tile.fg(config, is_visible),
                tile.bg(config, is_visible),
                tile.glyph(config),
            );
        }
    }

    for (_, (pos, render)) in world.query::<(&Position, &Renderable)>().into_iter() {
        if map.is_tile_visible(pos.x, pos.y) {
            context.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}
