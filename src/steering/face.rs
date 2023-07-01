use crate::{domain::{kinematic::Kinematic, steering::Steering}, vector::Vector};

use super::align::Align;

pub struct Face {
  align: Align
}

impl Face {

  pub fn new() -> Face {
    Face {
      align: Align::new()
    }
  }

  pub fn calculate(self: &Self, kinematic: Kinematic, position: Vector) -> Steering {
    let direction = position - kinematic.position;

    if Vector::length(&direction) == 0.0 {
      return Steering {
        angular: 0.0,
        linear: Vector(0.0, 0.0)
      }
    }

    let next_orientation = direction.1.atan2(direction.0);

    self.align.calculate(kinematic, next_orientation)

  }

}
