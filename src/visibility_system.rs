use bracket_lib::prelude::field_of_view;
use hecs::{Entity, World};

use crate::{
    components::{Position, Viewshed},
    map::Map,
};

pub fn visibility_system(world: &mut World, player_entity: Entity, map_entity: Entity) {
    for (entity, (viewshed, pos)) in world.query::<(&mut Viewshed, &Position)>().into_iter() {
        if viewshed.dirty {
            let mut map = world.get_mut::<Map>(map_entity).unwrap();
            viewshed.dirty = false;
            viewshed.visible_tiles.clear();
            viewshed.visible_tiles = field_of_view(pos.to_point(), viewshed.range, &*map);
            viewshed.visible_tiles.retain(|p| {
                p.x >= 0 && p.x < map.get_width() && p.y >= 0 && p.y < map.get_height()
            });

            // If this is the player, reveal what they can see
            if entity == player_entity {
                map.clear_visible_tiles();
                for vis in viewshed.visible_tiles.iter() {
                    map.set_tile_revealed(vis.x, vis.y);
                    map.set_tile_visible(vis.x, vis.y);
                }
            }
        }
    }
}