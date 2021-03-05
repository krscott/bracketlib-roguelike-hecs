use bracket_lib::prelude::{BTerm, VirtualKeyCode};
use hecs::{Entity, World};

use crate::{
    command::{command_bundle, Command, InitiateAttackCommand},
    components::{CombatStats, Position, Viewshed},
    map::Map,
    RunState, State,
};

#[derive(Debug)]
pub struct Player;

impl Player {
    pub fn get_entity(world: &World) -> Option<Entity> {
        world
            .query::<&Player>()
            .into_iter()
            .next()
            .map(|(entity, _)| entity)
    }
}

/// Move the player if possible
fn try_move_player(world: &World, dx: i32, dy: i32) -> Vec<(Command, InitiateAttackCommand)> {
    if let Some((_, map)) = world.query::<&Map>().into_iter().next() {
        for (player_entity, (_player, pos, viewshed)) in world
            .query::<(&Player, &mut Position, &mut Viewshed)>()
            .into_iter()
        {
            let x = pos.x + dx;
            let y = pos.y + dy;

            for entity in map.get_entities_on_tile(x, y) {
                match world.get::<CombatStats>(*entity) {
                    Ok(_stats) => {
                        let attack_cmd_bundle = command_bundle(InitiateAttackCommand {
                            attacker: player_entity,
                            defender: *entity,
                        });

                        // TODO: Improve flow
                        return vec![attack_cmd_bundle];
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

    Vec::new()
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
            let attack_commands = try_move_player(&state.world, dx, dy);
            state.world.spawn_batch(attack_commands);
            RunState::PlayerTurn
        } else {
            RunState::AwaitingInput
        }
    } else {
        RunState::AwaitingInput
    }
}
