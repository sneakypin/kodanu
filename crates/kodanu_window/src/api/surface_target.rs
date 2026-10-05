use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

#[derive(Debug, Clone, Copy)]
pub struct SurfaceTarget {
    display: RawDisplayHandle,
    window: RawWindowHandle,
}

impl SurfaceTarget {
    pub fn new(display: RawDisplayHandle, window: RawWindowHandle) -> Self {
        Self { display, window }
    }
}

impl SurfaceTarget {
    pub fn raw_display(&self) -> RawDisplayHandle {
        self.display
    }

    pub fn raw_window(&self) -> RawWindowHandle {
        self.window
    }
}
