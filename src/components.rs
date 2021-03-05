use std::fmt::Display;

use bracket_lib::prelude::*;
use hecs::{Entity, World};

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn to_point(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }
}

impl From<Position> for Point {
    fn from(position: Position) -> Self {
        position.to_point()
    }
}

#[derive(Debug)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Debug)]
pub struct Player;

impl Player {
    pub fn get_entity(world: &mut World) -> Option<Entity> {
        world
            .query::<&Player>()
            .into_iter()
            .next()
            .map(|(entity, _)| entity)
    }
}

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

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug)]
pub struct BlocksTile;

#[derive(Debug, Default, Clone, Copy)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}

#[derive(Debug)]
pub struct Command;

#[derive(Debug)]
pub struct InitiateAttackCommand {
    pub attacker: Entity,
    pub defender: Entity,
}

#[derive(Debug)]
pub struct DamageCommand {
    pub entity: Entity,
    pub amount: i32,
}

#[derive(Debug)]
pub struct DespawnCommand(pub Entity);
