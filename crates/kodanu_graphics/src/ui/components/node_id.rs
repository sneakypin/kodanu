#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId {
    index: u32,
    generation: u32,
}

impl NodeId {
    pub const ROOT: Self = Self {
        index: 0,
        generation: 0,
    };
}

impl NodeId {
    pub fn new(index: u32, generation: u32) -> Self {
        Self { index, generation }
    }
}

impl NodeId {
    pub const fn index(self) -> u32 {
        self.index
    }

    pub const fn generation(self) -> u32 {
        self.generation
    }
}
