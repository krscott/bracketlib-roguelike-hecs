use hecs::World;

use crate::components::Command;

pub fn command_bundle<T>(component: T) -> (Command, T) {
    (Command, component)
}

pub fn clear_commands_system(world: &mut World) {
    let command_entities = world
        .query_mut::<&Command>()
        .into_iter()
        .map(|(entity, _)| entity)
        .collect::<Vec<_>>();

    for entity in command_entities {
        world.despawn(entity).unwrap();
    }
}
