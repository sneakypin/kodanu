use crate::{KeyboardEvent, MouseEvent, MouseWheelEvent};

use kodanu_math::{MousePosition, SurfaceSize};

#[derive(Debug, Clone, Copy)]
pub enum WindowEvent {
    CloseRequested,
    RedrawRequested,
    SurfaceResized(SurfaceSize),
    KeyboardInput(KeyboardEvent),
    MouseInput(MouseEvent),
    PointerMoved(MousePosition),
    MouseWheel(MouseWheelEvent),
    Unknown,
}
