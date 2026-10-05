use crate::{Event, EventQueue, SystemParam, WorldCell};

pub struct EventWriter<'w, E: Event> {
    value: &'w mut EventQueue<E>,
}

impl<'w, E: Event> EventWriter<'w, E> {
    pub fn new(value: &'w mut EventQueue<E>) -> Self {
        Self { value }
    }
}

impl<'w, E: Event> EventWriter<'w, E> {
    pub fn send(&mut self, event: E) {
        self.value.send(event);
    }

    pub fn get(&mut self) -> &mut EventQueue<E> {
        self.value
    }
}

impl<E: Event> SystemParam for EventWriter<'_, E> {
    type Item<'w> = EventWriter<'w, E>;

    fn fetch<'w>(cell: WorldCell<'w>) -> Self::Item<'w> {
        EventWriter::new(cell.expect_mut_event::<E>())
    }
}
