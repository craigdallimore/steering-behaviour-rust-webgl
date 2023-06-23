use super::kinematic::Kinematic;

pub struct Character {
  pub kinematic: Kinematic,
  pub label: Option<String>
}

impl Character {
  pub fn new(kinematic: Kinematic) -> Character {
    Character {
      kinematic,
      label: None
    }
  }
}
