use crate::domain::{kinematic::Kinematic, steering::Steering};
use crate::vector::Vector;

pub struct FollowPathChasePredict {}

impl FollowPathChasePredict {
    pub fn new() -> FollowPathChasePredict {
        FollowPathChasePredict {}
    }

    pub fn calculate(self: &Self, _kinematic: Kinematic) -> Steering {
        Steering {
            linear: Vector(0.0, 0.0),
            angular: 0.0,
        }
    }
}
