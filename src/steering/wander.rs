use crate::domain::{kinematic::Kinematic, steering::Steering};
use crate::vector::Vector;
use rand::Rng;

pub struct Wander {
    wander_offset: f32,
    wander_radius: f32,
    wander_rate: f32,
    wander_orientation: f32,
}

impl Wander {
    pub fn new() -> Wander {
        Wander {
            wander_offset: 40.0,
            wander_radius: 20.0,
            wander_rate: 0.4,
            wander_orientation: 0.0,
        }
    }

    pub fn calculate(self: &mut Self, kinematic: Kinematic) -> Steering {
        let mut rng = rand::thread_rng();

        let random: f32 = rng.gen::<f32>() - rng.gen::<f32>();

        self.wander_orientation += random + self.wander_rate;

        let target_orientation = self.wander_orientation + kinematic.orientation;
        let target_orientation_as_vector = Vector::from_radians(target_orientation);

        // Anchor for the wander circle
        let circle_center_position =
            Vector::from_radians(kinematic.orientation) * self.wander_offset;

        let next_target_position =
            circle_center_position + (target_orientation_as_vector * self.wander_radius);

        let linear = next_target_position * kinematic.max_acceleration;

        Steering {
            linear,
            angular: 0.0,
        }
    }
}
