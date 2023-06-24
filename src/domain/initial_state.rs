use crate::vector::Vector;
use crate::domain::kinematic::Kinematic;
use crate::domain::character::Character;

pub struct State {
  pub characters: Vec<Character>
}

pub enum Action {
  Tick(f32)
}

impl State {
  pub fn new() -> State {
    State {
      characters: vec![
        Character::new(
          Kinematic {
            max_acceleration: 25.0,
            max_angular_acceleration: 140.0,
            max_speed: 45.0,
            position: Vector(400.0, 400.0),
            orientation: 0.0,
            velocity: Vector(0.0, 0.0),
            rotation: 0.0
          },
          vec![]
        ),
        Character::new(
          Kinematic {
            max_acceleration: 25.0,
            max_angular_acceleration: 140.0,
            max_speed: 45.0,
            position: Vector(500.0, 400.0),
            orientation: 1.5708,
            velocity: Vector(0.0, 0.0),
            rotation: 0.0
          },
          vec![]
        )
      ]
    }
  }
  pub fn dispatch(self: &mut State, action: Action) -> &State {
    match action {
      Action::Tick(tick) => {

        for (_i, char) in self.characters.iter().enumerate() {
          char.apply_behaviours(tick);
        }

        self
      }
    }
  }
}
