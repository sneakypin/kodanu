use crate::{Direction, Spacing, UiPosition, UiSize};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    pub width: UiSize,
    pub height: UiSize,
    pub position: UiPosition,

    pub margin: Spacing,
    pub padding: Spacing,

    pub direction: Direction,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            width: UiSize::Fixed(0.0),
            height: UiSize::Fixed(0.0),
            position: UiPosition::default(),

            margin: Spacing::ZERO,
            padding: Spacing::ZERO,

            direction: Direction::Vertical,
        }
    }
}
