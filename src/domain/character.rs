use crate::vector::Vector;
use uuid::Uuid;


use super::{kinematic::Kinematic, steering::{Behaviour, Steering}, initial_state::State};

pub struct Character {
    pub id: Uuid,
    pub behaviours: Vec<Behaviour>,
    pub kinematic: Kinematic,
    pub label: Option<String>,
}

impl Character {
    pub fn new(kinematic: Kinematic, behaviours: Vec<Behaviour>) -> Character {
        Character {
            id: Uuid::new_v4(),
            behaviours,
            kinematic,
            label: None,
        }
    }
    pub fn calculate_steerings(self: &Character, state: &State) -> Vec<Steering> {

        // Development data ---------------------------------------------------

        let pi = std::f32::consts::PI;

     // let rad_east = 0.0;
     // let rad_ne = pi * 0.25;
     // let rad_north = pi * 0.5;
     // let rad_nw = pi * 0.75;
     // let rad_west = pi;
     // let rad_sw = pi * 1.25;
     // let rad_south = pi * 1.5;
        let rad_se = pi * 1.75;

        let target_orientation = rad_se;
        let target_position = Vector(400.0, 400.0);

        let mut target: Kinematic = Kinematic::default();
        target.velocity = Vector(0.0, 10.0);
        target.position = target_position;

        // --------------------------------------------------------------------

        self.behaviours.iter().fold(vec![], |mut acc, behaviour| {
            match behaviour {
                Behaviour::Align(behaviour) => {
                    let steering = behaviour.calculate(self.kinematic, target_orientation);
                    acc.push(steering);
                }
                Behaviour::Arrive(behaviour) => {
                    if let Some(steering) = behaviour.calculate(self.kinematic, target.position) {
                      acc.push(steering);
                    }
                }
                Behaviour::Pursue(behaviour) => {
                    let steering = behaviour.calculate(self.kinematic, target);
                    acc.push(steering);
                }
                Behaviour::CollisionAvoidance(behaviour) => {
                    let steering = behaviour.calculate(self.kinematic);
                    acc.push(steering);
                }
                Behaviour::Evade(behaviour) => {
                    let steering = behaviour.calculate(self.kinematic, target);
                    acc.push(steering);
                }
                Behaviour::Face(behaviour) => {
                    let steering = behaviour.calculate(self.kinematic, target_position);
                    acc.push(steering);
                }
                Behaviour::Flee(behaviour) => {
                    let steering = behaviour.calculate(self.kinematic, target_position);
                    acc.push(steering);
                }
                Behaviour::FollowPathChaseRabbit(behaviour) => {
                    let steering = behaviour.calculate(self.kinematic);
                    acc.push(steering);
                }
                Behaviour::FollowPathPredict(behaviour) => {
                    let steering = behaviour.calculate(self.kinematic);
                    acc.push(steering);
                }
                Behaviour::LookWhereYouAreGoing(behaviour) => {
                    let steering = behaviour.calculate(self.kinematic);
                    acc.push(steering);
                }
                Behaviour::MatchVelocity(behaviour) => {
                    let steering = behaviour.calculate(self.kinematic, target);
                    acc.push(steering);
                }
                Behaviour::ObstacleAvoidance(behaviour) => {
                    let steering = behaviour.calculate(self.kinematic);
                    acc.push(steering);
                }
                Behaviour::Seek(behaviour) => {
                    let steering = behaviour.calculate(self.kinematic, target_position);
                    acc.push(steering);
                }
                Behaviour::Separation(behaviour) => {
                    let others: Vec<&Kinematic> = state.characters.iter()
                      .filter(|a| a.id != self.id)
                      .map(|c| &c.kinematic)
                      .collect();

                    let steering = behaviour.calculate(self.kinematic, others);
                    acc.push(steering);
                }
                Behaviour::Wander(_behaviour) => {
                    /*
                    let steering = behaviour.calculate(self.kinematic);
                    acc.push(steering);
                    */
                }
            }
            acc
        })
    }
}
