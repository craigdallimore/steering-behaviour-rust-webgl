pub fn limit_orientation(o: f32) -> f32 {
  // return a value bounded between -2pi and 2pi
  let pi = std::f32::consts::PI;
  if o > 2.0 * pi {
    return -2.0 * pi;
  }
  if o < -2.0 * pi {
    return 2.0 * pi;
  }
  o
}

pub fn map_to_range(orientation: f32) -> f32 {
  // return a value bounded between -2pi and 2pi, where values utside the range map to equivalent
  // bearigs within the range

  let pi = std::f32::consts::PI;
  // To rotate all the way clockwise, use the value 6.283
  // 6.3 is roughtly NE 0.01 I expect.
  let next_orientation = if orientation.abs() > pi {
    orientation - pi * 2.0 * orientation.signum()
  } else {
    orientation
  };

  next_orientation % (pi * 2.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_limit_orientation() {

        let pi = std::f32::consts::PI;

        let left = limit_orientation(0.0);
        let right = 0.0;
        assert_eq!(left, right);

        let left = limit_orientation(2.0 * pi);
        let right = 2.0 * pi;
        assert_eq!(left, right);

        let left = limit_orientation(3.0 * pi);
        let right = -2.0 * pi;
        assert_eq!(left, right);

        let left = limit_orientation(-3.0 * pi);
        let right = 2.0 * pi;
        assert_eq!(left, right);

    }

    #[test]
    fn test_map_to_range() {

        let pi = std::f32::consts::PI;

        let left = map_to_range(0.0);
        let right = 0.0;
        assert_eq!(left, right);

        let left = map_to_range(2.0 * pi);
        let right = 0.0;
        assert_eq!(left, right);

        let left = map_to_range(3.0 * pi);
        let right = pi;

        // approximately pi; allowing for floating point inaccuracy
        assert!(left + 0.05 > right && left < right);

        let left = map_to_range(-3.0 * pi);
        let right = -pi;
        // approximately pi; allowing for floating point inaccuracy
        assert!(left > right && left < right + 0.05);

    }

}
