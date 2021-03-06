use crate::prelude::*;

pub fn map_indexing_system(world: &World) {
    for (_, map) in world.query::<&mut TileMap>().into_iter() {
        map.reset_blocked_tiles();
        map.clear_content_index();

        for (_, (_, position)) in world.query::<(&BlocksTile, &Position)>().into_iter() {
            map.set_tile_blocked(position.x, position.y, true);
        }

        for (entity, position) in world.query::<&Position>().into_iter() {
            map.add_entity_to_tile_content(position.x, position.y, entity);
        }
    }
}
