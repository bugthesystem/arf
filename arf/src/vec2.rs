use std::f64::consts::PI;
use std::ops::{Add, Div, Mul, Neg, Sub};

use libm::atan2;

/// `Vector2` type
#[derive(Copy, Clone, Debug)]
pub struct Vector2 {
    pub(crate) x: f64,
    pub(crate) y: f64,
}

impl Vector2 {
    /// new vector
    #[inline(always)]
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Vector with components value 0.0f
    #[inline(always)]
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// Vector with components value 1.0f
    #[inline(always)]
    pub fn one() -> Self {
        Self { x: 1.0, y: 1.0 }
    }

    /// Add two vectors (v1 + v2)
    #[inline(always)]
    pub fn add(&self, v2: Self) -> Self {
        Self { x: self.x + v2.x, y: self.y + v2.y }
    }

    /// Add vector and float value
    #[inline(always)]
    pub fn add_value(&self, add: f64) -> Self {
        Self { x: self.x + add, y: self.y + add }
    }

    /// Subtract two vectors (v1 - v2)
    #[inline(always)]
    pub fn subtract(&self, v2: Self) -> Self {
        Self { x: self.x - v2.x, y: self.y - v2.y }
    }

    /// Subtract vector by float value
    #[inline(always)]
    pub fn subtract_value(&self, sub: f64) -> Self {
        Self { x: self.x - sub, y: self.y - sub }
    }

    /// Calculate vector length
    #[inline(always)]
    pub fn length(&self) -> f64 {
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }

    /// Calculate vector square length
    #[inline(always)]
    pub fn length_sqr(&self) -> f64 {
        (self.x * self.x) + (self.y * self.y)
    }

    /// Calculate two vectors dot product
    #[inline(always)]
    pub fn dot_product(&self, v2: Vector2) -> f64 {
        self.x * v2.x + self.y * v2.y
    }

    /// Calculate distance between two vectors
    #[inline]
    pub fn distance(&self, v2: Vector2) -> f64 {
        let result: f64 = (
            (self.x - v2.x) * (self.x - v2.x)
                + (self.y - v2.y) * (self.y - v2.y)
        ).sqrt();

        return result;
    }

    /// Calculate angle from two vectors in X-axis
    #[inline]
    pub fn angle(&self, v2: Vector2) -> f64 {
        let mut result: f64 = atan2(v2.y - self.y, v2.x - self.x) * (180.0 / PI);

        if result < 0.0 {
            result += 360.0;
        }

        return result;
    }

    /// Scale vector (multiply by value)
    #[inline(always)]
    pub fn scale(&self, scale: f64) -> Vector2 {
        Vector2 { x: self.x * scale, y: self.y * scale }
    }

    /// Multiply by vector
    #[inline(always)]
    pub fn multiply(&self, v2: Vector2) -> Self {
        Self { x: self.x * v2.x, y: self.y * v2.y }
    }

    /// Negate vector
    #[inline(always)]
    pub fn negate(&self) -> Self {
        Self { x: -self.x, y: -self.y }
    }

    /// Divide by vector
    #[inline(always)]
    pub fn divide(&self, v2: Vector2) -> Self {
        Self { x: self.x / v2.x, y: self.y / v2.y }
    }

    /// Normalize vector
    #[inline]
    pub fn normalize(&self) -> Vector2 {
        let mut result: Vector2 = Vector2::zero();

        let length: f64 = ((self.x * self.x) + (self.y * self.y)).sqrt();

        if length > 0.0 {
            result.x = self.x * 1.0 / length;
            result.y = self.y * 1.0 / length;
        }

        return result;
    }


    /// Calculate linear interpolation between two vectors
    #[inline]
    pub fn lerp(&self, v2: Vector2, amount: f64) -> Vector2 {
        let mut result = Vector2::zero();

        result.x = self.x + amount * (v2.x - self.x);
        result.y = self.y + amount * (v2.y - self.y);

        return result;
    }

    /// Calculate reflected vector to normal
    #[inline]
    pub fn reflect(&self, normal: Vector2) -> Vector2 {
        let mut result = Vector2::zero();

        let dot_product: f64 = self.x * normal.x + self.y * normal.y; // Dot product

        result.x = self.x - (2.0 * normal.x) * dot_product;
        result.y = self.y - (2.0 * normal.y) * dot_product;

        return result;
    }

    /// Rotate vector by angle
    #[inline]
    pub fn rotate(&self, angle: f64) -> Vector2 {
        let mut result = Vector2::zero();

        result.x = self.x * angle.cos() - self.y * angle.sin();
        result.y = self.x * angle.sin() + self.y * angle.cos();

        return result;
    }

    /// Move Vector towards target
    #[inline]
    pub fn move_towards(&self, target: Vector2, max_distance: f64) -> Vector2 {
        let mut result = Vector2::zero();

        let dx: f64 = target.x - self.x;
        let dy: f64 = target.y - self.y;
        let value: f64 = (dx * dx) + (dy * dy);

        if (value == 0.0) || ((max_distance >= 0.0) && (value <= max_distance * max_distance)) {
            return target;
        }

        let dist: f64 = value.sqrt();

        result.x = self.x + dx / dist * max_distance;
        result.y = self.y + dy / dist * max_distance;

        return result;
    }
}

/// The addition operator +
impl Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

/// The subtraction operator -
impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

/// The division operator /
impl Div for Vector2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self { x: self.x / rhs.x, y: self.y / rhs.y }
    }
}

/// The multiplication operator *
impl Mul for Vector2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}

/// The unary negation operator -
impl Neg for Vector2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { x: -self.x, y: -self.y }
    }
}

impl PartialEq for Vector2 {
    fn eq(&self, rhs: &Self) -> bool {
        self.x == rhs.x && self.y == rhs.y
    }
}


#[cfg(test)]
mod tests {
    use crate::vec2::Vector2;

    #[test]
    fn test_add() {
        let v1 = Vector2::new(6.0, 4.0);
        let v2 = Vector2::new(4.0, 2.0);

        let actual = v1 + v2;
        let expected = Vector2::new(10.0, 6.0);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_sub() {
        let v1 = Vector2::new(6.0, 4.0);
        let v2 = Vector2::new(4.0, 2.0);

        let actual = v1 - v2;
        let expected = Vector2::new(2.0, 2.0);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_div() {
        let v1 = Vector2::new(6.0, 4.0);
        let v2 = Vector2::new(3.0, 2.0);

        let actual = v1 / v2;
        let expected = Vector2::new(2.0, 2.0);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_mul() {
        let v1 = Vector2::new(6.0, 4.0);
        let v2 = Vector2::new(3.0, 2.0);

        let actual = v1 * v2;
        let expected = Vector2::new(18.0, 8.0);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_distance() {
        let v1 = Vector2::new(2.0, 1.0);
        let v2 = Vector2::new(5.0, 5.0);

        let result = v1.distance(v2);
        assert_eq!(result, 5.0);
    }
}


