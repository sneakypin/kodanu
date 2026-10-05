use crate::{QueryFilter, QueryStorage};

use std::{iter::FusedIterator, marker::PhantomData};

pub struct QueryIter<'w, S, F>
where
    S: QueryStorage<'w>,
    F: QueryFilter,
{
    storage: S,
    filter: F::Storage<'w>,
    dense: usize,
    marker: PhantomData<&'w ()>,
}

impl<'w, S, F> QueryIter<'w, S, F>
where
    S: QueryStorage<'w>,
    F: QueryFilter,
{
    pub fn new(storage: S, filter: F::Storage<'w>) -> Self {
        Self {
            storage,
            filter,
            dense: 0,
            marker: PhantomData,
        }
    }
}

impl<'w, S, F> Iterator for QueryIter<'w, S, F>
where
    S: QueryStorage<'w>,
    F: QueryFilter,
{
    type Item = S::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while self.dense < self.storage.len() {
            let dense = self.dense;
            self.dense += 1;

            let entity = self.storage.entity(dense);

            if !F::matches(&self.filter, entity) {
                continue;
            }

            if let Some(item) = self.storage.get(dense) {
                return Some(item);
            }
        }

        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.storage.len().saturating_sub(self.dense)))
    }
}

impl<'w, S, F> FusedIterator for QueryIter<'w, S, F>
where
    S: QueryStorage<'w>,
    F: QueryFilter,
{
}
