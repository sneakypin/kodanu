pub use {
    kodanu_camera::{ActiveCamera, Camera, CameraSettings},
    kodanu_ecs::{Commands, Plugin, Query, Registry, Res, Stage, With, Write},
    kodanu_input::{Axis, Input},
    kodanu_math::Vec3,
    kodanu_physics::{Collider, RigidBody},
    kodanu_time::Time,
    kodanu_transform::Transform,
};

pub struct FreeCameraPlugin;

impl Plugin for FreeCameraPlugin {
    fn build(&self, registry: &mut impl Registry) {
        registry
            .with_res(CameraSettings::default())
            .with_system(Stage::Startup, free_camera_startup_system)
            .with_system(Stage::LateUpdate, free_camera_system);
    }
}

fn free_camera_system(
    query: Query<Write<Transform>, With<ActiveCamera>>,
    settings: Res<CameraSettings>,
    input: Res<Input>,
    time: Res<Time>,
) {
    for transform in query {
        let direction = transform.forward() * input.axis(Axis::MoveY)
            + -transform.right() * input.axis(Axis::MoveX)
            + transform.up() * input.axis(Axis::MoveZ);

        let yaw = input.axis(Axis::LookX) * settings.sens * time.delta();
        let pitch = input.axis(Axis::LookY) * settings.sens * time.delta();

        transform.translate(direction * settings.speed * time.delta());

        transform.rotate(Vec3::Y, yaw);
        transform.rotate_local(Vec3::X, pitch);
    }
}

fn free_camera_startup_system(commands: Commands) {
    commands.spawn_bundle((
        Transform::default(),
        Camera::default(),
        Collider::sphere(1.0),
        RigidBody::kinematic(),
        ActiveCamera,
    ));
}
