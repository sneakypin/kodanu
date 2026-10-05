use {kodanu_ecs::Event, std::ops::Sub};

#[derive(Event, Default, Debug, Clone, Copy, PartialEq)]
pub struct MousePosition {
    x: f32,
    y: f32,
}

impl MousePosition {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl MousePosition {
    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }
}

impl Sub for MousePosition {
    type Output = MousePosition;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}
