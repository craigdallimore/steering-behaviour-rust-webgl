use crate::domain::{kinematic::Kinematic, steering::Steering};
use crate::vector::Vector;

pub struct Separation {
    threshold: f32,
    decayCoefficient: f32,
}

impl Separation {
    pub fn new() -> Separation {
        Separation {
            // The threshold to take action
            threshold: 250.0,
            // Holds the constant coefficient of decay for the inverse square law force
            decayCoefficient: 1500.0,
        }
    }

    pub fn calculate(self: &Self, kinematic: Kinematic, targets: Vec<&Kinematic>) -> Steering {
        let init = Steering {
            linear: Vector(0.0, 0.0),
            angular: 0.0,
        };

        return targets.iter().fold(init, |acc, target| {
            let direction = kinematic.position - target.position;
            let distance = Vector::length(&direction);

            if distance < self.threshold {
                let decay = self.decayCoefficient / distance.powi(2);
                let strength = decay.min(kinematic.max_acceleration);
                let normal_direction = Vector::normalise(&direction);
                let linear = normal_direction * strength;
                let linear = acc.linear + linear;

                return Steering {
                    angular: 0.0,
                    linear,
                };
            }

            acc
        });
    }
}
