use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

  pub fn travel_time(self, to: Self) -> f32 {
    let start = self.travel_value();
    let end = to.travel_value();

    (start - end).abs()
  }
}

use std::fmt;
impl fmt::Display for Location {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let s = match self {
      Self::Lounge => "Lounge",
      Self::Quarters => "Quarters",
      Self::Cafeteria => "Cafeteria",
      Self::Work => "Work",
    };

    write!(f, "{}", s)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::f32::EPSILON;

  #[test]
  fn test_travel_time() {
    assert!((Location::Lounge.travel_time(Location::Work) - 4.0).abs() < EPSILON);
    assert!((Location::Work.travel_time(Location::Lounge) - 4.0).abs() < EPSILON);

    assert!((Location::Quarters.travel_time(Location::Cafeteria) - 1.0).abs() < EPSILON);

    assert!(Location::Quarters.travel_time(Location::Quarters).abs() < EPSILON);
  }
}
