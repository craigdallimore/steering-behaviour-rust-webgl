use crate::steering::align::Align;
use crate::steering::face::Face;
use crate::vector::Vector;
use crate::domain::kinematic::Kinematic;
use crate::domain::character::Character;
use crate::domain::steering::Behaviour;

pub struct State {
  pub characters: Vec<Character>
}

pub enum Action {
  Tick(f32)
}

impl State {
  pub fn new() -> State {

    let pi = std::f32::consts::PI;
    let east = 0.0; // east
    let west = pi; // west
    let north = pi * 0.5; // south
    let south = pi * 1.5; // north???
    State {
      characters: vec![
        Character::new(
          Kinematic {
            max_acceleration: 25.0,
            max_angular_acceleration: 140.0,
            max_speed: 45.0,
            position: Vector(400.0, 400.0),
            orientation: south,
            velocity: Vector(0.0, 0.0),
            rotation: 0.0
          },
          vec![
            //Behaviour::Align(Align::new())
          ]
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
          vec![
            Behaviour::Face(Face::new())
          ]
        )
      ]
    }
  }
  pub fn dispatch(self: &mut State, action: Action) -> () {
    match action {
      Action::Tick(tick) => {

        for (_i, char) in self.characters.iter_mut().enumerate() {
          char.apply_behaviours(tick);
        }

      }
    }
  }
}
