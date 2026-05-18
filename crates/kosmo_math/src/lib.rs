pub mod integrator;

use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn magnitude(self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(self) -> Self {
        let magnitude = self.magnitude();

        const TINY: f64 = 1e-10;

        if magnitude < TINY {
            return Self { x: 0.0, y: 0.0 };
        }

        self / magnitude
    }

    pub fn distance_and_direction(self, other: Vec2) -> (f64, Vec2) {
        let in_dist = other - self;

        let distance = in_dist.magnitude();

        const TINY: f64 = 1e-10;

        if distance < TINY {
            return (0.0, Vec2::new(0.0, 0.0));
        }

        let direction = in_dist / distance;

        (distance, direction)
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, coeff: f64) -> Self::Output {
        Self {
            x: self.x * coeff,
            y: self.y * coeff,
        }
    }
}

impl Mul<Vec2> for f64 {
    type Output = Vec2;

    fn mul(self, v: Vec2) -> Self::Output {
        Vec2 {
            x: v.x * self,
            y: v.y * self,
        }
    }
}

impl Div<f64> for Vec2 {
    type Output = Vec2;

    fn div(self, divisor: f64) -> Self::Output {
        Self {
            x: self.x / divisor,
            y: self.y / divisor,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TINY: f64 = 1e-10;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < TINY
    }

    #[test]
    fn test_vec2_addition() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);

        let result = a + b;

        assert_eq!(result, Vec2::new(4.0, 6.0));
    }

    #[test]
    fn test_vec2_subtraction() {
        let a = Vec2::new(5.0, 7.0);
        let b = Vec2::new(2.0, 3.0);

        let result = a - b;

        assert_eq!(result, Vec2::new(3.0, 4.0));
    }

    #[test]
    fn test_vec2_scalar_multiplication() {
        let v = Vec2::new(2.0, 3.0);

        let result = v * 4.0;

        assert_eq!(result, Vec2::new(8.0, 12.0));
    }

    #[test]
    fn test_scalar_vec2_multiplication() {
        let v = Vec2::new(2.0, 3.0);

        let result = 4.0 * v;

        assert_eq!(result, Vec2::new(8.0, 12.0));
    }

    #[test]
    fn test_vec2_division() {
        let v = Vec2::new(8.0, 12.0);

        let result = v / 4.0;

        assert_eq!(result, Vec2::new(2.0, 3.0));
    }

    #[test]
    fn test_vec2_magnitude() {
        let v = Vec2::new(3.0, 4.0);

        let result = v.magnitude();

        assert!(approx_eq(result, 5.0));
    }

    #[test]
    fn test_vec2_normalization() {
        let v = Vec2::new(3.0, 4.0);

        let normalized = v.normalize();

        assert!(approx_eq(normalized.x, 0.6));
        assert!(approx_eq(normalized.y, 0.8));
    }

    #[test]
    fn test_zero_vector_normalization() {
        let v = Vec2::new(0.0, 0.0);

        let normalized = v.normalize();

        assert_eq!(normalized, Vec2::new(0.0, 0.0));
    }

    #[test]
    fn test_normalized_vector_has_unit_length() {
        let v = Vec2::new(10.0, 15.0);

        let normalized = v.normalize();

        assert!(approx_eq(normalized.magnitude(), 1.0));
    }

    #[test]
    fn test_copy_trait() {
        let a = Vec2::new(1.0, 2.0);

        let b = a;

        assert_eq!(a, b);
    }

    #[test]
    fn test_clone_trait() {
        let a = Vec2::new(7.0, 9.0);

        let b = a.clone();

        assert_eq!(a, b);
    }

    #[test]
    fn test_distance_and_direction() {
        let a = Vec2::new(0.0, 0.0);
        let b = Vec2::new(3.0, 4.0);

        let (distance, direction) = a.distance_and_direction(b);

        assert!(approx_eq(distance, 5.0));

        assert!(approx_eq(direction.x, 0.6));
        assert!(approx_eq(direction.y, 0.8));
    }
}
