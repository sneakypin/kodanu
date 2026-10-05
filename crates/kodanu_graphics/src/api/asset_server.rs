use crate::{AssetStorage, Handle, MeshRenderer};

use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use kodanu_ecs::{Query, ResMut, Resource, Write};

#[derive(Resource, Default)]
pub struct AssetServer {
    assets: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl AssetServer {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            assets: HashMap::with_capacity(capacity),
        }
    }
}

impl AssetServer {
    pub fn add<A>(&mut self, asset: A) -> Handle<A>
    where
        A: Any + Send + Sync,
    {
        self.assets
            .entry(TypeId::of::<A>())
            .or_insert_with(|| Box::new(AssetStorage::<A>::default()))
            .downcast_mut::<AssetStorage<A>>()
            .unwrap()
            .add(asset)
    }

    pub fn get<A>(&self, handle: Handle<A>) -> Option<&A>
    where
        A: Any + Send + Sync,
    {
        self.assets
            .get(&TypeId::of::<A>())
            .and_then(|asset| asset.downcast_ref::<AssetStorage<A>>())?
            .get(handle)
    }
}

impl AssetServer {
    pub(crate) fn asset_system(mut server: ResMut<AssetServer>, query: Query<Write<MeshRenderer>>) {
        for renderer in query {
            if renderer.material_handle().is_some() || renderer.mesh_handle().is_some() {
                continue;
            }

            let Some(mesh) = renderer.take_mesh() else {
                continue;
            };

            let Some(material) = renderer.take_material() else {
                continue;
            };

            renderer.set_mesh_handle(Some(server.add(mesh)));
            renderer.set_material_handle(Some(server.add(material)));
        }
    }
}
