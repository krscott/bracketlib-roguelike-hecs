use bracket_lib::prelude::*;

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Debug)]
pub struct Player;

#[derive(Debug)]
pub struct Viewshed {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
    pub dirty: bool,
}

impl Viewshed {
    pub fn with_range(range: i32) -> Self {
        Self {
            visible_tiles: Vec::new(),
            range,
            dirty: true,
        }
    }
}

#[derive(Debug)]
pub struct Monster;

#[derive(Debug)]
pub struct Name(pub String);

#[derive(Debug)]
pub struct BlocksTile;
