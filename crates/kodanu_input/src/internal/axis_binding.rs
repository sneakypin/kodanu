use kodanu_window::Key;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct AxisBinding {
    positive: Key,
    negative: Key,
}

impl AxisBinding {
    pub fn new(positive: Key, negative: Key) -> Self {
        Self { positive, negative }
    }
}

impl AxisBinding {
    pub fn positive(&self) -> Key {
        self.positive
    }

    pub fn negative(&self) -> Key {
        self.negative
    }
}
