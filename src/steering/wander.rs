use crate::domain::{steering::Steering, kinematic::Kinematic};
use crate::vector::Vector;

pub struct Wander {}

impl Wander {
  pub fn new() -> Wander {
    Wander {}
  }

  pub fn calculate(self: &Self, kinematic: Kinematic) -> Steering {
    Steering {
      linear: Vector(0.0, 0.0),
      angular: 0.0
    }

  }

}

