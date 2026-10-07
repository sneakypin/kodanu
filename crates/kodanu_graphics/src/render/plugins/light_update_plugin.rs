use crate::LightRenderer;

use kodanu_ecs::{Plugin, Registry, Stage};

pub struct LightUpdatePlugin;

impl Plugin for LightUpdatePlugin {
    fn build(&self, registry: &mut impl Registry) {
        registry
            .with_system(Stage::Startup, LightRenderer::light_startup_system)
            .with_system(Stage::PreRender, LightRenderer::light_system);
    }
}
