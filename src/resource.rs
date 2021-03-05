use std::{fmt::Display, marker::PhantomData};

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
struct ResourceInnerComponent;

pub struct Resource<'a, T: Component> {
    entity: Entity,
    world: &'a World,
    __phantom_data: PhantomData<T>,
}

impl<'a, T: Component> Resource<'a, T> {
    pub fn map<F, R>(&mut self, op: F) -> Result<R, ComponentError>
    where
        F: FnOnce(RefMut<T>) -> R,
    {
        let component = self.world.get_mut::<T>(self.entity)?;

        Ok(op(component))
    }

    pub fn entity(&self) -> Entity {
        self.entity
    }
}

// pub struct ResourceMut<'a, T: Component> {
//     entity: Entity,
//     world: &'a mut World,
//     __phantom_data: PhantomData<T>,
// }

// impl<'a, T: Component> ResourceMut<'a, T> {
//     pub fn map<F, R>(&mut self, mut op: F) -> Result<R, ComponentError>
//     where
//         F: FnMut(RefMut<T>) -> R,
//     {
//         let component = self.world.get_mut::<T>(self.entity)?;

//         Ok(op(component))
//     }

//     pub fn entity(&self) -> Entity {
//         self.entity
//     }
// }

pub trait WorldResources {
    fn spawn_resource<T: Component>(
        &mut self,
        resource_component: T,
        other_components: impl DynamicBundle,
    ) -> Result<Entity, ResourceTypeAlreadyExists>;

    fn resource<'a, T: Component>(&'a self) -> Result<Resource<'a, T>, NoSuchEntity>;

    // fn resource_mut<'a, T: Component>(&'a mut self) -> Result<Resource<'a, T>, NoSuchEntity>;

    fn resource_entity<T: Component>(&self) -> Result<Entity, NoSuchEntity>;

    fn resource_clone<T: Component + Clone>(&self) -> Result<T, NoSuchEntity>;
}

impl WorldResources for World {
    fn spawn_resource<T: Component>(
        &mut self,
        resource_component: T,
        other_components: impl DynamicBundle,
    ) -> Result<Entity, ResourceTypeAlreadyExists> {
        spawn(self, resource_component, other_components)
    }

    fn resource<'a, T: Component>(&'a self) -> Result<Resource<'a, T>, NoSuchEntity> {
        let entity = query::<T>(self)?;

        Ok(Resource {
            entity,
            world: self,
            __phantom_data: PhantomData,
        })
    }

    // fn resource_mut<'a, T: Component>(&'a mut self) -> Result<Resource<'a, T>, NoSuchEntity> {
    //     let entity = query::<T>(self)?;

    //     Ok(ResourceMut {
    //         entity,
    //         world: self,
    //         __phantom_data: PhantomData,
    //     })
    // }

    fn resource_entity<T: Component>(&self) -> Result<Entity, NoSuchEntity> {
        Ok(self.resource::<T>()?.entity())
    }

    fn resource_clone<T: Component + Clone>(&self) -> Result<T, NoSuchEntity> {
        get_component_clone::<T>(self)
    }
}

fn spawn<T: Component>(
    world: &mut World,
    resource_component: T,
    other_components: impl DynamicBundle,
) -> Result<Entity, ResourceTypeAlreadyExists> {
    for (_, _) in world.query::<&T>().into_iter() {
        return Err(ResourceTypeAlreadyExists);
    }

    let entity = world.spawn((ResourceInnerComponent, resource_component));

    world.insert(entity, other_components).unwrap();

    Ok(entity)
}

fn query<T: Component>(world: &World) -> Result<Entity, NoSuchEntity> {
    for (entity, _) in world.query::<(&ResourceInnerComponent, &T)>().into_iter() {
        return Ok(entity);
    }

    Err(NoSuchEntity)
}

// fn query_mut<T: Component>(world: &mut World) -> Result<Entity, NoSuchEntity> {
//     for (entity, _) in world.query::<(&ResourceInnerComponent, &T)>().into_iter() {
//         return Ok(entity);
//     }

//     Err(NoSuchEntity)
// }

pub fn get_component_clone<T: Component + Clone>(world: &World) -> Result<T, NoSuchEntity> {
    for (_, (_, component)) in world.query::<(&ResourceInnerComponent, &T)>().into_iter() {
        return Ok(component.clone());
    }

    Err(NoSuchEntity)
}

// pub fn map<T: Component, F, R>(world: &mut World, mut op: F) -> Result<R, ComponentError>
// where
//     F: FnMut(RefMut<T>) -> R,
// {
//     let entity = query::<T>(world)?;

//     let component = world.get_mut::<T>(entity)?;

//     Ok(op(component))
// }
