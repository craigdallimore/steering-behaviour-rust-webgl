use crate::domain::{kinematic::Kinematic, steering::Steering};
use crate::vector::Vector;

pub struct Arrive {
    target_radius: f32,
    slow_radius: f32,
    time_to_target: f32,
}

impl Arrive {
    pub fn new() -> Arrive {
        Arrive {
            target_radius: 5.0,
            time_to_target: 3.0,
            slow_radius: 60.0,
        }
    }

    pub fn calculate(
        self: &Self,
        kinematic: Kinematic,
        target_postion: Vector,
    ) -> Option<Steering> {
        let distance_to_target = Vector::distance(kinematic.position, target_postion);
        let direction_to_target = target_postion - kinematic.position;

        if distance_to_target < self.target_radius {
            return None;
        }

        let ideal_speed = if distance_to_target > self.slow_radius {
            kinematic.max_speed
        } else {
            kinematic.max_speed + (distance_to_target / self.slow_radius)
        };

        // Here we appear to take a vector from the two points, and relate it to
        // the ideal speed

        let ideal_velocity = Vector::normalise(&direction_to_target) * ideal_speed;
        let reduced = ideal_velocity - kinematic.velocity;

        // A higher value will arrive sooner
        let linear = reduced * (1.0 / self.time_to_target);

        let linear = if Vector::length(&linear) > kinematic.max_acceleration {
            Vector::normalise(&linear) * kinematic.max_acceleration
        } else {
            linear
        };

        Some(Steering {
            linear,
            angular: 0.0,
        })
    }
}
