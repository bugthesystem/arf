use std::ops::{Add, Mul, Sub};

/// `Vector3` type
#[derive(Copy, Clone, Debug)]
pub struct Vector3 {
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) z: f64,
}

impl Vector3 {
    /// Vector with components value 0.0f
    pub fn zero() -> Self {
        return Self { x: 0.0, y: 0.0, z: 0.0 };
    }

    /// Vector with components value 1.0f
    pub fn one() -> Self {
        return Self { x: 1.0, y: 1.0, z: 1.0 };
    }

    /// Add two vectors
    pub fn add(v1: Vector3, v2: Vector3) -> Self {
        return Self { x: v1.x + v2.x, y: v1.y + v2.y, z: v1.z + v2.z };
    }

    /// Add vector and float value
    pub fn add_value(v1: Vector3, val: f64) -> Self {
        return Self { x: v1.x + val, y: v1.y + val, z: v1.z + val };
    }

    /// Subtract two vectors
    pub fn sub(v1: Vector3, v2: Vector3) -> Self {
        return Self { x: v1.x - v2.x, y: v1.y - v2.y, z: v1.z - v2.z };
    }

    /// Subtract vector by float value
    pub fn sub_value(v1: Vector3, val: f64) -> Self {
        return Self { x: v1.x - val, y: v1.y - val, z: v1.z - val };
    }

    /// Multiply vector by scalar
    pub fn scale(v1: Vector3, scalar: f64) -> Self {
        return Self { x: v1.x * val, y: v1.y * val, z: v1.z * val };
    }

    /// Multiply vector by vector
    pub fn multiply(v1: Vector3, v2: Vector3) -> Self {
        return Self { x: v1.x * v2.x, y: v1.y * v2.y, z: v1.z * v2.z };
    }
}

/// The addition operator +
impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        return Self::Output { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z };
    }
}

/// The subtract operator -
impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        return Self::Output { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z };
    }
}

/// The multiply operator *
impl Mul for Vector3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        return Self::Output { x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z };
    }
}
