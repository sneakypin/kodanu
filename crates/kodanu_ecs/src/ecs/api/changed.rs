use crate::{Component, QueryFilter, WorldCell};

use {std::collections::HashSet, std::marker::PhantomData};

pub struct Changed<C>(PhantomData<fn() -> C>);

impl<C> Default for Changed<C> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<C: Component> QueryFilter for Changed<C> {
    type Storage<'w> = HashSet<u32>;

    fn fetch<'w>(cell: WorldCell<'w>) -> Self::Storage<'w> {
        let tick = cell.change_tick();
        let mut entities = HashSet::new();

        if let Some(storage) = cell.storage::<C>() {
            for &entity in storage.indices() {
                if storage.changed_tick(entity) == Some(tick) {
                    entities.insert(entity);
                }
            }
        }

        entities
    }

    fn matches(storage: &Self::Storage<'_>, entity: u32) -> bool {
        storage.contains(&entity)
    }
}
