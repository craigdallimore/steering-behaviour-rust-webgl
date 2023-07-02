use crate::vector::Vector;

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

    let target_orientation = 0.0; // east
    let target_position = Vector(700.0, 100.0);
    let mut target: Kinematic = Kinematic::default();
    target.velocity = Vector(0.0, 10.0);
    target.position =  Vector(600.0, 600.0);

    for (_i, behaviour) in self.behaviours.iter_mut().enumerate() {
      match behaviour {
        Behaviour::Align(behaviour) => {
          let steering = behaviour.calculate(self.kinematic, target_orientation);
          self.kinematic.update(steering, tick);
        }
        Behaviour::Arrive(behaviour) => {
          let steering = behaviour.calculate(self.kinematic);
          self.kinematic.update(steering, tick);
        }
        Behaviour::Pursue(behaviour) => {
          let steering = behaviour.calculate(self.kinematic);
          self.kinematic.update(steering, tick);
        }
        Behaviour::CollisionAvoidance(behaviour) => {
          let steering = behaviour.calculate(self.kinematic);
          self.kinematic.update(steering, tick);
        }
        Behaviour::Evade(behaviour) => {
          let steering = behaviour.calculate(self.kinematic);
          self.kinematic.update(steering, tick);
        }
        Behaviour::Face(behaviour) => {
          let steering = behaviour.calculate(self.kinematic, target_position);
          self.kinematic.update(steering, tick);
        }
        Behaviour::Flee(behaviour) => {
          let steering = behaviour.calculate(self.kinematic, target_position);
          self.kinematic.update(steering, tick);
        }
        Behaviour::FollowPathChaseRabbit(behaviour) => {
          let steering = behaviour.calculate(self.kinematic);
          self.kinematic.update(steering, tick);
        }
        Behaviour::FollowPathPredict(behaviour) => {
          let steering = behaviour.calculate(self.kinematic);
          self.kinematic.update(steering, tick);
        }
        Behaviour::LookWhereYouAreGoing(behaviour) => {
          let steering = behaviour.calculate(self.kinematic);
          self.kinematic.update(steering, tick);
        }
        Behaviour::MatchVelocity(behaviour) => {
          let steering = behaviour.calculate(self.kinematic, target);
          self.kinematic.update(steering, tick);
        }
        Behaviour::ObstacleAvoidance(behaviour) => {
          let steering = behaviour.calculate(self.kinematic);
          self.kinematic.update(steering, tick);
        }
        Behaviour::Seek(behaviour) => {
          let steering = behaviour.calculate(self.kinematic, target_position);
          self.kinematic.update(steering, tick);
        }
        Behaviour::Separation(behaviour) => {
          let steering = behaviour.calculate(self.kinematic);
          self.kinematic.update(steering, tick);
        }
        Behaviour::Wander(behaviour) => {
          let steering = behaviour.calculate(self.kinematic);
          self.kinematic.update(steering, tick);
        }
      }
    }
  }
}
