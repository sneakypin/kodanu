use crate::{Component, QueryError, QueryStorage, SparseSet};

use std::{marker::PhantomData, ptr::NonNull};

pub struct WriteStorage<'w, C: Component> {
    storage: Option<NonNull<SparseSet<C>>>,
    marker: PhantomData<&'w mut SparseSet<C>>,
}

impl<'w, C: Component> From<Option<&'w mut SparseSet<C>>> for WriteStorage<'w, C> {
    fn from(value: Option<&'w mut SparseSet<C>>) -> Self {
        Self {
            storage: value.map(NonNull::from),
            marker: PhantomData,
        }
    }
}

impl<'w, C: Component> QueryStorage<'w> for WriteStorage<'w, C> {
    type Item = &'w mut C;

    fn len(&self) -> usize {
        self.storage
            .map(|storage| unsafe { storage.as_ref().dense().len() })
            .unwrap_or(0)
    }

    fn entity(&self, dense: usize) -> u32 {
        unsafe {
            *self
                .storage
                .unwrap_or_else(|| panic!("{}", QueryError::StorageMissing))
                .as_ref()
                .indices()
                .get_unchecked(dense)
        }
    }

    fn contains(&self, entity: u32) -> bool {
        unsafe {
            self.storage
                .is_some_and(|storage| storage.as_ref().contains(entity))
        }
    }

    fn get(&mut self, dense: usize) -> Option<Self::Item> {
        Some(unsafe { &mut *self.storage?.as_mut().dense_mut().as_mut_ptr().add(dense) })
    }

    fn get_by_entity(&mut self, entity: u32) -> Option<Self::Item> {
        let mut storage = self.storage?;

        let dense = unsafe { storage.as_ref().dense_index(entity)? };

        Some(unsafe { &mut *storage.as_mut().dense_mut().as_mut_ptr().add(dense) })
    }
}
