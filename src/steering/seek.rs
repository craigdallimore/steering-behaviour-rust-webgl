use crate::{domain::{kinematic::Kinematic, steering::Steering}, vector::Vector};

pub struct Seek {}

impl Seek {
  pub fn new() -> Seek {
    Seek {}
  }
  pub fn calculate(self: &Self, kinematic: Kinematic, target_position: Vector) -> Steering {
    let angular = 0.0;

    let linear = target_position - kinematic.position;
    let linear = Vector::normalise(&linear);
    let linear = linear * kinematic.max_acceleration;

    Steering { linear, angular }

  }
}
