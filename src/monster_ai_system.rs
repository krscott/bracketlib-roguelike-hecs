use bracket_lib::prelude::*;
use hecs::{Entity, World};

use crate::{
    command::command_bundle,
    components::{InitiateAttackCommand, Monster, Name, Position, Viewshed},
    map::Map,
    RunState,
};

pub fn monster_ai_system(
    world: &mut World,
    run_state_entity: Entity,
    player_entity: Entity,
    map_entity: Entity,
) {
    {
        let mut run_state = world.query_one::<&RunState>(run_state_entity).unwrap();
        let run_state = run_state.get().unwrap();
        if *run_state != RunState::AiTurn {
            return;
        }
    }

    let mut attack_cmd_batch = Vec::new();

    {
        let mut player_query = world
            .query_one::<(&Position, &Name)>(player_entity)
            .unwrap();
        let (player_pos, _player_name) = player_query.get().unwrap();

        // let player_pos = world.get::<Position>(player_entity).unwrap();
        let mut map = world.get_mut::<Map>(map_entity).unwrap();
        let player_pos_index = map.get_index(player_pos.x, player_pos.y).unwrap();

        for (monster_entity, (_, monster_viewshed, _monster_name, monster_pos)) in world
            .query::<(&Monster, &mut Viewshed, &Name, &mut Position)>()
            .into_iter()
        {
            if monster_viewshed
                .visible_tiles
                .contains(&player_pos.to_point())
            {
                let distance_to_player = DistanceAlg::Pythagoras
                    .distance2d(monster_pos.to_point(), player_pos.to_point());
                if distance_to_player < 1.5 {
                    attack_cmd_batch.push(command_bundle(InitiateAttackCommand {
                        attacker: monster_entity,
                        defender: player_entity,
                    }));
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

    world.spawn_batch(attack_cmd_batch);
}
