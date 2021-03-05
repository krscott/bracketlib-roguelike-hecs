use bracket_lib::prelude::console;
use hecs::{Entity, World};

use crate::{
    components::{Name, Position},
    gamelog::GameLog,
    player::Player,
    resource::WorldResources,
};

#[derive(Debug)]
pub struct InInventory {
    pub owner: Entity,
}

#[derive(Debug, Clone, Copy)]
pub struct PickupItemCommand {
    pub collector: Entity,
    pub item: Entity,
}

pub fn inventory_system(world: &mut World) -> anyhow::Result<()> {
    let player_entity = world.resource_entity::<Player>().ok();

    let pickup_commands = world
        .query::<&PickupItemCommand>()
        .into_iter()
        .map(|(_, cmd)| *cmd)
        .collect::<Vec<_>>();

    for pickup_item_command in pickup_commands {
        if let Err(err) = world.remove_one::<Position>(pickup_item_command.item) {
            console::log(format!(
                "Error: Failed to remove Position from item {}: {}",
                pickup_item_command.item.id(),
                err
            ));

            continue;
        }

        if let Err(err) = world.insert_one(
            pickup_item_command.item,
            InInventory {
                owner: pickup_item_command.collector,
            },
        ) {
            console::log(format!(
                "Error: Failed to add InInventory to item {}: {}",
                pickup_item_command.item.id(),
                err
            ));

            continue;
        }

        if Some(pickup_item_command.collector) == player_entity {
            match world.get::<Name>(pickup_item_command.item) {
                Ok(item_name) => {
                    GameLog::resource_push(
                        world,
                        format!("You pick up the {}.", item_name.as_str()),
                    )?;
                }
                Err(err) => {
                    console::log(format!(
                        "Error: Failed to log pickup message for item entity {}: {}",
                        pickup_item_command.item.id(),
                        err
                    ));
                }
            }
        }
    }

    Ok(())
}
