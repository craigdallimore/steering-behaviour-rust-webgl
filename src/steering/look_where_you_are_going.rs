use crate::domain::{steering::Steering, kinematic::Kinematic};
use crate::vector::Vector;

pub struct LookWhereYouAreGoing {}

impl LookWhereYouAreGoing {
  pub fn new() -> LookWhereYouAreGoing {
    LookWhereYouAreGoing {}
  }

  pub fn calculate(self: &Self, kinematic: Kinematic) -> Steering {
    Steering {
      linear: Vector(0.0, 0.0),
      angular: 0.0
    }

  }

}

