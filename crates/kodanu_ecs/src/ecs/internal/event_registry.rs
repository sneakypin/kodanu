use crate::{Event, EventQueue, EventStorage};

use {std::any::TypeId, std::collections::HashMap};

#[derive(Default)]
pub(crate) struct EventRegistry {
    events: HashMap<TypeId, Box<dyn EventStorage>>,
}

impl EventRegistry {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            events: HashMap::with_capacity(capacity),
        }
    }
}

impl EventRegistry {
    pub fn push<E: Event>(&mut self, event: EventQueue<E>) {
        self.events.insert(TypeId::of::<E>(), Box::new(event));
    }

    #[allow(dead_code)]
    pub fn contains<E: Event>(&self) -> bool {
        self.events.contains_key(&TypeId::of::<E>())
    }

    pub fn get<E: Event>(&self) -> Option<&EventQueue<E>> {
        self.events
            .get(&TypeId::of::<E>())
            .and_then(|event| event.as_any().downcast_ref::<EventQueue<E>>())
    }

    pub fn get_mut<E: Event>(&mut self) -> Option<&mut EventQueue<E>> {
        self.events
            .get_mut(&TypeId::of::<E>())
            .and_then(|event| event.as_any_mut().downcast_mut::<EventQueue<E>>())
    }

    pub fn update(&mut self) {
        for event in self.events.values_mut() {
            event.update();
        }
    }
}
