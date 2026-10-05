use crate::{Key, KeyStatus};

use kodanu_ecs::Event;

#[derive(Event, Debug, Clone, Copy)]
pub struct KeyboardEvent {
    key: Key,
    state: KeyStatus,
}

impl KeyboardEvent {
    pub fn new(key: Key, state: KeyStatus) -> Self {
        Self { key, state }
    }
}

impl KeyboardEvent {
    pub fn key(&self) -> Key {
        self.key
    }

    pub fn state(&self) -> KeyStatus {
        self.state
    }
}
