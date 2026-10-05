use kodanu_ecs::Resource;

#[derive(Resource, Debug, Clone, Copy)]
pub struct CameraSettings {
    pub sens: f32,
    pub speed: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self::new(100.0, 15.0)
    }
}

impl CameraSettings {
    pub fn new(sens: f32, speed: f32) -> Self {
        Self { sens, speed }
    }
}
