use crate::domain::{kinematic::Kinematic, steering::Steering};
use crate::vector::Vector;

use super::helpers::map_to_range;

pub struct Align {
    align_tolerance: f32,
    max_rotation: f32,
    deceleration_tolerance: f32,
    time_to_target: f32,
}

impl Align {
    pub fn new() -> Align {
        Align {
            max_rotation: 120.0,
            deceleration_tolerance: 2.0,
            align_tolerance: 0.01,
            time_to_target: 0.1,
        }
    }

    pub fn calculate(self: &Self, kinematic: Kinematic, orientation: f32) -> Steering {
        let linear = Vector(0.0, 0.0);

        // what change in orientation is needed
        let rotation = map_to_range(orientation - kinematic.orientation);

        // how much should it change
        let rotation_size = rotation.abs();

        // do nothing, if the amount to change is trivial
        if rotation_size < self.align_tolerance {
            return Steering {
                linear,
                angular: 0.0,
            };
        }

        let should_decelerate = rotation_size <= self.deceleration_tolerance;
        let ideal_rotation = if should_decelerate {
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
            linear,
        }
    }
}
