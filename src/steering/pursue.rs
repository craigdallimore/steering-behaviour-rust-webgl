use crate::domain::{kinematic::Kinematic, steering::Steering};
use crate::vector::Vector;

use super::seek::Seek;

pub struct Pursue {
    max_prediction: f32,
    seek: Seek,
}

impl Pursue {
    pub fn new() -> Pursue {
        Pursue {
            max_prediction: 1.0,
            seek: Seek::new(),
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

        self.seek.calculate(kinematic, next_target_position)
    }
}
