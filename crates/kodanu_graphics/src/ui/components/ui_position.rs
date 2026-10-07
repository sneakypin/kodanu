#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct UiPosition {
    pub x: f32,
    pub y: f32,
}

impl UiPosition {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
