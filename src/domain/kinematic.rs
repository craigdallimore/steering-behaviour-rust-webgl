use crate::vector::Vector;
use super::steering::Steering;

#[derive(Clone, Copy)]
pub struct Kinematic {
    pub max_acceleration: f32,
    pub max_angular_acceleration: f32,
    pub max_speed: f32,
    pub position: Vector,
    pub orientation: f32,
    pub velocity: Vector,
    pub rotation: f32,
}

fn limit_orientation(o: f32) -> f32 {
  let pi = std::f32::consts::PI;
  if o > 2.0 * pi {
    return -2.0 * pi;
  }
  if o < -2.0 * pi {
    return 2.0 * pi;
  }
  o
}


impl Kinematic {
  pub fn update(&mut self, steering: Steering, time: f32) -> () {

    let next_position:Vector = self.position + (self.velocity * time);

    // The velocity is increased by a difference of the maximum acceleration
    // multiplied by time.
    let next_velocity = self.velocity + (steering.linear * time);

    let next_velocity = if Vector::length(&next_velocity) >= self.max_speed {
      Vector::normalise(&next_velocity) * self.max_speed
    } else {
      next_velocity
    };

    // time is going to be a decimal between 0 and 1 (probably).
    // Here we multiply the rotation speed by time (giving a time-proportional
    // value) and add it to the current orientation.
    let next_orientation = limit_orientation(self.orientation + (self.rotation * time));
    let next_rotation = steering.angular * time;

    self.position = next_position;
    self.velocity = next_velocity;
    self.orientation = next_orientation;
    self.rotation = next_rotation;
  }
}
