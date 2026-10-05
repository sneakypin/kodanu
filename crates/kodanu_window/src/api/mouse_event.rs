use crate::{Button, ButtonStatus};

use kodanu_ecs::Event;

#[derive(Event, Debug, Clone, Copy)]
pub struct MouseEvent {
    button: Button,
    state: ButtonStatus,
}

impl MouseEvent {
    pub fn new(button: Button, state: ButtonStatus) -> Self {
        Self { button, state }
    }
}

impl MouseEvent {
    pub fn button(&self) -> Button {
        self.button
    }

    pub fn state(&self) -> ButtonStatus {
        self.state
    }
}
