use crate::domain::{kinematic::Kinematic, steering::Steering};
use crate::vector::Vector;

pub struct FollowPathChaseRabbit {}

impl FollowPathChaseRabbit {
    pub fn new() -> FollowPathChaseRabbit {
        FollowPathChaseRabbit {}
    }

    pub fn calculate(self: &Self, _kinematic: Kinematic) -> Steering {
        Steering {
            linear: Vector(0.0, 0.0),
            angular: 0.0,
        }
    }
}
