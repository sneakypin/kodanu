//! Camera and projection components for the Kodanu game engine.
//!
//! This crate provides camera components, projection types, and camera-related
//! settings for rendering scenes in 3D.
//!
//! The main types are [`crate::Camera`], [`crate::Projection`], and
//! [`crate::PerspectiveProjection`].

#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]

mod api;
mod components;
mod resources;

/// Commonly used types from `kodanu_camera`.
///
/// ```
/// use kodanu_camera::prelude::*;
/// ```
pub mod prelude {
    pub use crate::{ActiveCamera, Camera, CameraSettings, PerspectiveProjection, Projection};
}

pub use {api::*, components::*, resources::*};
