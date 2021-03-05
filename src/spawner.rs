use anyhow::anyhow;
use bracket_lib::{prelude::Rect, random::RandomNumberGenerator};
use hecs::{Entity, World};

use crate::{
    components::{
        BlocksTile, CombatStats, HealingItem, Item, Monster, Name, Position, Renderable, Viewshed,
    },
    config::Config,
    player::Player,
    resource,
};

const MAX_MONSTERS: i32 = 4;
const MAX_ITEMS: i32 = 2;

const SPAWN_ATTEMPTS_TIMEOUT: i32 = 1000;

pub fn player(world: &mut World, config: &Config, x: i32, y: i32) -> anyhow::Result<Entity> {
    Ok(resource::spawn(
        world,
        Player,
        (
            Name("Player".into()),
            Position { x, y },
            Renderable {
                glyph: config.player.glyph,
                fg: config.player.fg,
                bg: config.player.bg,
            },
            Viewshed::with_range(8),
            CombatStats {
                max_hp: 30,
                hp: 30,
                defense: 2,
                power: 5,
            },
        ),
    )?)
}

pub fn rng_monster(world: &mut World, config: &Config, x: i32, y: i32) -> anyhow::Result<Entity> {
    let dice_roll =
        resource::map::<RandomNumberGenerator, _, _>(world, |mut rng| rng.roll_dice(1, 2))?;

    let entity = match dice_roll {
        1 => orc(world, config, x, y),
        _ => goblin(world, config, x, y),
    };

    Ok(entity)
}

fn orc(world: &mut World, config: &Config, x: i32, y: i32) -> Entity {
    monster(world, x, y, config.orc.to_renderable(), "Orc")
}

fn goblin(world: &mut World, config: &Config, x: i32, y: i32) -> Entity {
    monster(world, x, y, config.goblin.to_renderable(), "Goblin")
}

fn monster<S: Into<String>>(
    world: &mut World,
    x: i32,
    y: i32,
    renderable: Renderable,
    name: S,
) -> Entity {
    world.spawn((
        Monster,
        Name(name.into()),
        Position { x, y },
        renderable,
        Viewshed::with_range(8),
        BlocksTile,
        CombatStats {
            max_hp: 16,
            hp: 16,
            defense: 1,
            power: 4,
        },
    ))
}

fn get_random_points_in_rect(
    rect: &Rect,
    rng: &mut RandomNumberGenerator,
    max_points: i32,
) -> Vec<(i32, i32)> {
    let mut points = Vec::new();

    for _ in 0..max_points {
        for _ in 0..SPAWN_ATTEMPTS_TIMEOUT {
            let x = rect.x1 + rng.roll_dice(1, rect.width());
            let y = rect.y1 + rng.roll_dice(1, rect.height());

            let point = (x, y);

            if !points.contains(&point) {
                points.push(point);
                break;
            }
        }
    }

    points
}

pub fn rng_room_entities(world: &mut World, config: &Config, room: &Rect) -> anyhow::Result<()> {
    let monster_spawn_points;
    let item_spawn_points;

    {
        let mut rng = world.query::<&mut RandomNumberGenerator>();
        let (_, rng) = rng
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("Missing RandomNumberGenerator entity"))?;

        let num_monsters = rng.roll_dice(1, MAX_MONSTERS + 2) - 3;
        let num_items = rng.roll_dice(1, MAX_ITEMS + 2) - 3;

        monster_spawn_points = get_random_points_in_rect(&room, rng, num_monsters);
        item_spawn_points = get_random_points_in_rect(&room, rng, num_items);
    }

    for (x, y) in monster_spawn_points {
        rng_monster(world, config, x, y)?;
    }

    for (x, y) in item_spawn_points {
        health_potion(world, config, x, y);
    }

    Ok(())
}

pub fn health_potion(world: &mut World, config: &Config, x: i32, y: i32) -> Entity {
    world.spawn((
        Position { x, y },
        config.health_potion.to_renderable(),
        Name("Health Potion".into()),
        Item,
        HealingItem { heal_amount: 8 },
    ))
}
