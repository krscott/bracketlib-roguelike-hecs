use anyhow::anyhow;
use bracket_lib::random::RandomNumberGenerator;
use hecs::{Entity, World};

use crate::{
    components::{BlocksTile, CombatStats, Monster, Name, Position, Renderable, Viewshed},
    config::Config,
    player::Player,
    rect::Rect,
};

const MAX_MONSTERS: i32 = 4;
const _MAX_ITEMS: i32 = 2;

const SPAWN_ATTEMPTS_TIMEOUT: i32 = 1000;

pub fn player(world: &mut World, config: &Config, x: i32, y: i32) -> Entity {
    world.spawn((
        Player,
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
    ))
}

pub fn rng_monster(world: &mut World, config: &Config, x: i32, y: i32) -> anyhow::Result<Entity> {
    let dice_roll = {
        let mut rng = world.query::<&mut RandomNumberGenerator>();
        let (_, rng) = rng
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("Missing RandomNumberGenerator entity"))?;
        rng.roll_dice(1, 2)
    };

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

pub fn rng_room_of_monsters(world: &mut World, config: &Config, room: &Rect) -> anyhow::Result<()> {
    let mut monster_spawn_points = Vec::new();

    {
        let mut rng = world.query::<&mut RandomNumberGenerator>();
        let (_, rng) = rng
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("Missing RandomNumberGenerator entity"))?;

        let num_monsters = rng.roll_dice(1, MAX_MONSTERS + 2) - 3;

        for _ in 0..num_monsters {
            for _ in 0..SPAWN_ATTEMPTS_TIMEOUT {
                let x = room.x1 + rng.roll_dice(1, room.width());
                let y = room.y1 + rng.roll_dice(1, room.height());

                let point = (x, y);

                if !monster_spawn_points.contains(&point) {
                    monster_spawn_points.push(point);
                    break;
                }
            }
        }
    }

    for (x, y) in monster_spawn_points {
        rng_monster(world, config, x, y)?;
    }

    Ok(())
}
