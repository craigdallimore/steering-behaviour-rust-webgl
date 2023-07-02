use crate::domain::{steering::Steering, kinematic::Kinematic};
use crate::vector::Vector;

pub struct Flee {}

impl Flee {
  pub fn new() -> Flee {
    Flee {}
  }

  pub fn calculate(self: &Self, kinematic: Kinematic, target_postion: Vector) -> Steering {
    let angular = 0.0;

    let linear = kinematic.position - target_postion;
    let linear = Vector::normalise(&linear);
    let linear = linear * kinematic.max_acceleration;

    Steering { linear, angular }

  }

}

