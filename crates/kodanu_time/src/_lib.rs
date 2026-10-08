//! Time management for the Kodanu engine.
//!
//! This crate provides [`crate::Time`] for frame timing and [`crate::TimePlugin`]
//! for registering time updates with the ECS.

#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]

mod api;
mod plugins;

/// Commonly used types from `kodanu_time`.
///
/// ```
/// use kodanu_time::prelude::*;
/// ```
pub mod prelude {
    pub use crate::{Time, TimePlugin};
}

pub use {api::*, plugins::*};
