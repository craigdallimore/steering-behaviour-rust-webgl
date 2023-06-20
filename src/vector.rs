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
    fn length(v: &Vector) -> f32 {
        v.0.hypot(v.1)
    }
    // TODO: check if this is legit
    fn to_radians(v: &Vector) -> f32 {
        v.0.atan2(0.0 - v.1)
    }
    fn from_radians(rad: f32) -> Vector {
        Vector(rad.sin(), rad.cos())
    }
    fn from_degrees(angle: f32) -> Vector {
        let theta = (angle * std::f32::consts::PI) / 180.0;
        Vector(theta.cos(), theta.sin())
    }
    fn normalise(v: &Vector) -> Vector {
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
    fn add(self, _rhs: Vector) -> Vector {
        Vector(self.0 + _rhs.0, self.1 + _rhs.1)
    }
}
impl ops::Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, _rhs: Vector) -> Vector {
        Vector(self.0 - _rhs.0, self.1 - _rhs.1)
    }
}
impl ops::Mul<f32> for Vector {
    type Output = Vector;
    fn mul(self, _rhs: f32) -> Vector {
        Vector(self.0 * _rhs, self.1 * _rhs)
    }
}
impl ops::MulAssign<f32> for Vector {
    fn mul_assign(&mut self, _rhs: f32) {
        self.0 *= _rhs;
        self.1 *= _rhs;
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
        let v = Vector(3.0, 4.0);
        // same as node
        assert_eq!(Vector::to_radians(&v), 2.4980917);
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
