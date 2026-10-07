use crate::AssetServer;

use kodanu_ecs::{Plugin, Registry, Stage};

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, registry: &mut impl Registry) {
        registry.with_system(Stage::PreRender, AssetServer::asset_system);
    }
}
