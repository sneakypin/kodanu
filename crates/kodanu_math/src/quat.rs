use crate::Vec3;

use std::ops::{Div, DivAssign, Mul, MulAssign, Neg};

/// A quaternion representing a 3D rotation.
///
/// The quaternion is stored in `(x, y, z, w)` order, where `(x, y, z)`
/// is the vector part and `w` is the scalar part.
///
/// Rotation constructors that accept angles use degrees.
/// Quaternion operations generally assume a valid, non-zero quaternion
/// when normalization or rotation is required.
///
/// # Examples
///
/// ```
/// use kodanu_math::{Quat, Vec3};
///
/// let rotation = Quat::from_rotation_y(90.0);
/// let direction = rotation * Vec3::FORWARD;
///
/// assert!((direction.x - 1.0).abs() < 1e-5);
/// assert!(direction.y.abs() < 1e-5);
/// assert!(direction.z.abs() < 1e-5);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quat {
    /// The x component of the quaternion's vector part.
    pub x: f32,
    /// The y component of the quaternion's vector part.
    pub y: f32,
    /// The z component of the quaternion's vector part.
    pub z: f32,
    /// The scalar component of the quaternion.
    pub w: f32,
}

impl Default for Quat {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Quat {
    /// The identity rotation `(0, 0, 0, 1)`.
    pub const IDENTITY: Self = Self::new(0.0, 0.0, 0.0, 1.0);
}

impl Quat {
    /// Creates a quaternion from its `(x, y, z, w)` components.
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}

impl Quat {
    /// Returns the quaternion components as an array in `[x, y, z, w]` order.
    #[must_use]
    pub const fn to_array(self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }

    /// Returns the vector part `(x, y, z)` of the quaternion.
    #[must_use]
    pub const fn xyz(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }

    /// Returns the squared magnitude of the quaternion.
    #[must_use]
    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    /// Returns the magnitude of the quaternion.
    #[must_use]
    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    /// Returns the dot product of two quaternions.
    #[must_use]
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    /// Returns the normalized quaternion, or `None` if its squared magnitude
    /// is less than or equal to `f32::EPSILON`.
    #[must_use]
    pub fn try_normalize(self) -> Option<Self> {
        let length_squared = self.length_squared();

        if length_squared <= f32::EPSILON {
            return None;
        }

        Some(self / length_squared.sqrt())
    }

    /// Returns the normalized quaternion.
    ///
    /// A zero-length quaternion produces non-finite components.
    /// Use [`Self::try_normalize`] when the quaternion may have zero length.
    #[must_use]
    pub fn normalize(self) -> Self {
        self / self.length()
    }

    /// Returns the conjugate of the quaternion.
    ///
    /// The vector part is negated while the scalar part remains unchanged.
    #[must_use]
    pub fn conjugate(self) -> Self {
        Self::new(-self.x, -self.y, -self.z, self.w)
    }

    /// Returns the multiplicative inverse of the quaternion.
    ///
    /// For a non-zero quaternion `q`, the inverse is
    /// `conjugate(q) / length_squared(q)`.
    ///
    /// A zero-length quaternion produces non-finite components.
    #[must_use]
    pub fn inverse(self) -> Self {
        self.conjugate() / self.length_squared()
    }

    /// Converts the quaternion to an axis-angle representation.
    ///
    /// Returns the rotation axis and angle in degrees.
    /// If the quaternion cannot provide a valid axis, returns
    /// [`Vec3::RIGHT`] and `0.0`.
    #[must_use]
    pub fn to_axis_angle(self) -> (Vec3, f32) {
        let Some(mut q) = self.try_normalize() else {
            return (Vec3::RIGHT, 0.0);
        };

        if q.w < 0.0 {
            q = -q;
        }

        let angle = 2.0 * q.w.clamp(-1.0, 1.0).acos();
        let sin_half_angle = (1.0 - q.w * q.w).sqrt();

        if sin_half_angle <= f32::EPSILON {
            return (Vec3::RIGHT, 0.0);
        }

        let axis = Vec3::new(q.x, q.y, q.z) / sin_half_angle;

        (axis, angle.to_degrees())
    }

