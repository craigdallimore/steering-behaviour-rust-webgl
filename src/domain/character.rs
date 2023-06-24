use crate::steering::align::Align;

use super::{kinematic::Kinematic, steering::Behaviour};

type AlignArgs = f32;

pub struct Character {
  pub behaviours: Vec<Align>,
  pub kinematic: Kinematic,
  pub label: Option<String>,
}

impl Character {
  pub fn new(kinematic: Kinematic, behaviours: Vec<Align>) -> Character {
    Character {
      behaviours,
      kinematic,
      label: None,
    }
  }
  pub fn apply_behaviour(self: &Self, tick: f32) {

    let target_orientation = 0.0;

    if self.behaviours.len() > 0 {
      let steering = self.behaviours[0].calculate(self.kinematic, target_orientation);
      self.kinematic.update(steering, tick);
    }


  }
}
