use crate::{SurfaceTarget, WindowFrontend, WindowHandle, WindowHandlerError};

use {
    kodanu_math::SurfaceSize,
    raw_window_handle::{HasDisplayHandle, HasWindowHandle},
    std::sync::Arc,
};

use winit::window::Window as WinitWindow;

pub struct WinitWindowBackend {
    window: Arc<dyn WinitWindow>,
}

impl WindowFrontend for WinitWindowBackend {
    fn redraw(&self) {
        self.window.request_redraw();
    }

    fn size(&self) -> SurfaceSize {
        let size = self.window.surface_size();

        SurfaceSize::clamped(size.width, size.height)
    }
}

impl WindowHandle for WinitWindowBackend {
    fn target(&self) -> SurfaceTarget {
        SurfaceTarget::new(
            self.window
                .display_handle()
                .unwrap_or_else(|_| panic!("{}", WindowHandlerError::RawDisplayHandle))
                .as_raw(),
            self.window
                .window_handle()
                .unwrap_or_else(|_| panic!("{}", WindowHandlerError::RawWindowHandle))
                .as_raw(),
        )
    }
}

impl From<Box<dyn WinitWindow>> for WinitWindowBackend {
    fn from(value: Box<dyn WinitWindow>) -> Self {
        Self {
            window: Arc::from(value),
        }
    }
}
