use kodanu_ecs::Event;

#[derive(Event, Debug, Clone, Copy, PartialEq, Eq)]
pub struct SurfaceSize {
    width: u32,
    height: u32,
}

impl SurfaceSize {
    pub fn clamped(width: u32, height: u32) -> Self {
        Self {
            width: width.max(1),
            height: height.max(1),
        }
    }
}

impl SurfaceSize {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
