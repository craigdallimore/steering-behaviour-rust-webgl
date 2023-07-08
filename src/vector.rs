use float_cmp::approx_eq;
use std::ops;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector(pub f32, pub f32);

impl Vector {
    fn dot(a: &Vector, b: &Vector) -> f32 {
        a.0 * b.0 + a.1 * b.1
    }
    fn cross(a: &Vector, b: &Vector) -> f32 {
        a.0 * b.1 - a.1 * b.0
    }
    pub fn length(v: &Vector) -> f32 {
        v.0.hypot(v.1)
    }
    pub fn to_radians(v: &Vector) -> f32 {
        let pi = std::f32::consts::PI;
        let n = v.1.atan2(0.0 - v.0);

        if n >= pi {
            n - pi
        } else {
            n + pi
        }
    }
    fn from_radians(rad: f32) -> Vector {
        Vector(rad.sin(), rad.cos())
    }
    fn from_degrees(angle: f32) -> Vector {
        let theta = (angle * std::f32::consts::PI) / 180.0;
        Vector(theta.cos(), theta.sin())
    }
    pub fn normalise(v: &Vector) -> Vector {
        let l = Vector::length(&v);
        if l == 0.0 {
            return Vector(v.0, v.1);
        }
        let factor = 1.0 / l;
        let v2 = v.clone();
        v2 * factor
    }
    fn distance(a: Vector, b: Vector) -> f32 {
        let v = a - b;
        Vector::length(&v)
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, rhs: Vector) -> Vector {
        Vector(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl ops::Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Vector {
        Vector(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl ops::Mul<f32> for Vector {
    type Output = Vector;
    fn mul(self, rhs: f32) -> Vector {
        Vector(self.0 * rhs, self.1 * rhs)
    }
}
impl ops::MulAssign<f32> for Vector {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
        self.1 *= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let a = Vector(1.0, 2.0);
        let b = Vector(3.0, 4.5);

        let left = a + b;
        let right = Vector(4.0, 6.5);

        assert_eq!(left, right);
    }
    #[test]
    fn test_subtract() {
        let a = Vector(1.0, 2.0);
        let b = Vector(3.0, 4.5);

        let left = a - b;
        let right = Vector(-2.0, -2.5);

        assert_eq!(left, right);
    }
    #[test]
    fn test_mult() {
        let a = Vector(1.0, 2.0);

        let left = a * 2.0;
        let right = Vector(2.0, 4.0);

        assert_eq!(left, right);
    }
    #[test]
    fn test_dot() {
        let a = Vector(1.0, 2.0);
        let b = Vector(3.0, 4.5);
        assert_eq!(Vector::dot(&a, &b), 12.0);
    }
    #[test]
    fn test_cross() {
        let a = Vector(1.0, 2.0);
        let b = Vector(3.0, 4.5);
        assert_eq!(Vector::cross(&a, &b), -1.5);
    }
    #[test]
    fn test_length() {
        let v = Vector(3.0, 4.0);
        assert_eq!(Vector::length(&v), 5.0);
    }
    #[test]
    fn test_to_radians() {
        let pi = std::f32::consts::PI;

        let rad_east = 0.0;
        let rad_ne = pi * 0.25;
        let rad_north = pi * 0.5;
        let rad_nw = pi * 0.75;
        let rad_west = pi;
        let rad_sw = pi * 1.25;
        let rad_south = pi * 1.5;
        let rad_se = pi * 1.75;

        let vec_e = Vector(1.0, 0.0);
        let vec_ne = Vector(1.0, -1.0);
        let vec_n = Vector(0.0, -1.0);
        let vec_nw = Vector(-1.0, -1.0);
        let vec_w = Vector(-1.0, 0.0);
        let vec_sw = Vector(-1.0, 1.0);
        let vec_s = Vector(0.0, 1.0);
        let vec_se = Vector(1.0, 1.0);

        assert!(approx_eq!(f32, Vector::to_radians(&vec_e), rad_east));
        assert!(approx_eq!(f32, Vector::to_radians(&vec_ne), rad_ne));
        assert!(approx_eq!(f32, Vector::to_radians(&vec_n), rad_north));
        assert!(approx_eq!(f32, Vector::to_radians(&vec_nw), rad_nw));
        assert!(approx_eq!(f32, Vector::to_radians(&vec_w), rad_west));
        assert!(approx_eq!(f32, Vector::to_radians(&vec_sw), rad_sw));
        assert!(approx_eq!(f32, Vector::to_radians(&vec_s), rad_south));
        assert!(approx_eq!(f32, Vector::to_radians(&vec_se), rad_se));
    }
    #[test]
    fn test_from_radians() {
        // same as node
        let v = Vector(-0.9589243, 0.2836622);
        assert_eq!(Vector::from_radians(5.0), v);
    }
    /*
    #[test]
    fn test_to_from_radians() {
        let v = Vector::from_radians(4.0);
        let r = Vector::to_radians(&v);
        assert_eq!(r, 4.0);
    }
    */
    #[test]
    fn test_from_degrees() {
        let v = Vector(0.9961947, 0.087155744);
        assert_eq!(Vector::from_degrees(5.0), v);
    }
    #[test]
    fn test_normalise() {
        let v = Vector(5.0, 5.0);
        assert_eq!(Vector::normalise(&v), Vector(0.7071068, 0.7071068));
    }
    #[test]
    fn test_distance() {
        let a = Vector(1.0, 1.0);
        let b = Vector(4.0, 5.0);
        assert_eq!(Vector::distance(a, b), 5.0);
    }
}
