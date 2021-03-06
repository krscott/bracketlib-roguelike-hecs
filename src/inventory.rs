use crate::{despawn_entities_system::queue_despawn_batch, prelude::*};

pub fn pickup_item_system(world: &mut World) -> anyhow::Result<()> {
    let player = world.resource_entity::<Player>().ok();

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

        if Some(pickup_item_command.collector) == player {
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
    let player = world.resource_entity::<Player>().ok();

    let mut items_to_despawn = Vec::new();

    for (_, UseItemCommand { user, item }) in world.query::<&UseItemCommand>().into_iter() {
        let user = *user;
        let item = *item;

        let is_user_player = Some(user) == player;

        let mut item_name = match world.query_one::<&Name>(item) {
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

        if is_user_player {
            GameLog::resource_push(world, format!("You use the {}.", item_name))?;
        }

        if let Ok(mut stats) = world.get_mut::<CombatStats>(user) {
            if let Ok(healing_item) = world.get::<HealingItem>(item) {
                stats.hp = i32::min(stats.max_hp, stats.hp + healing_item.heal_amount);

                if is_user_player {
                    GameLog::resource_push(
                        world,
                        format!("It heals you for {} hp.", healing_item.heal_amount),
                    )?;
                }
            }
        }

        items_to_despawn.push(item);
    }

    queue_despawn_batch(world, items_to_despawn);

    Ok(())
}

pub fn drop_item_system(world: &mut World) -> anyhow::Result<()> {
    let player = world.resource_entity::<Player>().ok();

    let mut dropper_item_position = Vec::new();

    for (_, DropItemCommand { dropper, item }) in world.query::<&DropItemCommand>().into_iter() {
        let dropper = *dropper;
        let item = *item;

        let position = world.get::<Position>(dropper)?;

        dropper_item_position.push((dropper, item, *position));
    }

    for (dropper, item, position) in dropper_item_position {
        world.insert_one(item, position)?;

        if Some(dropper) == player {
            let name = world.get::<Name>(item)?;
            GameLog::resource_push(world, format!("You drop the {}.", name.as_str()))?;
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
