use crate::prelude::*;

pub fn pickup_item_system(world: &mut World) -> anyhow::Result<()> {
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

pub fn use_item_system(world: &mut World) -> anyhow::Result<()> {
    let player_entity = world.resource_entity::<Player>().ok();

    for (_, UseItemCommand { user, item }) in world.query::<&UseItemCommand>().into_iter() {
        let mut item_name = match world.query_one::<&Name>(*item) {
            Ok(query) => query,
            Err(err) => {
                console::log(format!(
                    "Error: Failed to get item {} name: {}",
                    item.id(),
                    err
                ));
                continue;
            }
        };

        let Name(item_name) = item_name.get().expect("Unfiltered query");

        if Some(*user) == player_entity {
            GameLog::resource_push(
                world,
                format!("You want to use the {}, but don't know how!", item_name),
            )?;
        }
    }

    Ok(())
}

pub fn get_inventory_list(world: &World, owner: Entity) -> Vec<(Entity, String)> {
    let mut inventory = world.query::<(&InInventory, &Name)>();
    inventory
        .into_iter()
        .filter(|(_, (in_inventory, _))| in_inventory.owner == owner)
        .map(|(entity, (_, Name(name)))| (entity, name.to_string()))
        .collect::<Vec<_>>()
}
