use {
    serde::{Deserialize, Serialize},
    std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

/// A three-dimensional vector with `f32` components.
///
/// `Vec3` is used for 3D positions, directions, scales, and other
/// quantities represented by three components.
///
/// The vector provides common operations such as dot product, cross
/// product, length calculation, and normalization.
///
/// The directional constants use the following coordinate convention:
/// positive Y is up, positive X is right, and negative Z is forward.
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Vec3 {
    /// The x component of the vector.
    pub x: f32,
    /// The y component of the vector.
    pub y: f32,
    /// The z component of the vector.
    pub z: f32,
}

impl Vec3 {
    /// The zero vector `(0, 0, 0)`.
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0);
    /// The vector `(1, 1, 1)`.
    pub const ONE: Self = Self::new(1.0, 1.0, 1.0);
    /// The positive Y-axis direction `(0, 1, 0)`.
    pub const UP: Self = Self::new(0.0, 1.0, 0.0);
    /// The negative Y-axis direction `(0, -1, 0)`.
    pub const DOWN: Self = Self::new(0.0, -1.0, 0.0);
    /// The negative X-axis direction `(-1, 0, 0)`.
    pub const LEFT: Self = Self::new(-1.0, 0.0, 0.0);
    /// The positive X-axis direction `(1, 0, 0)`.
    pub const RIGHT: Self = Self::new(1.0, 0.0, 0.0);
    /// The forward direction along the negative Z-axis `(0, 0, -1)`.
    pub const FORWARD: Self = Self::new(0.0, 0.0, -1.0);
    /// The backward direction along the positive Z-axis `(0, 0, 1)`.
    pub const BACKWARD: Self = Self::new(0.0, 0.0, 1.0);
}

impl Vec3 {
    /// Creates a vector from its X, Y, and Z components.
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl Vec3 {
    /// Returns the squared length of the vector without calculating a square root.
    #[must_use]
    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    /// Returns the Euclidean length of the vector.
    #[must_use]
    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    /// Returns the dot product of two vectors.
    #[must_use]
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Returns the cross product of two vectors.
    ///
    /// The resulting vector is perpendicular to both input vectors
    /// according to the right-hand rule.
    #[must_use]
    pub fn cross(self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// Returns the normalized vector, or `None` if its squared length
    /// is less than or equal to `f32::EPSILON`.
    #[must_use]
    pub fn try_normalize(self) -> Option<Self> {
        let length_squared = self.length_squared();

        if length_squared <= f32::EPSILON {
            return None;
        }

        Some(self / length_squared.sqrt())
    }

    /// Returns the vector divided by its length.
    ///
    /// A zero-length vector produces non-finite components.
    /// Use [`Self::try_normalize`] when the vector may have zero length.
    #[must_use]
    pub fn normalize(self) -> Self {
        self / self.length()
    }

    /// Returns the vector components as an array in `[x, y, z]` order.
    #[must_use]
    pub const fn to_array(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

impl Add for Vec3 {
    type Output = Self;

    /// Adds two vectors component-wise.
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    /// Adds two vectors component-wise.
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    /// Multiplies each component of the vector by a scalar.
    fn mul(self, scalar: f32) -> Self {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    /// Divides each component of the vector by a scalar.
    fn div(self, scalar: f32) -> Self {
        Self::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    /// Negates each component of the vector.
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl AddAssign for Vec3 {
    /// Adds another vector to this vector in place.
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl SubAssign for Vec3 {
    /// Subtracts another vector from this vector in place.
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl MulAssign<f32> for Vec3 {
    /// Multiplies each component of this vector by a scalar in place.
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl DivAssign<f32> for Vec3 {
    /// Divides each component of this vector by a scalar in place.
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    /// Multiplies two vectors component-wise.
    ///
    /// This is not a dot product. Use [`Vec3::dot`] to calculate the dot
    /// product of two vectors.
    fn mul(self, other: Vec3) -> Self {
        Self::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}
