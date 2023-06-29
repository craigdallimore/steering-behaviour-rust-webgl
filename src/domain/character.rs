use super::{kinematic::Kinematic, steering::Behaviour};

pub struct Character {
  pub behaviours: Vec<Behaviour>,
  pub kinematic: Kinematic,
  pub label: Option<String>,
}

impl Character {
  pub fn new(kinematic: Kinematic, behaviours: Vec<Behaviour>) -> Character {
    Character {
      behaviours,
      kinematic,
      label: None,
    }
  }
  pub fn apply_behaviours(self: &mut Self, tick: f32) {

    let target_orientation = 0.0;

    for (_i, behaviour) in self.behaviours.iter_mut().enumerate() {
      match behaviour {
        Behaviour::Align(align) => {
          let steering = align.calculate(self.kinematic, target_orientation);
          self.kinematic.update(steering, tick);
        }
      }


    }


  }
}
