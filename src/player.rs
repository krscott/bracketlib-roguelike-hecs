use bracket_lib::prelude::*;
use hecs::{Entity, World};

use crate::{
    components::{Player, Position, Viewshed},
    map::{self, TileType},
    RunState, State,
};

/// Move the player if possible
pub fn try_move_player(delta_x: i32, delta_y: i32, world: &mut World) {
    let map = map::query_map(world).unwrap();

    for (_, (_player, pos, viewshed)) in world
        .query::<(&Player, &mut Position, &mut Viewshed)>()
        .into_iter()
    {
        let x = pos.x + delta_x;
        let y = pos.y + delta_y;
        if let Some(tile) = map.get_tile(x, y) {
            match tile {
                TileType::Wall => {
                    // Do nothing
                }
                TileType::Floor => {
                    pos.x = x;
                    pos.y = y;
                    viewshed.dirty = true;
                }
            }
        }
    }
}

/// Check for player input and try to move Player entity
pub fn player_input(state: &mut State, context: &mut BTerm) -> RunState {
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
            try_move_player(dx, dy, &mut state.world);
            RunState::Running
        } else {
            RunState::Paused
        }
    } else {
        RunState::Paused
    }
}

pub fn query_player_entity(world: &World) -> Option<Entity> {
    world
        .query::<&Player>()
        .into_iter()
        .next()
        .map(|(entity, _)| entity)
}
