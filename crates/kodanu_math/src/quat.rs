use std::ops::{Div, DivAssign, Mul, MulAssign, Neg};

use crate::Vec3;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quat {
    pub const IDENTITY: Self = Self::new(0.0, 0.0, 0.0, 1.0);
}

impl Quat {
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}

impl Quat {
    pub fn to_array(self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }

    pub fn xyz(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }

    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn normalize(self) -> Self {
        self / self.length()
    }

    pub fn try_normalize(self) -> Option<Self> {
        let length_squared = self.length_squared();

        if length_squared <= f32::EPSILON {
            return None;
        }

        Some(self / length_squared.sqrt())
    }

    pub fn conjugate(self) -> Self {
        Self::new(-self.x, -self.y, -self.z, self.w)
    }

    pub fn inverse(self) -> Self {
        self.conjugate() / self.length_squared()
    }

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

    pub fn rotate_vec3(self, vector: Vec3) -> Vec3 {
        let q = self.normalize();

        let qv = Vec3::new(q.x, q.y, q.z);
        let t = qv.cross(vector) * 2.0;

        vector + t * q.w + qv.cross(t)
    }
}

impl Quat {
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

    pub fn from_axis_angle(axis: Vec3, degrees: f32) -> Self {
        let radians = degrees.to_radians();
        let (sin, cos) = (radians * 0.5).sin_cos();
        let axis = axis.normalize();

        Self::new(axis.x * sin, axis.y * sin, axis.z * sin, cos)
    }

    pub fn from_scaled_axis(axis: Vec3) -> Self {
        let degrees = axis.length();

        if degrees <= f32::EPSILON {
            return Self::IDENTITY;
        }

        Self::from_axis_angle(axis / degrees, degrees)
    }

    pub fn from_xyzw(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self::new(x, y, z, w)
    }

    pub fn from_array(arr: [f32; 4]) -> Self {
        Self::new(arr[0], arr[1], arr[2], arr[3])
    }

    pub fn from_rotation_x(degrees: f32) -> Self {
        let radians = degrees.to_radians();
        let (sin, cos) = (radians * 0.5).sin_cos();

        Self::new(sin, 0.0, 0.0, cos)
    }

    pub fn from_rotation_y(degrees: f32) -> Self {
        let radians = degrees.to_radians();
        let (sin, cos) = (radians * 0.5).sin_cos();

        Self::new(0.0, sin, 0.0, cos)
    }

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
