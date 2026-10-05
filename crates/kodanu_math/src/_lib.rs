mod mouse_delta;
mod mouse_position;
mod mouse_scroll;
mod mouse_scroll_delta;
mod surface_size;
mod window_size;

pub use {
    mouse_delta::MouseDelta, mouse_position::MousePosition, mouse_scroll::MouseScroll,
    mouse_scroll_delta::MouseScrollDelta, surface_size::SurfaceSize, window_size::WindowSize,
};

pub mod prelude {
    pub use glam::camera::rh::proj::directx::perspective;

    pub use glam::*;
}

pub use glam::camera::rh::proj::directx::perspective;

pub use glam::*;
