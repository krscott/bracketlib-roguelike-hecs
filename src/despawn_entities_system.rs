use hecs::World;

use crate::components::DespawnCommand;

pub fn despawn_entities_system(world: &mut World) {
    let despawn_entities = world
        .query::<&DespawnCommand>()
        .into_iter()
        .map(|(_, cmd)| cmd.0)
        .collect::<Vec<_>>();

    for entity in despawn_entities {
        world.despawn(entity).unwrap();
    }
}
