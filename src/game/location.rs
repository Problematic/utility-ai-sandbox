#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Location {
  Lounge = 0,
  Quarters = 1,
  Cafeteria = 2,
  Work = 4,
}

impl Location {
  #[allow(clippy::cast_precision_loss)]
  pub fn travel_time(self, to: Self) -> f32 {
    let start = self as i32;
    let end = to as i32;

    (start - end).abs() as f32
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::f32::EPSILON;

  #[test]
  fn test_travel_time() {
    assert!(Location::Lounge.travel_time(Location::Work) - 4.0 < EPSILON);
    assert!(Location::Work.travel_time(Location::Lounge) - 4.0 < EPSILON);

    assert!(Location::Quarters.travel_time(Location::Cafeteria) - 1.0 < EPSILON);

    assert!(Location::Quarters.travel_time(Location::Quarters) < EPSILON);
  }
}
