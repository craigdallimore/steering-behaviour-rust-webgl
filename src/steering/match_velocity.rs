use crate::domain::{steering::Steering, kinematic::Kinematic};
use crate::vector::Vector;

pub struct MatchVelocity {
  time_to_target: f32
}

impl MatchVelocity {
  pub fn new() -> MatchVelocity {
    MatchVelocity {
      time_to_target: 0.1
    }
  }

  pub fn calculate(self: &Self, kinematic: Kinematic, target: Kinematic) -> Steering {

    let linear = target.velocity - kinematic.velocity;
    let divided_linear = linear * (1.0 / self.time_to_target);
    let linear = if Vector::length(&divided_linear) > kinematic.max_acceleration {
      Vector::normalise(&linear) * kinematic.max_acceleration
    } else {
      linear
    };

    Steering {
      linear,
      angular: 0.0
    }

  }

}

