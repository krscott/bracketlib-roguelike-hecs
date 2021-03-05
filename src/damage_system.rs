use std::collections::HashSet;

use hecs::World;

use crate::{
    command::command_bundle,
    components::{DamageCommand, DespawnCommand, Name},
    gamelog::GameLog,
    CombatStats,
};

pub fn damage_system(world: &mut World) {
    let mut despawn_entities = HashSet::new();

    {
        for (_, cmd) in world.query::<&DamageCommand>().into_iter() {
            let mut query_combat_stats = world.query_one::<&mut CombatStats>(cmd.entity).unwrap();
            let stats = query_combat_stats.get().unwrap();

            stats.hp = i32::max(0, stats.hp - cmd.amount);

            if stats.hp <= 0 {
                despawn_entities.insert(cmd.entity);

                if let Ok(mut q) = world.query_one::<&Name>(cmd.entity) {
                    if let Some(Name(name)) = q.get() {
                        GameLog::push_world(world, format!("{} was slain!", name));
                    }
                }
            }
        }
    }

    world.spawn_batch(
        despawn_entities
            .into_iter()
            .map(|ent| command_bundle(DespawnCommand(ent))),
    );
}
