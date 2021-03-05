use hecs::{Entity, World};

use crate::despawn_entities_system;

#[derive(Debug)]
pub struct Command;

#[derive(Debug)]
pub struct InitiateAttackCommand {
    pub attacker: Entity,
    pub defender: Entity,
}

#[derive(Debug)]
pub struct DamageCommand {
    pub entity: Entity,
    pub amount: i32,
}

#[derive(Debug)]
pub struct DespawnCommand(pub Entity);

pub fn command_bundle<T>(component: T) -> (Command, T) {
    (Command, component)
}

pub fn clear_commands_system(world: &mut World) {
    let entities = world
        .query_mut::<&Command>()
        .into_iter()
        .map(|(entity, _)| entity)
        .collect::<Vec<_>>();

    despawn_entities_system::despawn_entities(world, entities);
}
