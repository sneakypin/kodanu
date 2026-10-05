use crate::{Closing, KeyboardEvent, MouseEvent, MouseWheelEvent, Redrawing};

pub use {
    kodanu_ecs::{EventQueue, Plugin, Registry},
    kodanu_math::{MousePosition, SurfaceSize},
};

pub struct WindowEventPlugin;

impl Plugin for WindowEventPlugin {
    fn build(&self, registry: &mut impl Registry) {
        registry
            .with_event(EventQueue::<Closing>::default())
            .with_event(EventQueue::<Redrawing>::default())
            .with_event(EventQueue::<SurfaceSize>::default());

        registry
            .with_event(EventQueue::<KeyboardEvent>::default())
            .with_event(EventQueue::<MouseEvent>::default())
            .with_event(EventQueue::<MousePosition>::default())
            .with_event(EventQueue::<MouseWheelEvent>::default());
    }
}
