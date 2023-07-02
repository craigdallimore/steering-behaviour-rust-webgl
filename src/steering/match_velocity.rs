use crate::domain::{steering::Steering, kinematic::Kinematic};
use crate::vector::Vector;

pub struct MatchVelocity {}

impl MatchVelocity {
  pub fn new() -> MatchVelocity {
    MatchVelocity {}
  }

  pub fn calculate(self: &Self, kinematic: Kinematic) -> Steering {
    Steering {
      linear: Vector(0.0, 0.0),
      angular: 0.0
    }

  }

}

