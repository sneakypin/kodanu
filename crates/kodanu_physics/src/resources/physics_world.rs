use crate::{Collider, RigidBody};

use rapier3d::prelude::{
    PhysicsWorld as RapierPhysicsWorld, Pose3 as RapierPose3, RigidBodyType as RapierRigidBodyType,
};

pub use {
    kodanu_ecs::{Query, Read, Res, ResMut, Resource, Write},
    kodanu_math::Vec3,
    kodanu_transform::Transform,
};

#[derive(Resource, Default)]
pub struct PhysicsWorld {
    physics: RapierPhysicsWorld,
}

impl PhysicsWorld {
    pub fn set_gravity(&mut self, gravity: Vec3) {
        self.physics.gravity = Vec3::new(gravity.x, gravity.y, gravity.z);
    }

    pub fn remove_rigid_body(&mut self, body: RigidBody) {
        if let Some(handle) = body.handle() {
            self.physics.remove_body(handle);
        }
    }

    pub fn remove_collider(&mut self, collider: Collider) {
        if let Some(handle) = collider.handle() {
            self.physics.remove_collider(handle);
        }
    }
}

impl PhysicsWorld {
    pub(crate) fn collider_system(
        mut world: ResMut<PhysicsWorld>,
        query: Query<(Read<RigidBody>, Write<Collider>)>,
    ) {
        for (rigidbody, collider) in query {
            if collider.handle().is_some() {
                continue;
            }

            collider.set_handle(Some(
                world
                    .physics
                    .insert_collider(collider.builder().build(), rigidbody.handle()),
            ));
        }
    }

    pub(crate) fn rigidbody_system(
        mut world: ResMut<PhysicsWorld>,
        query: Query<(Read<Transform>, Write<RigidBody>)>,
    ) {
        for (transform, rigid_body) in query {
            if rigid_body.handle().is_some() {
                continue;
            }

            rigid_body.set_handle(Some(
                world
                    .physics
                    .insert_body(rigid_body.builder(transform).build()),
            ));
        }
    }

    pub(crate) fn dynamic_physics_system(
        world: Res<PhysicsWorld>,
        query: Query<(Write<Transform>, Write<RigidBody>)>,
    ) {
        for (transform, rigid_body) in query {
            let Some(handle) = rigid_body.handle() else {
                continue;
            };

            if rigid_body.body_type() != RapierRigidBodyType::Dynamic {
                continue;
            }

            let Some(body) = world.physics.bodies.get(handle) else {
                continue;
            };

            let position = body.position();

            transform.set_position_and_rotation(position.translation, position.rotation);
        }
    }

    pub(crate) fn kinematic_physics_system(
        mut world: ResMut<PhysicsWorld>,
        query: Query<(Read<Transform>, Write<RigidBody>)>,
    ) {
        for (transform, rigid_body) in query {
            let Some(handle) = rigid_body.handle() else {
                continue;
            };

            if rigid_body.body_type() != RapierRigidBodyType::KinematicPositionBased {
                continue;
            }

            let Some(body) = world.physics.bodies.get_mut(handle) else {
                continue;
            };

            body.set_next_kinematic_position(RapierPose3::from_parts(
                transform.position(),
                transform.rotation(),
            ));
        }
    }

    pub(crate) fn physics_system(mut world: ResMut<PhysicsWorld>) {
        world.physics.step();
    }
}
