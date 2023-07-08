use crate::domain::{kinematic::Kinematic, steering::Steering};
use crate::vector::Vector;

pub struct Evade {}

impl Evade {
    pub fn new() -> Evade {
        Evade {}
    }

    pub fn calculate(self: &Self, kinematic: Kinematic) -> Steering {
        Steering {
            linear: Vector(0.0, 0.0),
            angular: 0.0,
        }
    }
}
