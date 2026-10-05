#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowSize {
    width: u32,
    height: u32,
}

impl Default for WindowSize {
    fn default() -> Self {
        Self {
            width: 640,
            height: 520,
        }
    }
}

impl WindowSize {
    pub fn clamped(width: u32, height: u32) -> Self {
        Self {
            width: width.max(1),
            height: height.max(1),
        }
    }
}

impl WindowSize {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
