use bracket_lib::prelude::console;
use hecs::{Entity, World};

use crate::components::DespawnCommand;

pub fn despawn_entities_system(world: &mut World) {
    let entities = world
        .query::<&DespawnCommand>()
        .into_iter()
        .map(|(_, cmd)| cmd.0)
        .collect::<Vec<_>>();

    despawn_entities(world, entities);
}

pub fn despawn_entities(world: &mut World, entities: Vec<Entity>) {
    for entity in entities {
        if let Err(_) = world.despawn(entity) {
            console::log(format!("Tried to despawn missing entity: {}", entity.id()));
        }
    }
}
