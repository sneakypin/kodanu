use crate::{
    AssetPlugin, CameraUpdatePlugin, DepthViewPlugin, InstanceUpdatePlugin, LightUpdatePlugin,
    MeshUploadPlugin, MsaaViewPlugin, PipelineBuilderPlugin, RendererPlugin,
};

use kodanu_ecs::{Plugin, Registry};

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, registry: &mut impl Registry) {
        registry
            .with_plugin(PipelineBuilderPlugin)
            .with_plugin(InstanceUpdatePlugin)
            .with_plugin(DepthViewPlugin)
            .with_plugin(MsaaViewPlugin)
            .with_plugin(CameraUpdatePlugin)
            .with_plugin(MeshUploadPlugin)
            .with_plugin(LightUpdatePlugin)
            .with_plugin(AssetPlugin)
            .with_plugin(RendererPlugin);
    }
}
