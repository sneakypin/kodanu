use rapier3d::prelude::{
    RigidBodyBuilder as RapierRigidBodyBuilder, RigidBodyHandle as RapierRigidBodyHandle,
    RigidBodyType as RapierRigidBodyType,
};

use {kodanu_ecs::Component, kodanu_transform::Transform};

#[derive(Component, Debug)]
pub struct RigidBody {
    handle: Option<RapierRigidBodyHandle>,
    body_type: RapierRigidBodyType,
}

impl Default for RigidBody {
    fn default() -> Self {
        Self::dynamic()
    }
}

impl RigidBody {
    pub fn dynamic() -> Self {
        Self::from(RapierRigidBodyType::Dynamic)
    }

    pub fn fixed() -> Self {
        Self::from(RapierRigidBodyType::Fixed)
    }

    pub fn kinematic() -> Self {
        Self::from(RapierRigidBodyType::KinematicPositionBased)
    }
}

impl RigidBody {
    pub fn handle(&self) -> Option<RapierRigidBodyHandle> {
        self.handle
    }

    pub fn body_type(&self) -> RapierRigidBodyType {
        self.body_type
    }
}

impl RigidBody {
    pub(crate) fn builder(&self, transform: &Transform) -> RapierRigidBodyBuilder {
        let (axis, angle) = transform.rotation().to_axis_angle();

        RapierRigidBodyBuilder::new(self.body_type)
            .translation(transform.position())
            .rotation(axis * angle)
    }

    pub(crate) fn set_handle(&mut self, handle: Option<RapierRigidBodyHandle>) {
        self.handle = handle
    }
}

impl From<RapierRigidBodyType> for RigidBody {
    fn from(value: RapierRigidBodyType) -> Self {
        Self {
            handle: None,
            body_type: value,
        }
    }
}
