use hecs::World;

use crate::{components::SufferDamage, CombatStats};

pub fn damage_system(world: &mut World) {
    let mut entities = Vec::new();

    for (entity, (stats, damage)) in world
        .query::<(&mut CombatStats, &mut SufferDamage)>()
        .into_iter()
    {
        stats.hp -= damage.amount.iter().sum::<i32>();

        entities.push(entity);
    }

    for entity in entities {
        world.remove_one::<SufferDamage>(entity).unwrap();
    }
}

pub fn delete_the_dead(world: &mut World) {
    let dead = world
        .query::<&CombatStats>()
        .into_iter()
        .filter(|(_entity, stats)| stats.hp <= 0)
        .map(|(entity, _)| entity)
        .collect::<Vec<_>>();

    for victim in dead {
        world.despawn(victim).unwrap();
    }
}
