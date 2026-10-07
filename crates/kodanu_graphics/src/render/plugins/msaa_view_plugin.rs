use crate::MsaaView;

use kodanu_ecs::{Plugin, Registry, Stage};

pub struct MsaaViewPlugin;

impl Plugin for MsaaViewPlugin {
    fn build(&self, registry: &mut impl Registry) {
        registry
            .with_system(Stage::Startup, MsaaView::msaa_view_startup)
            .with_system(Stage::PreRender, MsaaView::msaa_view_system);
    }
}
