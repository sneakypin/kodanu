use {kodanu_ecs::Event, kodanu_math::MouseScroll};

#[derive(Event, Debug, Clone, Copy)]
pub struct MouseWheelEvent {
    scroll: MouseScroll,
}

impl MouseWheelEvent {
    pub fn delta(&self) -> MouseScroll {
        self.scroll
    }
}

impl From<MouseScroll> for MouseWheelEvent {
    fn from(value: MouseScroll) -> Self {
        Self { scroll: value }
    }
}
