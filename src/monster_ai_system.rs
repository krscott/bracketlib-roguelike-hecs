use bracket_lib::prelude::*;
use hecs::{Entity, World};

use crate::{
    components::{Monster, Name, Position, Viewshed},
    map::Map,
};

pub fn monster_ai_system(world: &mut World, player_entity: Entity, map_entity: Entity) {
    // let player_entity = player::query_player_entity(world).unwrap();
    let player_pos = world.get::<Position>(player_entity).unwrap();
    let player_name = world.get::<Name>(player_entity).unwrap();
    let mut map = world.get_mut::<Map>(map_entity).unwrap();
    let player_pos_index = map.get_index(player_pos.x, player_pos.y).unwrap();

    for (_, (_, monster_viewshed, Name(monster_name), monster_pos)) in world
        .query::<(&Monster, &mut Viewshed, &Name, &mut Position)>()
        .into_iter()
    {
        if monster_viewshed
            .visible_tiles
            .contains(&Point::new(player_pos.x, player_pos.y))
        {
            let distance_to_player = DistanceAlg::Pythagoras.distance2d(
                Point::new(monster_pos.x, monster_pos.y),
                Point::new(player_pos.x, player_pos.y),
            );
            if distance_to_player < 1.5 {
                //TODO: Attack

                console::log(&format!(
                    "{} shouts insults at {}",
                    monster_name, player_name.0
                ));
            } else {
                let path = a_star_search(
                    map.get_index(monster_pos.x, monster_pos.y).unwrap(),
                    player_pos_index,
                    &mut *map,
                );

                if path.success {
                    if let Some(i) = path.steps.get(1) {
                        let (next_x, next_y) = map.get_coords(*i);
                        map.set_tile_blocked(monster_pos.x, monster_pos.y, false);
                        monster_pos.x = next_x;
                        monster_pos.y = next_y;
                        monster_viewshed.dirty = true;
                        map.set_tile_blocked(monster_pos.x, monster_pos.y, true);
                    }
                }
            }
        }
    }
}
