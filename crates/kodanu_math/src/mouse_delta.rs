use crate::MousePosition;

use std::ops::AddAssign;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct MouseDelta {
    x: f32,
    y: f32,
}

impl MouseDelta {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl MouseDelta {
    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }
}

impl AddAssign for MouseDelta {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl From<MousePosition> for MouseDelta {
    fn from(value: MousePosition) -> Self {
        Self::new(value.x(), value.y())
    }
}
