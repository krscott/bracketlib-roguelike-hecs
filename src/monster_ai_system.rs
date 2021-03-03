use bracket_lib::prelude::*;
use specs::prelude::*;

use crate::components::{Monster, Name, Player, Position, Viewshed};

pub struct MonsterAi;

impl<'a> System<'a> for MonsterAi {
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player, position, viewshed, monster, name) = data;

        if let Some((_player, player_pos, Name(player_name))) =
            (&player, &position, &name).join().next()
        {
            for (_monster, monster_viewshed, Name(monster_name)) in
                (&monster, &viewshed, &name).join()
            {
                if monster_viewshed
                    .visible_tiles
                    .contains(&Point::new(player_pos.x, player_pos.y))
                {
                    console::log(&format!(
                        "{} shouts insults at {}",
                        monster_name, player_name
                    ));
                }
            }
        }
    }
}
