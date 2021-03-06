use std::collections::HashSet;

use crate::{despawn_entities_system::queue_despawn_batch, prelude::*};

pub fn damage_system(world: &mut World) -> anyhow::Result<()> {
    let mut despawn_entities = HashSet::new();

    {
        let player_entity = world.resource_entity::<Player>().ok();

        for (_, cmd) in world.query::<&DamageCommand>().into_iter() {
            let mut stats = match world.query_one::<&mut CombatStats>(cmd.entity) {
                Ok(stats) => stats,
                Err(_err) => {
                    console::log(format!(
                        "Error: DamageCommand Entity {} does not have CombatStats component",
                        cmd.entity.id()
                    ));
                    continue;
                }
            };
            let stats = stats.get().expect("Unfiltered query");

            stats.hp = i32::max(0, stats.hp - cmd.amount);

            if stats.hp <= 0 {
                despawn_entities.insert(cmd.entity);

                if Some(cmd.entity) == player_entity {
                    GameLog::resource_push(world, "You are dead!")?;
                } else {
                    if let Ok(mut q) = world.query_one::<&Name>(cmd.entity) {
                        if let Some(Name(name)) = q.get() {
                            GameLog::resource_push(world, format!("{} was slain!", name))?;
                        }
                    }
                }
            }
        }
    }

    queue_despawn_batch(world, despawn_entities);

    Ok(())
}
