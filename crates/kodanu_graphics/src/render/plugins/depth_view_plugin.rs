use crate::DepthView;

use kodanu_ecs::{Plugin, Registry, Stage};

pub struct DepthViewPlugin;

impl Plugin for DepthViewPlugin {
    fn build(&self, registry: &mut impl Registry) {
        registry
            .with_system(Stage::Startup, DepthView::depth_view_startup_system)
            .with_system(Stage::PreRender, DepthView::depth_view_system);
    }
}
