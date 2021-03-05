use hecs::World;

use crate::{
    components::{BlocksTile, Position},
    map::Map,
    with_query,
};

pub fn map_indexing_system(world: &World) {
    with_query::<&mut Map, _>(world, |_, map| {
        map.reset_blocked_tiles();
        map.clear_content_index();

        for (entity, (_, position)) in world.query::<(&BlocksTile, &Position)>().into_iter() {
            map.set_tile_blocked(position.x, position.y, true);
            map.add_entity_to_tile_content(position.x, position.y, entity);
        }
    });

    // if let Some((_, map)) = world.query::<&mut Map>().into_iter().next() {
    //     map.reset_blocked_tiles();
    //     map.clear_content_index();

    //     for (entity, (_, position)) in world.query::<(&BlocksTile, &Position)>().into_iter() {
    //         map.set_tile_blocked(position.x, position.y, true);
    //         map.add_entity_to_tile_content(position.x, position.y, entity);
    //     }
    // }
}
