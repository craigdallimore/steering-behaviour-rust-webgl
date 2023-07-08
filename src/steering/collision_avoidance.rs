use crate::domain::{kinematic::Kinematic, steering::Steering};
use crate::vector::Vector;

pub struct CollisionAvoidance {}

impl CollisionAvoidance {
    pub fn new() -> CollisionAvoidance {
        CollisionAvoidance {}
    }

    pub fn calculate(self: &Self, kinematic: Kinematic) -> Steering {
        Steering {
            linear: Vector(0.0, 0.0),
            angular: 0.0,
        }
    }
}
