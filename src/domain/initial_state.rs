use crate::domain::character::Character;
use crate::domain::kinematic::Kinematic;
use crate::domain::steering::Behaviour;
use crate::steering::{
  //align::Align,
  //arrive::Arrive,
  //collision_avoidance::CollisionAvoidance,
  //evade::Evade,
  //face::Face,
  //flee::Flee,
  //follow_path_chase_rabbit::FollowPathChaseRabbit,
  //follow_path_predict::FollowPathChasePredict,
  //look_where_you_are_going::LookWhereYouAreGoing,
  //match_velocity::MatchVelocity,
  //obstacle_avoidance::ObstacleAvoidance,
  //pursue::Pursue,
  //seek::Seek,
    separation::Separation,
  //  wander::Wander,
};
use crate::vector::Vector;

use super::steering::Steering;

pub struct State {
    pub characters: Vec<Character>,
}

pub enum Action {
    Tick(f32),
}

impl State {
    pub fn new() -> State {
        let pi = std::f32::consts::PI;

        let east = 0.0;
        let west = pi;
        let north = pi * 0.5;
        let south = pi * 1.5;

     // let nw = Vector(-1.0, -1.0);
     // let ne = Vector(1.0, -1.0);
     // let sw = Vector(-1.0, 1.0);
     // let se = Vector(1.0, 1.0);

        let c1 = Character::new(
            Kinematic {
                position: Vector(750.0, 800.0),
                orientation: west,
                ..Kinematic::default()
            },
            vec![Behaviour::Separation(Separation::new())]
        );
        let c2 = Character::new(
            Kinematic {
                position: Vector(400.0, 360.0),
                orientation: north,
                ..Kinematic::default()
            },
            vec![],
        );
        let c3 = Character::new(
            Kinematic {
                position: Vector(400.0, 440.0),
                orientation: south,
                ..Kinematic::default()
            },
            vec![],
        );
        let c4 = Character::new(
            Kinematic {
                position: Vector(360.0, 400.0),
                orientation: west,
                ..Kinematic::default()
            },
            vec![],
        );
        let c5 = Character::new(
            Kinematic {
                position: Vector(440.0, 400.0),
                orientation: east,
                ..Kinematic::default()
            },
            vec![],
        );

        State {
            characters: vec![c1, c2, c3, c4, c5],
        }
    }
    fn get_steerings(self: &State, char: &mut Character) -> Vec<Steering> {
      char.calculate_steerings(self)
    }
    pub fn dispatch(self: &mut State, action: Action) -> () {
        match action {
            Action::Tick(tick) => {
                for char in self.characters.iter_mut() {
                  let steerings = self.get_steerings(char);

                  if let Some(steering) = steerings.first() {
                    char.kinematic.update(steering, tick);
                  }
                }

            }
        }
    }
}
