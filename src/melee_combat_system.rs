use bracket_lib::prelude::console;
use hecs::World;

use crate::components::{CombatStats, Name, SufferDamage, WantsToMelee};

pub fn melee_combat_system(world: &mut World) {
    let mut suffer_queue = Vec::new();
    let mut wants_melee_queue = Vec::new();

    for (entity, (wants_melee, Name(name), stats)) in world
        .query::<(&mut WantsToMelee, &Name, &CombatStats)>()
        .into_iter()
    {
        if stats.hp > 0 {
            let target_stats = world.get::<CombatStats>(wants_melee.target).unwrap();

            if target_stats.hp > 0 {
                let target_name = world.get::<Name>(wants_melee.target).unwrap();
                let damage = i32::max(0, stats.power - target_stats.defense);

                if damage > 0 {
                    console::log(&format!(
                        "{} hits {}, for {} hp.",
                        name, target_name.0, damage
                    ));
                    suffer_queue.push((wants_melee.target, damage))
                } else {
                    console::log(&format!("{} is unable to hurt {}.", name, target_name.0));
                }
            }
        }

        wants_melee_queue.push(entity);
    }

    for (target, damage) in suffer_queue {
        SufferDamage::new_damage(world, target, damage);
    }

    for entity in wants_melee_queue {
        world.remove_one::<WantsToMelee>(entity).unwrap();
    }
}
