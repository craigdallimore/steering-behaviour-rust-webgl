use crate::vector::Vector;
use crate::domain::kinematic::Kinematic;
use crate::domain::character::Character;

pub struct State {
  pub characters: Vec<Character>
}

pub enum Action {
  Tick(f64)
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
  pub fn dispatch(self: &mut State, action: Action) -> &State {
    match action {
      Action::Tick(tick) => {
        self.characters[0].kinematic.orientation += tick as f32;
        self
      }
    }
  }
}
