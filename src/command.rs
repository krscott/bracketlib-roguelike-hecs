use hecs::{Entity, World};
use thiserror::Error;

use crate::{components::Command, despawn_entities_system};

#[derive(Debug, Error)]
#[error("Failed to find some entities")]
pub struct NoSuchEntities(Vec<Entity>);

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
