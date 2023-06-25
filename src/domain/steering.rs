use crate::vector::Vector;
use super::kinematic::Kinematic;

pub struct Steering {
  pub linear: Vector,
  pub angular: f32
}

pub trait Behaviour {
  type Args;
  fn calculate(self: &Self, kinematic: Kinematic, args: Self::Args) -> Steering;
  fn new() -> Self;
}
