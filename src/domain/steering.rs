use crate::steering::face::Face;
use crate::steering::seek::Seek;
use crate::vector::Vector;
use crate::steering::align::Align;

pub struct Steering {
  pub linear: Vector,
  pub angular: f32
}

pub enum Behaviour {
  Align(Align),
  Seek(Seek),
  Face(Face)
}
