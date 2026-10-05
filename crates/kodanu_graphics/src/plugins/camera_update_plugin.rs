use crate::CameraRenderer;

pub use kodanu_ecs::{Plugin, Registry, Stage};

pub struct CameraUpdatePlugin;

impl Plugin for CameraUpdatePlugin {
    fn build(&self, registry: &mut impl Registry) {
        registry
            .with_system(Stage::Startup, CameraRenderer::camera_startup_system)
            .with_system(Stage::PreRender, CameraRenderer::camera_update_system);
    }
}
