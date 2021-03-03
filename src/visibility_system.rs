use bracket_lib::prelude::{field_of_view, Point};
use hecs::World;

use crate::{
    components::{Position, Viewshed},
    map::{self, Map},
    player,
};

pub fn visibility_system(world: &mut World) {
    let player_entity = player::query_player_entity(world).unwrap();
    let map_entity = map::query_map_entity(world).unwrap();

    for (entity, (viewshed, pos)) in world.query::<(&mut Viewshed, &Position)>().into_iter() {
        if viewshed.dirty {
            let mut map = world.get_mut::<Map>(map_entity).unwrap();
            viewshed.dirty = false;
            viewshed.visible_tiles.clear();
            viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
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
