use crate::prelude::*;

#[derive(Debug)]
struct DespawnCommand(pub Entity);

pub fn despawn_entities_system(world: &mut World) {
    let entities = world
        .query::<&DespawnCommand>()
        .into_iter()
        .map(|(_, cmd)| cmd.0)
        .collect::<Vec<_>>();

    despawn_entities(world, entities);
}

fn despawn_entities(world: &mut World, entities: Vec<Entity>) {
    for entity in entities {
        if let Err(_) = world.despawn(entity) {
            console::log(format!("Tried to despawn missing entity: {}", entity.id()));
        }
    }
}

// pub fn queue_despawn(world: &mut World, entity: Entity) {
//     world.spawn_command(DespawnCommand(entity));
// }

pub fn queue_despawn_batch<I>(world: &mut World, iter: I)
where
    I: IntoIterator<Item = Entity>,
{
    world.spawn_batch_commands(iter.into_iter().map(|entity| DespawnCommand(entity)));
}
