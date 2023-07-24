use crate::steering::{
    align::Align,
    arrive::Arrive,
    collision_avoidance::CollisionAvoidance,
    evade::Evade,
    face::Face,
    flee::Flee,
    follow_path_chase_rabbit::FollowPathChaseRabbit,
    follow_path_predict::FollowPathChasePredict,
    look_where_you_are_going::LookWhereYouAreGoing,
    match_velocity::MatchVelocity,
    obstacle_avoidance::ObstacleAvoidance,
    pursue::Pursue,
    seek::Seek,
    separation::Separation,
    wander::Wander,
};
use crate::vector::Vector;

pub struct Steering {
    pub linear: Vector,
    pub angular: f32,
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
