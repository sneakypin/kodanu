use crate::Projection;

use {
    kodanu_ecs::Component,
    kodanu_math::{Mat4, SurfaceSize},
};

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct Camera {
    projection: Projection,
}

impl Camera {
    pub fn view_proj(&self, view_matrix: Mat4) -> Mat4 {
        self.projection.projection_matrix() * view_matrix
    }

    pub fn set_aspect(&mut self, aspect: f32) {
        match &mut self.projection {
            Projection::Perspective(projection) => projection.set_aspect_ratio(aspect),
        }
    }

    pub fn set_viewport(&mut self, size: SurfaceSize) {
        if size.height() == 0 {
            return;
        }

        self.set_aspect(size.width() as f32 / size.height() as f32);
    }

    pub fn projection_matrix(&self) -> Mat4 {
        self.projection.projection_matrix()
    }

    pub fn projection(&self) -> Projection {
        self.projection
    }
}

impl From<Projection> for Camera {
    fn from(value: Projection) -> Self {
        Self { projection: value }
    }
}
