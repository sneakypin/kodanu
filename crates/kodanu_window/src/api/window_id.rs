#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowId(u32);

impl WindowId {
    pub fn new(id: u32) -> Self {
        Self(id)
    }
}

impl WindowId {
    pub fn id(&self) -> u32 {
        self.0
    }
}
