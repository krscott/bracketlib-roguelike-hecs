use hecs::{Entity, World};

use crate::{
    components::{BlocksTile, Position},
    map::Map,
};

pub fn map_indexing_system(world: &mut World, map_entity: Entity) {
    let mut map = world.get_mut::<Map>(map_entity).unwrap();
    map.reset_blocked_tiles();
    map.clear_content_index();

    for (entity, (_, position)) in world.query::<(&BlocksTile, &Position)>().into_iter() {
        map.set_tile_blocked(position.x, position.y, true);
        map.add_entity_to_tile_content(position.x, position.y, entity);
    }
}
