use kodanu_math::{Mat4, perspective};

#[derive(Debug, Clone, Copy)]
pub struct PerspectiveProjection {
    fov: f32,
    aspect: f32,
    near: f32,
    far: f32,
}

impl Default for PerspectiveProjection {
    fn default() -> Self {
        Self::new(90.0, 1.0, 0.03, 1000.0)
    }
}

impl PerspectiveProjection {
    pub fn new(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        Self {
            fov: fov.to_radians(),
            aspect,
            near,
            far,
        }
    }
}

impl PerspectiveProjection {
    pub fn set_aspect_ratio(&mut self, aspect: f32) {
        self.aspect = aspect;
    }

    pub fn projection_matrix(&self) -> Mat4 {
        perspective(self.fov, self.aspect, self.near, self.far)
    }

    pub fn fov(&self) -> f32 {
        self.fov
    }

    pub fn aspect(&self) -> f32 {
        self.aspect
    }

    pub fn near(&self) -> f32 {
        self.near
    }

    pub fn far(&self) -> f32 {
        self.far
    }
}
