use crate::MouseScrollDelta;

/// Describes the unit used by a mouse scroll event.
///
/// Mouse scroll input can be reported either as discrete lines
/// or as a continuous pixel offset.

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseScroll {
    /// Scroll amount reported in logical lines.
    ///
    /// The value is represented by [`crate::MouseScrollDelta`] and may be
    /// converted to pixels using [`crate::MouseScrollDelta::DEFAULT_MULTIPLE`].
    Lines(MouseScrollDelta),
    /// Scroll amount reported in physical pixels.
    ///
    /// Pixel-based scrolling is typically produced by high-resolution
    /// scrolling devices such as touchpads.
    Pixels(MouseScrollDelta),
}
