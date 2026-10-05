use crate::MouseScrollDelta;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseScroll {
    Lines(MouseScrollDelta),
    Pixels(MouseScrollDelta),
}
