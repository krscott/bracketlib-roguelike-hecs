use anyhow::anyhow;
use bracket_lib::random::RandomNumberGenerator;
use hecs::{Entity, World};

use crate::{
    components::{BlocksTile, CombatStats, Monster, Name, Player, Position, Renderable, Viewshed},
    config::Config,
};

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
