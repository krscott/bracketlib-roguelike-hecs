use std::fmt::Display;

use hecs::{Component, ComponentError, DynamicBundle, Entity, NoSuchEntity, RefMut, World};
use thiserror::Error;

#[derive(Debug, Error)]
pub struct ResourceTypeAlreadyExists;

impl Display for ResourceTypeAlreadyExists {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Resource type already exists"))
    }
}

#[derive(Debug)]
struct ResourceInner;

pub fn spawn<T: Component>(
    world: &mut World,
    resource_component: T,
    other_components: impl DynamicBundle,
) -> Result<Entity, ResourceTypeAlreadyExists> {
    for (_, _) in world.query::<&T>().into_iter() {
        return Err(ResourceTypeAlreadyExists);
    }

    let entity = world.spawn((ResourceInner, resource_component));

    world.insert(entity, other_components).unwrap();

    Ok(entity)
}

pub fn get<T: Component>(world: &World) -> Result<Entity, NoSuchEntity> {
    for (entity, _) in world.query::<(&ResourceInner, &T)>().into_iter() {
        return Ok(entity);
    }

    Err(NoSuchEntity)
}

// pub fn get_component_clone<T: Component + Clone>(world: &World) -> Result<T, NoSuchEntity> {
//     for (_, (_, component)) in world.query::<(&Resource, &T)>().into_iter() {
//         return Ok(component.clone());
//     }

//     Err(NoSuchEntity)
// }

pub fn map<T: Component, F, R>(world: &mut World, mut op: F) -> Result<R, ComponentError>
where
    F: FnMut(RefMut<T>) -> R,
{
    let entity = get::<T>(world)?;

    let component = world.get_mut::<T>(entity)?;

    Ok(op(component))
}
