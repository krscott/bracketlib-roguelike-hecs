use hecs::{Entity, World};

use crate::{
    components::{BlocksTile, Position},
    map::Map,
};

pub fn map_indexing_system(world: &mut World, map_entity: Entity) {
    let mut map = world.get_mut::<Map>(map_entity).unwrap();
    map.reset_blocked_tiles();

    for (_, (_, position)) in world.query::<(&BlocksTile, &Position)>().into_iter() {
        map.set_tile_blocked(position.x, position.y, true);
    }
}
