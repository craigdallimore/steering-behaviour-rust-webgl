use crate::steering::align::Align;
use crate::steering::arrive::Arrive;
use crate::steering::collision_avoidance::CollisionAvoidance;
use crate::steering::evade::Evade;
use crate::steering::face::Face;
use crate::steering::flee::Flee;
use crate::steering::follow_path_chase_rabbit::FollowPathChaseRabbit;
use crate::steering::follow_path_predict::FollowPathChasePredict;
use crate::steering::look_where_you_are_going::LookWhereYouAreGoing;
use crate::steering::match_velocity::MatchVelocity;
use crate::steering::obstacle_avoidance::ObstacleAvoidance;
use crate::steering::pursue::Pursue;
use crate::steering::seek::Seek;
use crate::steering::separation::Separation;
use crate::steering::wander::Wander;
use crate::vector::Vector;

pub struct Steering {
  pub linear: Vector,
  pub angular: f32
}

pub enum Behaviour {
  Align(Align),
  Arrive(Arrive),
  CollisionAvoidance(CollisionAvoidance),
  Evade(Evade),
  Face(Face),
  Flee(Flee),
  FollowPathChaseRabbit(FollowPathChaseRabbit),
  FollowPathPredict(FollowPathChasePredict),
  LookWhereYouAreGoing(LookWhereYouAreGoing),
  MatchVelocity(MatchVelocity),
  ObstacleAvoidance(ObstacleAvoidance),
  Pursue(Pursue),
  Seek(Seek),
  Separation(Separation),
  Wander(Wander),
}
