mod asset_plugin;
mod camera_update_plugin;
mod depth_view_plugin;
mod instance_update_plugin;
mod light_update_plugin;
mod mesh_upload_plugin;
mod msaa_view_plugin;
mod pipiline_builder_plugin;
mod render_plugin;
mod renderer_plugin;

pub use {
    asset_plugin::AssetPlugin, camera_update_plugin::CameraUpdatePlugin,
    depth_view_plugin::DepthViewPlugin, instance_update_plugin::InstanceUpdatePlugin,
    light_update_plugin::LightUpdatePlugin, mesh_upload_plugin::MeshUploadPlugin,
    msaa_view_plugin::MsaaViewPlugin, pipiline_builder_plugin::PipelineBuilderPlugin,
    render_plugin::RenderPlugin, renderer_plugin::RendererPlugin,
};
