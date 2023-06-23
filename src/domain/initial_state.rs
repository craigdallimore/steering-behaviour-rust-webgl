use crate::vector::Vector;
use crate::domain::kinematic::Kinematic;
use crate::domain::character::Character;

pub struct State {
  characters: Vec<Character>
}

impl State {
  pub fn new() -> State {
    State {
      characters: vec![
        Character::new(
          Kinematic {
            position: Vector(400.0, 400.0),
            orientation: 0.0,
            velocity: Vector(0.0, 0.0),
            rotation: 0.0
          }
        ),
        Character::new(
          Kinematic {
            position: Vector(500.0, 400.0),
            orientation: 1.5708,
            velocity: Vector(0.0, 0.0),
            rotation: 0.0
          }
        )
      ]
    }
  }


}
