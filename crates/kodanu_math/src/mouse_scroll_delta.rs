#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct MouseScrollDelta {
    x: f32,
    y: f32,
}

impl MouseScrollDelta {
    pub const DEFAULT_MULTIPLE: f32 = 32.0;
}

impl MouseScrollDelta {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl MouseScrollDelta {
    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }
}
