#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowId(usize);

impl WindowId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}

impl WindowId {
    pub fn id(&self) -> usize {
        self.0
    }
}
