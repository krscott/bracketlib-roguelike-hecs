use bracket_lib::prelude::console;
use hecs::{Component, Entity, World};

pub trait WorldCommands {
    fn spawn_command<T: Component>(&mut self, component: T) -> Entity;

    fn spawn_batch_commands<T: Component, I>(&mut self, bundle: I)
    where
        I: IntoIterator<Item = T>;

    fn clear_commands(&mut self);
}

impl WorldCommands for World {
    fn spawn_command<T: Component>(&mut self, component: T) -> Entity {
        self.spawn((Command, component))
    }

    fn spawn_batch_commands<T: Component, I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        self.spawn_batch(iter.into_iter().map(|c| (Command, c)));
    }

    fn clear_commands(&mut self) {
        let entities = self
            .query_mut::<&Command>()
            .into_iter()
            .map(|(entity, _)| entity)
            .collect::<Vec<_>>();

        for entity in entities {
            if let Err(_) = self.despawn(entity) {
                console::log(format!("Tried to despawn missing entity: {}", entity.id()));
            }
        }
    }
}

#[derive(Debug)]
struct Command;

#[derive(Debug)]
pub struct InitiateAttackCommand {
    pub attacker: Entity,
    pub defender: Entity,
}

#[derive(Debug)]
pub struct DamageCommand {
    pub entity: Entity,
    pub amount: i32,
}

#[derive(Debug)]
pub struct DespawnCommand(pub Entity);
