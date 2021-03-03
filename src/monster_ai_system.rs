use bracket_lib::prelude::*;
use specs::prelude::*;

use crate::components::{Monster, Player, Position, Viewshed};

pub struct MonsterAi;

impl<'a> System<'a> for MonsterAi {
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player, position, viewshed, monster) = data;

        if let Some((_player, player_pos)) = (&player, &position).join().next() {
            for (_monster, monster_viewshed) in (&monster, &viewshed).join() {
                if monster_viewshed
                    .visible_tiles
                    .contains(&Point::new(player_pos.x, player_pos.y))
                {
                    console::log("Monster shouts insults");
                }
            }
        }
    }
}
