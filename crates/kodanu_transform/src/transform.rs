use {
    kodanu_ecs::Component,
    kodanu_math::{Mat4, Quat, Vec3},
};

#[derive(Component, Debug, Clone)]
pub struct Transform {
    position: Vec3,
    rotation: Quat,
    scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self::new(Vec3::ZERO, Quat::IDENTITY, Vec3::ONE)
    }
}

impl Transform {
    pub fn new(position: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }
}

impl Transform {
    pub fn matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }

    pub fn view_matrix(&self) -> Mat4 {
        self.matrix().inverse()
    }

    pub fn point(&self, point: Vec3) -> Vec3 {
        self.position + self.rotation * (point * self.scale)
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    pub fn set_rotation(&mut self, rotation: Quat) {
        self.rotation = rotation;
    }

    pub fn set_position_and_rotation(&mut self, position: Vec3, rotation: Quat) {
        self.position = position;
        self.rotation = rotation;
    }

    pub fn set_scale(&mut self, scale: Vec3) {
        self.scale = scale;
    }

    pub fn position(&self) -> Vec3 {
        self.position
    }

    pub fn rotation(&self) -> Quat {
        self.rotation
    }

    pub fn scale(&self) -> Vec3 {
        self.scale
    }
}

impl Transform {
    pub fn forward(&self) -> Vec3 {
        self.rotation * Vec3::NEG_Z
    }

    pub fn right(&self) -> Vec3 {
        self.rotation * Vec3::X
    }

    pub fn up(&self) -> Vec3 {
        self.rotation * Vec3::Y
    }
}

impl Transform {
    pub fn translate(&mut self, translation: Vec3) {
        self.position += translation;
    }

    pub fn translate_local(&mut self, translation: Vec3) {
        self.position += self.rotation * translation
    }

    pub fn rotate(&mut self, axis: Vec3, angle: f32) {
        self.rotation = Quat::from_axis_angle(axis.normalize(), angle) * self.rotation;
    }

    pub fn rotate_local(&mut self, axis: Vec3, angle: f32) {
        self.rotation *= Quat::from_axis_angle(axis.normalize(), angle)
    }
}

impl Transform {
    pub fn from_position(position: Vec3) -> Self {
        Self::new(position, Quat::IDENTITY, Vec3::ONE)
    }

    pub fn from_rotation(rotation: Quat) -> Self {
        Self::new(Vec3::ZERO, rotation, Vec3::ONE)
    }

    pub fn from_scale(scale: Vec3) -> Self {
        Self::new(Vec3::ZERO, Quat::IDENTITY, scale)
    }

    pub fn from_position_rotation(position: Vec3, rotation: Quat) -> Self {
        Self::new(position, rotation, Vec3::ONE)
    }
}
