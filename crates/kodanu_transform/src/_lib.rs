//! Spatial transformations for entities in the Kodanu engine.
//!
//! This module provides [`crate::Transform`], a component that describes an entity's
//! position, rotation, and scale in 3D space.

#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]

mod transform;

/// Commonly used types from `kodanu_transform`.
///
/// ```
/// use kodanu_transform::prelude::*; or kodanu_transform::Transform
/// ```
pub mod prelude {
    pub use crate::Transform;
}

pub use transform::Transform;
