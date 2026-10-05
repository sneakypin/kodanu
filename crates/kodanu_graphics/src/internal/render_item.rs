use crate::{GpuMaterial, GpuMesh, GpuModel};

pub(crate) struct RenderItem {
    mesh: GpuMesh,
    material: GpuMaterial,
    model: GpuModel,
}

impl RenderItem {
    pub fn new(mesh: GpuMesh, material: GpuMaterial, model: GpuModel) -> Self {
        Self {
            mesh,
            material,
            model,
        }
    }
}

impl RenderItem {
    pub fn mesh(&self) -> &GpuMesh {
        &self.mesh
    }

    pub fn material(&self) -> &GpuMaterial {
        &self.material
    }

    pub fn model(&self) -> &GpuModel {
        &self.model
    }
}
