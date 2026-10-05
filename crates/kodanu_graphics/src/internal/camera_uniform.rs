use {
    bytemuck::{Pod, Zeroable},
    kodanu_math::Mat4,
};

#[repr(C)]
#[derive(Default, Debug, Clone, Copy, Pod, Zeroable)]
pub(crate) struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl From<Mat4> for CameraUniform {
    fn from(value: Mat4) -> Self {
        Self {
            view_proj: value.to_cols_array_2d(),
        }
    }
}
