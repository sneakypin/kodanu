use crate::Time;

use kodanu_ecs::{Plugin, Registry, Stage};

/// Registers the [`crate::Time`] resource and its update system.
pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, registry: &mut impl Registry) {
        registry
            .with_res(Time::default())
            .with_system(Stage::TimeUpdate, Time::time_system);
    }
}
