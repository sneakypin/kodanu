/// Represents a mouse scroll offset.
///
/// The meaning of the values depends on the [`crate::MouseScroll`] variant
/// that produced the delta.
///
/// Positive and negative values represent opposite scroll directions.
/// The exact direction depends on the input backend and engine
/// coordinate convention.
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct MouseScrollDelta {
    x: f32,
    y: f32,
}

impl MouseScrollDelta {
    /// Default number of pixels used to represent one scroll line.
    pub const DEFAULT_MULTIPLE: f32 = 32.0;
}

impl MouseScrollDelta {
    /// Creates a new scroll delta.
    #[must_use]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl MouseScrollDelta {
    /// Returns the horizontal scroll offset.
    #[must_use]
    pub fn x(&self) -> f32 {
        self.x
    }

    /// Returns the vertical scroll offset.
    #[must_use]
    pub fn y(&self) -> f32 {
        self.y
    }
}
