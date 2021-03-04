use bracket_lib::prelude::console;
use hecs::World;

use crate::{
    command::command_bundle,
    components::{CombatStats, DamageCommand, InitiateAttackCommand, Name},
};

pub fn melee_combat_system(world: &mut World) {
    let mut damage_commands_batch = Vec::new();

    for (_, cmd) in world.query::<&InitiateAttackCommand>().into_iter() {
        let mut attacker_query = world
            .query_one::<(&CombatStats, &Name)>(cmd.attacker)
            .unwrap();
        let (attacker_stats, attacker_name) = attacker_query.get().unwrap();

        let mut defender_query = world
            .query_one::<(&CombatStats, &Name)>(cmd.defender)
            .unwrap();
        let (defender_stats, defender_name) = defender_query.get().unwrap();

        if attacker_stats.hp > 0 && defender_stats.hp > 0 {
            let damage = i32::max(0, attacker_stats.power - defender_stats.defense);

            if damage > 0 {
                console::log(&format!(
                    "{} hits {}, for {} hp.",
                    attacker_name.0, defender_name.0, damage
                ));
                damage_commands_batch.push(command_bundle(DamageCommand {
                    entity: cmd.defender,
                    amount: damage,
                }))
            } else {
                console::log(&format!(
                    "{} is unable to hurt {}.",
                    attacker_name.0, defender_name.0
                ));
            }
        }
    }

    world.spawn_batch(damage_commands_batch);
}
