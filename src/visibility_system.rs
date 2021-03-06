use crate::prelude::*;

pub fn visibility_system(world: &mut World) {
    let player_entity = world.resource_entity::<Player>().ok();

    for (entity, (viewshed, pos)) in world.query::<(&mut Viewshed, &Position)>().into_iter() {
        if viewshed.dirty {
            if let Some((_, map)) = world.query::<&mut TileMap>().into_iter().next() {
                update_viewshed(viewshed, pos, map);

                // If this is the player, reveal what they can see
                if Some(entity) == player_entity {
                    apply_viewshed_to_map(viewshed, map);
                }
            }
        }
    }
}

fn update_viewshed(viewshed: &mut Viewshed, pos: &Position, map: &TileMap) {
    viewshed.dirty = false;
    viewshed.visible_tiles.clear();
    viewshed.visible_tiles = field_of_view(pos.to_point(), viewshed.range, &*map);
    viewshed
        .visible_tiles
        .retain(|p| p.x >= 0 && p.x < map.get_width() && p.y >= 0 && p.y < map.get_height());
}

fn apply_viewshed_to_map(viewshed: &mut Viewshed, map: &mut TileMap) {
    map.clear_visible_tiles();
    for vis in viewshed.visible_tiles.iter() {
        map.set_tile_revealed(vis.x, vis.y);
        map.set_tile_visible(vis.x, vis.y);
    }
}
