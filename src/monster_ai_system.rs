use bracket_lib::prelude::{a_star_search, DistanceAlg};
use hecs::{Entity, World};

use crate::{
    command::{command_bundle, Command, InitiateAttackCommand},
    components::{Monster, Name, Position, Viewshed},
    map::Map,
    player::Player,
    RunState,
};

pub fn monster_ai_system(world: &mut World) {
    match RunState::from_world(world) {
        Some(RunState::AiTurn) => {}
        _ => return,
    }

    let mut attack_cmd_batch = Vec::new();

    {
        if let Some((player_entity, (_, player_pos, _player_name))) = world
            .query::<(&Player, &Position, &Name)>()
            .into_iter()
            .next()
        {
            if let Some((_, mut map)) = world.query::<&mut Map>().into_iter().next() {
                if let Some(player_pos_index) = map.get_index(player_pos.x, player_pos.y) {
                    for (monster_entity, (_, monster_viewshed, _monster_name, monster_pos)) in world
                        .query::<(&Monster, &mut Viewshed, &Name, &mut Position)>()
                        .into_iter()
                    {
                        monster_ai_to_player(
                            &mut attack_cmd_batch,
                            &mut map,
                            monster_entity,
                            monster_pos,
                            monster_viewshed,
                            player_entity,
                            player_pos,
                            player_pos_index,
                        )
                    }
                }
            }
        }
    }

    world.spawn_batch(attack_cmd_batch);
}

fn monster_ai_to_player(
    attack_cmd_batch: &mut Vec<(Command, InitiateAttackCommand)>,
    map: &mut Map,
    monster_entity: Entity,
    monster_pos: &mut Position,
    monster_viewshed: &mut Viewshed,
    player_entity: Entity,
    player_pos: &Position,
    player_pos_index: usize,
) {
    if monster_viewshed
        .visible_tiles
        .contains(&player_pos.to_point())
    {
        let distance_to_player =
            DistanceAlg::Pythagoras.distance2d(monster_pos.to_point(), player_pos.to_point());
        if distance_to_player < 1.5 {
            attack_cmd_batch.push(command_bundle(InitiateAttackCommand {
                attacker: monster_entity,
                defender: player_entity,
            }));
        } else {
            if let Some(start) = map.get_index(monster_pos.x, monster_pos.y) {
                let nav = a_star_search(start, player_pos_index, &mut *map);

                if nav.success {
                    if let Some(i) = nav.steps.get(1) {
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
