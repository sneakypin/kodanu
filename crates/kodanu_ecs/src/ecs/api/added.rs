use crate::{Component, QueryFilter, SparseSet, Tick, WorldCell};

use std::marker::PhantomData;

pub struct Added<C>(PhantomData<fn() -> C>);

impl<C> Default for Added<C> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<C: Component> QueryFilter for Added<C> {
    type Storage<'w> = (Option<&'w SparseSet<C>>, Tick);

    fn fetch<'w>(cell: WorldCell<'w>) -> Self::Storage<'w> {
        (cell.storage::<C>(), cell.change_tick())
    }

    fn matches(storage: &Self::Storage<'_>, entity: u32) -> bool {
        let (Some(set), tick) = storage else {
            return false;
        };

        set.added_tick(entity) == Some(*tick)
    }
}
