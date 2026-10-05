mod api;
mod components;
mod internal;
mod plugins;

pub mod prelude {
    pub use crate::{
        AssetServer, Backend, Color, Material, Mesh, MeshRenderer, Vertex,
    };
}

pub use {api::*, components::*, plugins::*};

pub(crate) use internal::*;
