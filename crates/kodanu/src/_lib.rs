pub mod prelude {
    pub use kodanu_app::prelude::*;
    pub use kodanu_camera::prelude::*;
    pub use kodanu_ecs::prelude::*;
    pub use kodanu_graphics::prelude::*;
    pub use kodanu_input::prelude::*;
    pub use kodanu_math::prelude::*;
    pub use kodanu_plugins::prelude::*;
    pub use kodanu_time::prelude::*;
    pub use kodanu_transform::prelude::*;
    pub use kodanu_window::prelude::*;

    #[cfg(feature = "macros")]
    pub use kodanu_macros::*;
    #[cfg(feature = "physics")]
    pub use kodanu_physics::prelude::*;
}

#[cfg(feature = "macros")]
pub use kodanu_macros as macros;
#[cfg(feature = "physics")]
pub use kodanu_physics as physics;
