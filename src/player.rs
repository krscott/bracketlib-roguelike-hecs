use bracket_lib::prelude::*;
use hecs::{Entity, World};

use crate::{
    components::{CombatStats, Player, Position, Viewshed, WantsToMelee},
    map::Map,
    RunState, State,
};

/// Move the player if possible
fn try_move_player(world: &mut World, player_entity: Entity, map_entity: Entity, dx: i32, dy: i32) {
    let mut wants_to_melee = None;

    {
        let map = world.get::<Map>(map_entity).unwrap();

        for (_, (_player, pos, viewshed)) in world
            .query::<(&Player, &mut Position, &mut Viewshed)>()
            .into_iter()
        {
            let x = pos.x + dx;
            let y = pos.y + dy;

            for entity in map.get_entities_on_tile(x, y) {
                match world.get::<CombatStats>(*entity) {
                    Ok(_stats) => {
                        // console::log(&format!("From Hell's Heart, I stab thee!"));
                        wants_to_melee = Some(WantsToMelee { target: *entity });

                        // TODO: Improve flow
                        break;
                    }
                    Err(_) => {}
                }
            }

            if map.get_tile(x, y).is_some() && !map.is_tile_blocked(x, y) {
                pos.x = x;
                pos.y = y;
                viewshed.dirty = true;
            }
        }
    }

    if let Some(wants_to_melee) = wants_to_melee {
        world.insert_one(player_entity, wants_to_melee).unwrap();
    }
}

/// Check for player input and try to move Player entity
pub fn player_input(
    state: &mut State,
    context: &mut BTerm,
    player_entity: Entity,
    map_entity: Entity,
) -> RunState {
    if let Some(key) = context.key {
        let delta_xy = match key {
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::H => Some((-1, 0)),
            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::L => Some((1, 0)),
            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::K => Some((0, -1)),
            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::J => Some((0, 1)),
            VirtualKeyCode::Numpad7 | VirtualKeyCode::Y => Some((-1, -1)),
            VirtualKeyCode::Numpad9 | VirtualKeyCode::U => Some((1, -1)),
            VirtualKeyCode::Numpad1 | VirtualKeyCode::B => Some((-1, 1)),
            VirtualKeyCode::Numpad3 | VirtualKeyCode::N => Some((1, 1)),
            VirtualKeyCode::Numpad5 | VirtualKeyCode::Period => Some((0, 0)),
            _ => None,
        };

        if let Some((dx, dy)) = delta_xy {
            try_move_player(&mut state.world, player_entity, map_entity, dx, dy);
            RunState::Running
        } else {
            RunState::Paused
        }
    } else {
        RunState::Paused
    }
}
