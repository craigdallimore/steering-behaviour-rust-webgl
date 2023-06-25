use crate::domain::{steering::{Behaviour, Steering}, kinematic::Kinematic};
use crate::vector::Vector;

fn map_to_range(orientation: f32) -> f32 {
  let pi = std::f32::consts::PI;
  // To rotate all the way clockwise, use the value 6.283
  // 6.3 is roughtly NE 0.01 I expect.
  let next_orientation = if orientation.abs() > pi {
    orientation - pi * 2.0 * orientation.signum()
  } else {
    orientation
  };

  next_orientation % (pi * 2.0)
}

pub struct Align {
  align_tolerance: f32,
  max_rotation: f32,
  deceleration_tolerance: f32,
  time_to_target: f32
}


impl Behaviour for Align {
  type Args = f32;

  fn new() -> Align {
    Align {
      max_rotation: 120.0,
      deceleration_tolerance: 2.0,
      align_tolerance: 0.01,
      time_to_target: 0.1
    }
  }

  fn calculate(self: &Self, kinematic: Kinematic, orientation: Self::Args) -> Steering {
    let linear = Vector(0.0, 0.0);

    let rotation = map_to_range(orientation - kinematic.orientation);
    let rotation_size = rotation.abs();

    if rotation_size > self.align_tolerance {
      return Steering {
        linear,
        angular: 0.0
      };
    }

    let is_slowed = rotation_size <= self.deceleration_tolerance;
    let ideal_rotation = if is_slowed {
      (self.max_rotation * rotation_size) / self.deceleration_tolerance
    } else {
      self.max_rotation
    };


    let next_ideal_rotation = ideal_rotation * (rotation / rotation_size);
    let angular = (next_ideal_rotation - kinematic.rotation) / self.time_to_target;

    let angular_acceleration = angular.abs();
    let final_angular = if angular_acceleration > kinematic.max_angular_acceleration {
      (angular * kinematic.max_angular_acceleration) / angular_acceleration
    } else {
      angular
    };

    Steering {
      angular: final_angular,
      linear
    }
  }

}
