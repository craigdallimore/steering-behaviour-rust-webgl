use crate::domain::character::Character;
use crate::domain::kinematic::Kinematic;
use crate::domain::steering::Behaviour;
use crate::steering::align::Align;
use crate::steering::arrive::Arrive;
use crate::steering::face::Face;
use crate::steering::flee::Flee;
use crate::steering::look_where_you_are_going::LookWhereYouAreGoing;
use crate::steering::match_velocity::MatchVelocity;
use crate::steering::pursue::Pursue;
use crate::steering::seek::Seek;
use crate::vector::Vector;

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

        let nw = Vector(-1.0, -1.0);
        let ne = Vector(1.0, -1.0);
        let sw = Vector(-1.0, 1.0);
        let se = Vector(1.0, 1.0);

        State {
            characters: vec![
                Character::new(
                    Kinematic {
                        position: Vector(150.0, 200.0),
                        velocity: se,
                        orientation: west,
                        ..Kinematic::default()
                    },
                    vec![Behaviour::Pursue(Pursue::new())],
                ),
                Character::new(
                    Kinematic {
                        position: Vector(400.0, 360.0),
                        orientation: north,
                        ..Kinematic::default()
                    },
                    vec![],
                ),
                Character::new(
                    Kinematic {
                        position: Vector(400.0, 440.0),
                        orientation: south,
                        ..Kinematic::default()
                    },
                    vec![],
                ),
                Character::new(
                    Kinematic {
                        position: Vector(360.0, 400.0),
                        orientation: west,
                        ..Kinematic::default()
                    },
                    vec![],
                ),
                Character::new(
                    Kinematic {
                        position: Vector(440.0, 400.0),
                        orientation: east,
                        ..Kinematic::default()
                    },
                    vec![],
                ),
            ],
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
