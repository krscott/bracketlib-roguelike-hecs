use bracket_lib::prelude::*;
use hecs::{Entity, World};

use crate::components::{Monster, Name, Position, Viewshed};

pub fn monster_ai_system(world: &mut World, player_entity: Entity) {
    // let player_entity = player::query_player_entity(world).unwrap();
    let player_pos = world.get::<Position>(player_entity).unwrap();
    let player_name = world.get::<Name>(player_entity).unwrap();

    for (_, (_, monster_viewshed, Name(monster_name))) in
        world.query::<(&Monster, &Viewshed, &Name)>().into_iter()
    {
        if monster_viewshed
            .visible_tiles
            .contains(&Point::new(player_pos.x, player_pos.y))
        {
            console::log(&format!(
                "{} shouts insults at {}",
                monster_name, player_name.0
            ));
        }
    }
}
