use bracket_lib::prelude::*;
use specs::prelude::*;

use crate::{
    components::{Player, Position},
    map::{self, GameMap, TileType},
    State,
};

/// Move the player if possible
pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<GameMap>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let new_pos_index = map::get_map_index(pos.x + delta_x, pos.y + delta_y);
        match map[new_pos_index] {
            TileType::Wall => {
                // Do nothing
            }
            TileType::Floor => {
                let (x, y) = map::get_map_coords(new_pos_index);
                pos.x = x;
                pos.y = y;
            }
        }
    }
}

/// Check for player input and try to move Player entity
pub fn player_input(state: &mut State, context: &mut BTerm) {
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
            _ => None,
        };

        if let Some((dx, dy)) = delta_xy {
            try_move_player(dx, dy, &mut state.ecs);
        }
    }
}