    /// Rotates a vector by this quaternion.
    ///
    /// The quaternion is normalized before applying the rotation.
    /// A zero-length quaternion does not represent a valid rotation.
    #[must_use]
    pub fn rotate_vec3(self, vector: Vec3) -> Vec3 {
        let q = self.normalize();

        let qv = Vec3::new(q.x, q.y, q.z);
        let t = qv.cross(vector) * 2.0;

        vector + t * q.w + qv.cross(t)
    }
}

impl Quat {
    /// Creates a quaternion from Euler angles in degrees.
    ///
    /// The angles represent rotations around the X, Y, and Z axes,
    /// respectively.
    ///
    /// The exact composition convention depends on the formula used
    /// by this implementation.
    #[must_use]
    pub fn from_euler(x_degrees: f32, y_degrees: f32, z_degrees: f32) -> Self {
        let x = x_degrees.to_radians();
        let y = y_degrees.to_radians();
        let z = z_degrees.to_radians();

        let (sin_x, cos_x) = (x * 0.5).sin_cos();
        let (sin_y, cos_y) = (y * 0.5).sin_cos();
        let (sin_z, cos_z) = (z * 0.5).sin_cos();

        Self::new(
            sin_x * cos_y * cos_z + cos_x * sin_y * sin_z,
            cos_x * sin_y * cos_z - sin_x * cos_y * sin_z,
            cos_x * cos_y * sin_z + sin_x * sin_y * cos_z,
            cos_x * cos_y * cos_z - sin_x * sin_y * sin_z,
        )
    }

    /// Creates a quaternion from an axis and an angle in degrees.
    ///
    /// The axis is normalized before use. The axis must have non-zero length.
    #[must_use]
    pub fn from_axis_angle(axis: Vec3, degrees: f32) -> Self {
        let radians = degrees.to_radians();
        let (sin, cos) = (radians * 0.5).sin_cos();
        let axis = axis.normalize();

        Self::new(axis.x * sin, axis.y * sin, axis.z * sin, cos)
    }

    /// Creates a quaternion from a scaled axis.
    ///
    /// The vector direction specifies the rotation axis and its length
    /// specifies the rotation angle in degrees.
    ///
    /// A vector with a length less than or equal to `f32::EPSILON`
    /// produces the identity rotation.
    #[must_use]

    pub fn from_scaled_axis(axis: Vec3) -> Self {
        let degrees = axis.length();

        if degrees <= f32::EPSILON {
            return Self::IDENTITY;
        }

        Self::from_axis_angle(axis / degrees, degrees)
    }

    /// Creates a quaternion from `(x, y, z, w)` components.
    #[must_use]
    pub const fn from_xyzw(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self::new(x, y, z, w)
    }

    /// Creates a quaternion from an array in `[x, y, z, w]` order.
    #[must_use]
    pub const fn from_array(arr: [f32; 4]) -> Self {
        Self::new(arr[0], arr[1], arr[2], arr[3])
    }

    /// Creates a rotation around the X axis by the given angle in degrees.
    #[must_use]
    pub fn from_rotation_x(degrees: f32) -> Self {
        let radians = degrees.to_radians();
        let (sin, cos) = (radians * 0.5).sin_cos();

        Self::new(sin, 0.0, 0.0, cos)
    }

    /// Creates a rotation around the Y axis by the given angle in degrees.
    #[must_use]
    pub fn from_rotation_y(degrees: f32) -> Self {
        let radians = degrees.to_radians();
        let (sin, cos) = (radians * 0.5).sin_cos();

        Self::new(0.0, sin, 0.0, cos)
    }

    /// Creates a rotation around the Z axis by the given angle in degrees.
    #[must_use]
    pub fn from_rotation_z(degrees: f32) -> Self {
        let radians = degrees.to_radians();
        let (sin, cos) = (radians * 0.5).sin_cos();

        Self::new(0.0, 0.0, sin, cos)
    }
}

impl Mul for Quat {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x,
            self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w,
            self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
        )
    }
}

impl MulAssign for Quat {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Mul<f32> for Quat {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl MulAssign<f32> for Quat {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}

impl Div<f32> for Quat {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}
impl DivAssign<f32> for Quat {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self.w /= rhs;
    }
}

impl Neg for Quat {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl Mul<Vec3> for Quat {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        self.rotate_vec3(rhs)
    }
}
