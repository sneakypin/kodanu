use crate::{Color, Handle, Material, Mesh};

use kodanu_ecs::Component;

#[derive(Component, Debug)]
pub struct MeshRenderer {
    mesh: Option<Mesh>,
    mesh_handle: Option<Handle<Mesh>>,
    material: Option<Material>,
    material_handle: Option<Handle<Material>>,
}

impl MeshRenderer {
    pub fn new(mesh: Option<Mesh>, material: Option<Material>) -> Self {
        Self {
            mesh_handle: None,
            mesh,
            material_handle: None,
            material,
        }
    }
}

impl MeshRenderer {
    pub fn cube(color: Color) -> Self {
        Self::new(Some(Mesh::cube()), Some(Material::from(color)))
    }
}

impl MeshRenderer {
    pub fn mesh_handle(&self) -> Option<Handle<Mesh>> {
        self.mesh_handle
    }

    pub fn take_mesh(&mut self) -> Option<Mesh> {
        self.mesh.take()
    }

    pub fn material_handle(&self) -> Option<Handle<Material>> {
        self.material_handle
    }

    pub fn take_material(&mut self) -> Option<Material> {
        self.material.take()
    }
}
impl MeshRenderer {
    pub(crate) fn set_mesh_handle(&mut self, handle: Option<Handle<Mesh>>) {
        self.mesh_handle = handle
    }

    pub(crate) fn set_material_handle(&mut self, handle: Option<Handle<Material>>) {
        self.material_handle = handle
    }
}
