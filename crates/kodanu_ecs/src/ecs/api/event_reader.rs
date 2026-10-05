use crate::{Event, EventQueue, SystemParam, WorldCell};

pub struct EventReader<'w, E: Event> {
    value: &'w EventQueue<E>,
}

impl<'w, E: Event> EventReader<'w, E> {
    pub fn new(value: &'w EventQueue<E>) -> Self {
        Self { value }
    }
}

impl<'w, E: Event> EventReader<'w, E> {
    pub fn iter(&self) -> impl Iterator<Item = &E> {
        self.value.iter()
    }

    pub fn get(&self) -> &EventQueue<E> {
        self.value
    }
}

impl<E: Event> SystemParam for EventReader<'_, E> {
    type Item<'w> = EventReader<'w, E>;

    fn fetch<'w>(cell: WorldCell<'w>) -> Self::Item<'w> {
        EventReader::new(cell.expect_event::<E>())
    }
}
