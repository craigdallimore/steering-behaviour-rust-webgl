use crate::vector::Vector;

#[derive(Clone, Copy)]
pub struct Kinematic {
    pub position: Vector,
    pub orientation: f64,
    pub velocity: Vector,
    pub rotation: f64,
}