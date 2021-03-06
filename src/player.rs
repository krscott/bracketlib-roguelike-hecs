use crate::prelude::*;

/// Check for player input and try to move Player entity
pub fn player_input(context: &mut BTerm, world: &mut World) -> anyhow::Result<RunState> {
    if let Some(key) = context.key {
        match key {
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::H => {
                try_move_player(world, -1, 0)
            }
            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::L => {
                try_move_player(world, 1, 0)
            }
            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::K => {
                try_move_player(world, 0, -1)
            }
            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::J => {
                try_move_player(world, 0, 1)
            }
            VirtualKeyCode::Numpad7 | VirtualKeyCode::Y => try_move_player(world, -1, -1),
            VirtualKeyCode::Numpad9 | VirtualKeyCode::U => try_move_player(world, 1, -1),
            VirtualKeyCode::Numpad1 | VirtualKeyCode::B => try_move_player(world, -1, 1),
            VirtualKeyCode::Numpad3 | VirtualKeyCode::N => try_move_player(world, 1, 1),
            VirtualKeyCode::Numpad5 | VirtualKeyCode::Period => try_move_player(world, 0, 0),
            VirtualKeyCode::G => try_pickup_item(world),
            VirtualKeyCode::I => Ok(RunState::ShowInventory),
            _ => Ok(RunState::AwaitingInput),
        }
    } else {
        Ok(RunState::AwaitingInput)
    }
}

/// Move the player if possible
fn try_move_player(world: &mut World, dx: i32, dy: i32) -> anyhow::Result<RunState> {
    let mut is_taking_turn = false;
    let mut attack_cmd_bundle = None;

    if let Some((_, map)) = world.query::<&TileMap>().into_iter().next() {
        'outer: for (player_entity, (_player, pos, viewshed)) in world
            .query::<(&Player, &mut Position, &mut Viewshed)>()
            .into_iter()
        {
            let x = pos.x + dx;
            let y = pos.y + dy;

            // TODO: Remove get_entities_on_tile call, use ECS query
            for entity in map.get_entities_on_tile(x, y) {
                match world.get::<CombatStats>(*entity) {
                    Ok(_stats) => {
                        attack_cmd_bundle = Some(InitiateAttackCommand {
                            attacker: player_entity,
                            defender: *entity,
                        });
                        is_taking_turn = true;

                        // TODO: Fix program flow
                        break 'outer;
                    }
                    Err(_) => {}
                }
            }

            if map.get_tile(x, y).is_some() && !map.is_tile_blocked(x, y) {
                pos.x = x;
                pos.y = y;
                viewshed.dirty = true;
                is_taking_turn = true;
            }

            break;
        }
    }

    if let Some(components) = attack_cmd_bundle {
        world.spawn_command(components);
    }

    Ok(if is_taking_turn {
        RunState::PlayerTurn
    } else {
        RunState::AwaitingInput
    })
}

fn try_pickup_item(world: &mut World) -> anyhow::Result<RunState> {
    let mut item_player_pair = None;

    'outer: for (player_entity, (_player, player_pos)) in
        world.query::<(&Player, &Position)>().into_iter()
    {
        for (item_entity, (_item, item_pos)) in world.query::<(&Item, &Position)>().into_iter() {
            if player_pos == item_pos {
                item_player_pair = Some((item_entity, player_entity));
                break 'outer;
            }
        }
    }

    match item_player_pair {
        Some((item_entity, player_entity)) => {
            world.spawn_command(PickupItemCommand {
                collector: player_entity,
                item: item_entity,
            });

            Ok(RunState::PlayerTurn)
        }
        None => {
            GameLog::resource_push(world, "There is nothing here to pick up.")?;

            Ok(RunState::AwaitingInput)
        }
    }
}
