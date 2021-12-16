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

    /// Calculate two vectors cross product
    pub fn cross_product(v1: Vector3, v2: Vector3) -> Vector3 {
        return Self { x: v1.y * v2.z - v1.z * v2.y, y: v1.z * v2.x - v1.x * v2.z, z: v1.x * v2.y - v1.y * v2.x };
    }

    /// Calculate one vector perpendicular vector
    pub fn perpendicular(v: Vector3) -> Vector3 {
        let mut result: Vector3 = Vector3::zero();

        let mut min: f64 = v.x.abs();
        let mut cardinal_axis: Vector3 = Vector3 { x: 1.0, y: 0.0, z: 0.0 };

        if v.y.abs() < min {
            min = v.y.abs();

            cardinal_axis = Vector3 { x: 0.0, y: 1.0, z: 0.0 };
        }

        if v.z.abs() < min {
            cardinal_axis = Vector3 { x: 0.0, y: 0.0, z: 1.0 };
        }

        // cross product between vectors
        result.x = v.y * cardinal_axis.z - v.z * cardinal_axis.y;
        result.y = v.z * cardinal_axis.x - v.x * cardinal_axis.z;
        result.z = v.x * cardinal_axis.y - v.y * cardinal_axis.x;

        return result;
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
