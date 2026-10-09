use {
    kodanu_ecs::Component,
    kodanu_math::{Mat4, Quat, Vec3},
};

/// Describes the position, rotation, and scale of an entity in 3D space.
///
/// `Transform` uses a translation, rotation, and scale representation.
/// The three components are combined into a transformation matrix by
/// [`Self::matrix`].
///
/// # Coordinate system
///
/// The coordinate system follows the conventions defined by [`Vec3`].
///
/// # Examples
///
/// ```
/// use {kodanu_math::{Quat, Vec3}, kodanu_transform::Transform};
///
/// let transform = Transform::new(
///     Vec3::new(10.0, 0.0, 5.0),
///     Quat::IDENTITY,
///     Vec3::ONE,
/// );
///
/// assert_eq!(transform.position(), Vec3::new(10.0, 0.0, 5.0));
/// assert_eq!(transform.scale(), Vec3::ONE);
/// ```
#[derive(Component, Debug, Clone)]
pub struct Transform {
    position: Vec3,
    rotation: Quat,
    scale: Vec3,
}

impl Default for Transform {
    /// Creates an identity transform.
    ///
    /// The default transform has zero position, identity rotation,
    /// and unit scale.
    fn default() -> Self {
        Self::new(Vec3::ZERO, Quat::IDENTITY, Vec3::ONE)
    }
}

impl Transform {
    /// Creates a transform from a position, rotation, and scale.
    ///
    /// No normalization or validation is performed on the supplied values.
    #[must_use]
    pub const fn new(position: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }
}

impl Transform {
    /// Returns the transformation matrix.
    ///
    /// The matrix combines this transform's scale, rotation, and translation.
    #[must_use]
    pub fn matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }

    /// Returns the inverse transformation matrix.
    ///
    /// This is useful for transforming world-space coordinates into the
    /// local space represented by this transform.
    #[must_use]
    pub fn inverse_matrix(&self) -> Mat4 {
        self.matrix().inverse()
    }

    /// Transforms a point from local space into world space.
    ///
    /// The point is scaled, rotated, and then translated.
    #[must_use]
    pub fn transform_point(&self, point: Vec3) -> Vec3 {
        self.position + self.rotation * (point * self.scale)
    }

    /// Sets the world-space position.
    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    /// Sets the rotation.
    pub fn set_rotation(&mut self, rotation: Quat) {
        self.rotation = rotation;
    }

    /// Sets the position and rotation.
    ///
    /// The current scale is preserved.
    pub fn set_position_and_rotation(&mut self, position: Vec3, rotation: Quat) {
        self.position = position;
        self.rotation = rotation;
    }

    /// Sets the scale.
    pub fn set_scale(&mut self, scale: Vec3) {
        self.scale = scale;
    }

    /// Returns the world-space position.
    #[must_use]
    pub const fn position(&self) -> Vec3 {
        self.position
    }

    /// Returns the rotation.
    #[must_use]
    pub const fn rotation(&self) -> Quat {
        self.rotation
    }

    /// Returns the scale.
    pub const fn scale(&self) -> Vec3 {
        self.scale
    }
}

impl Transform {
    /// Returns the forward direction of the transform in world space.
    #[must_use]
    pub fn forward(&self) -> Vec3 {
        self.rotation * Vec3::FORWARD
    }

    /// Returns the right direction of the transform in world space.
    #[must_use]
    pub fn right(&self) -> Vec3 {
        self.rotation * Vec3::RIGHT
    }

    /// Returns the up direction of the transform in world space.
    #[must_use]
    pub fn up(&self) -> Vec3 {
        self.rotation * Vec3::UP
    }
}

impl Transform {
    /// Moves the transform in world space.
    pub fn translate(&mut self, translation: Vec3) {
        self.position += translation;
    }

    /// Moves the transform in local space.
    ///
    /// The translation is rotated by the transform's current rotation
    /// before being applied to its position.
    pub fn translate_local(&mut self, translation: Vec3) {
        self.position += self.rotation * translation
    }

    /// Rotates the transform around a world-space axis.
    ///
    /// The rotation is applied before the current rotation.
    ///
    /// `axis` is normalized before constructing the rotation.
    ///
    /// # Panics
    ///
    /// This method may produce invalid results if `axis` is zero-length,
    /// depending on the behavior of [`Vec3::normalize`].
    pub fn rotate(&mut self, axis: Vec3, angle: f32) {
        self.rotation = Quat::from_axis_angle(axis.normalize(), angle) * self.rotation;
    }

    /// Rotates the transform around a local-space axis.
    ///
    /// The rotation is applied after the current rotation.
    ///
    /// `axis` is normalized before constructing the rotation.
    pub fn rotate_local(&mut self, axis: Vec3, angle: f32) {
        self.rotation *= Quat::from_axis_angle(axis.normalize(), angle)
    }
}

impl Transform {
    /// Creates a transform with the given position.
    ///
    /// Rotation is [`Quat::IDENTITY`] and scale is [`Vec3::ONE`].
    #[must_use]
    pub const fn from_position(position: Vec3) -> Self {
        Self::new(position, Quat::IDENTITY, Vec3::ONE)
    }

    /// Creates a transform with the given rotation.
    ///
    /// Position is [`Vec3::ZERO`] and scale is [`Vec3::ONE`].
    #[must_use]
    pub const fn from_rotation(rotation: Quat) -> Self {
        Self::new(Vec3::ZERO, rotation, Vec3::ONE)
    }

    /// Creates a transform with the given scale.
    ///
    /// Position is [`Vec3::ZERO`] and rotation is [`Quat::IDENTITY`].
    #[must_use]
    pub const fn from_scale(scale: Vec3) -> Self {
        Self::new(Vec3::ZERO, Quat::IDENTITY, scale)
    }

    /// Creates a transform with the given position and rotation.
    ///
    /// Scale is initialized to [`Vec3::ONE`].
    #[must_use]
    pub const fn from_position_rotation(position: Vec3, rotation: Quat) -> Self {
        Self::new(position, rotation, Vec3::ONE)
    }
}
