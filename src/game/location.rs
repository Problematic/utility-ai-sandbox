#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Location {
  Lounge,
  Quarters,
  Cafeteria,
  Work,
}

impl Location {
  fn travel_value(self) -> f32 {
    match self {
      Self::Lounge => 1.0,
      Self::Quarters => 2.0,
      Self::Cafeteria => 3.0,
      Self::Work => 5.0,
    }
  }

  /// maximum distance between any two locations
  pub fn max_distance() -> f32 {
    Self::Work.travel_time(Self::Lounge)
  }

  #[allow(clippy::cast_precision_loss)]
  pub fn travel_time(self, to: Self) -> f32 {
    let start = self.travel_value();
    let end = to.travel_value();

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
