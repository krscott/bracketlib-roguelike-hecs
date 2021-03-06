use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
    pub render_order: i32,
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

impl Name {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

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
pub struct Item;

#[derive(Debug)]
pub struct HealingItem {
    pub heal_amount: i32,
}

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
pub struct Player;

#[derive(Debug)]
pub struct InInventory {
    pub owner: Entity,
}

#[derive(Debug, Clone, Copy)]
pub struct PickupItemCommand {
    pub collector: Entity,
    pub item: Entity,
}

#[derive(Debug)]
pub struct UseItemCommand {
    pub user: Entity,
    pub item: Entity,
}

#[derive(Debug, Clone, Copy)]
pub struct DropItemCommand {
    pub dropper: Entity,
    pub item: Entity,
}
