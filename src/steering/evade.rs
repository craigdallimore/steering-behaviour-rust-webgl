use crate::domain::{kinematic::Kinematic, steering::Steering};
use crate::vector::Vector;

use super::flee::Flee;

pub struct Evade {
    max_prediction: f32,
    flee: Flee,
}

impl Evade {
    pub fn new() -> Evade {
        Evade {
            max_prediction: 1.0,
            flee: Flee::new(),
        }
    }

    pub fn calculate(self: &Self, kinematic: Kinematic, target: Kinematic) -> Steering {
        let direction = target.position - kinematic.position;
        let distance = Vector::length(&direction);
        let speed = Vector::length(&kinematic.velocity);

        let prediction = if speed <= distance / self.max_prediction {
            self.max_prediction
        } else {
            distance / speed
        };

        let next_target_position = target.position + (target.velocity * prediction);

        self.flee.calculate(kinematic, next_target_position)
    }
}
