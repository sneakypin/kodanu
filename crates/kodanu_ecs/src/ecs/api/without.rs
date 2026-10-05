use std::marker::PhantomData;

use crate::{Component, QueryFilter, SparseSet, WorldCell};

pub struct Without<C>(PhantomData<fn() -> C>);

impl<C: Component> QueryFilter for Without<C> {
    type Storage<'w> = Option<&'w SparseSet<C>>;

    fn fetch<'w>(cell: WorldCell<'w>) -> Self::Storage<'w> {
        cell.storage::<C>()
    }

    fn matches(storage: &Self::Storage<'_>, entity: u32) -> bool {
        storage.is_none_or(|storage| !storage.contains(entity))
    }
}
