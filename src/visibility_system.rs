use bracket_lib::prelude::{field_of_view, Point};
use specs::prelude::*;

use crate::{
    components::{Player, Position, Viewshed},
    map::Map,
};

pub struct VisibilitySystem;

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, mut viewshed, pos, player) = data;

        for (ent, viewshed, pos) in (&entities, &mut viewshed, &pos).join() {
            if viewshed.dirty {
                viewshed.dirty = false;
                viewshed.visible_tiles.clear();
                viewshed.visible_tiles =
                    field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
                viewshed.visible_tiles.retain(|p| {
                    p.x >= 0 && p.x < map.get_width() && p.y >= 0 && p.y < map.get_height()
                });

                // If this is the player, reveal what they can see
                if player.get(ent).is_some() {
                    map.clear_visible_tiles();
                    for vis in viewshed.visible_tiles.iter() {
                        map.set_tile_revealed(vis.x, vis.y);
                        map.set_tile_visible(vis.x, vis.y);
                    }
                }
            }
        }
    }
}
