use crate::PhysicsWorld;

use kodanu_ecs::{Plugin, Registry, Stage};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, registry: &mut impl Registry) {
        registry.with_res(PhysicsWorld::default());

        registry
            .with_system(Stage::PreFixedUpdate, PhysicsWorld::rigidbody_system)
            .with_system(Stage::PreFixedUpdate, PhysicsWorld::collider_system)
            .with_system(
                Stage::PreFixedUpdate,
                PhysicsWorld::kinematic_physics_system,
            );

        registry
            .with_system(Stage::FixedUpdate, PhysicsWorld::physics_system)
            .with_system(Stage::PostFixedUpdate, PhysicsWorld::dynamic_physics_system);
    }
}
