use bracket_lib::prelude::console;
use hecs::World;

use crate::{
    command::command_bundle,
    components::{CombatStats, DamageCommand, InitiateAttackCommand, Name, Player},
    gamelog::GameLog,
};

pub fn melee_combat_system(world: &mut World) {
    let mut damage_commands_batch = Vec::new();

    let player_entity = Player::get_entity(world);

    for (_, cmd) in world.query::<&InitiateAttackCommand>().into_iter() {
        let mut attacker_query = match world.query_one::<(&CombatStats, &Name)>(cmd.attacker) {
            Ok(q) => q,
            Err(_err) => {
                console::log(format!(
                    "Error: InitiateAttackCommand attacker Entity {} does not have CombatStats component",
                    cmd.attacker.id()
                ));
                continue;
            }
        };
        let (attacker_stats, attacker_name) = attacker_query.get().expect("Unfiltered query");

        let mut defender_query = match world.query_one::<(&CombatStats, &Name)>(cmd.defender) {
            Ok(q) => q,
            Err(_err) => {
                console::log(format!(
                    "Error: InitiateAttackCommand defender Entity {} does not have CombatStats component",
                    cmd.attacker.id()
                ));
                continue;
            }
        };
        let (defender_stats, defender_name) = defender_query.get().expect("Unfiltered query");

        if attacker_stats.hp > 0 && defender_stats.hp > 0 {
            let damage = i32::max(0, attacker_stats.power - defender_stats.defense);

            let (attacker_name, is_are, defender_name) = if Some(cmd.attacker) == player_entity {
                ("You", "are", defender_name.0.as_str())
            } else if Some(cmd.defender) == player_entity {
                (attacker_name.0.as_str(), "is", "you")
            } else {
                (attacker_name.0.as_str(), "is", defender_name.0.as_str())
            };

            if damage > 0 {
                GameLog::push_world(
                    world,
                    format!("{} hit {} for {} hp.", attacker_name, defender_name, damage),
                );
                damage_commands_batch.push(command_bundle(DamageCommand {
                    entity: cmd.defender,
                    amount: damage,
                }))
            } else {
                GameLog::push_world(
                    world,
                    format!(
                        "{} {} unable to hurt {}.",
                        attacker_name, is_are, defender_name
                    ),
                );
            }
        }
    }

    world.spawn_batch(damage_commands_batch);
}
