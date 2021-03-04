use hecs::World;

use crate::{
    command::command_bundle,
    components::{CombatStats, DamageCommand, InitiateAttackCommand, Name},
    gamelog::GameLog,
};

pub fn melee_combat_system(world: &mut World) {
    let mut damage_commands_batch = Vec::new();

    {
        let mut log = world.query::<&mut GameLog>();
        let log = &mut log.into_iter().next().map(|(_ent, log)| log);

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
                    if let Some(log) = log {
                        log.push(format!(
                            "{} hits {}, for {} hp.",
                            attacker_name.0, defender_name.0, damage
                        ));
                    }
                    damage_commands_batch.push(command_bundle(DamageCommand {
                        entity: cmd.defender,
                        amount: damage,
                    }))
                } else {
                    if let Some(log) = log {
                        log.push(format!(
                            "{} is unable to hurt {}.",
                            attacker_name.0, defender_name.0
                        ));
                    }
                }
            }
        }
    }

    world.spawn_batch(damage_commands_batch);
}
