use crate::domain::{kinematic::Kinematic, steering::Steering};
use crate::vector::Vector;

use super::align::Align;

pub struct LookWhereYouAreGoing {
    align: Align,
}

impl LookWhereYouAreGoing {
    pub fn new() -> LookWhereYouAreGoing {
        LookWhereYouAreGoing {
            align: Align::new(),
        }
    }

    pub fn calculate(self: &Self, kinematic: Kinematic) -> Steering {
        if Vector::length(&kinematic.velocity) == 0.0 {
            Steering {
                linear: Vector(0.0, 0.0),
                angular: 0.0,
            }
        } else {
            let orientation = Vector::to_radians(&kinematic.velocity);
            return self.align.calculate(kinematic, orientation);
        }
    }
}
