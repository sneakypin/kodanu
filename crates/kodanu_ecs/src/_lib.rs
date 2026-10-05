mod ecs;
mod plugin;
mod scheduler;

pub mod prelude {
    pub use crate::{
        Bundle, Component, Entity, Event, EventQueue, Plugin, Registry, Resource, Stage, World,
    };
}

pub use {ecs::api::*, macros::*, plugin::api::*, scheduler::api::*};

pub(crate) use {ecs::internal::*, scheduler::internal::*};
