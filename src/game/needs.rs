use crate::utils;
use std::ops;

#[derive(Debug)]
pub struct Attribute {
  pub current: f32,
  min: f32,
  max: f32,
}

impl Default for Attribute {
  fn default() -> Self {
    Self {
      current: 100.0,
      min: 0.0,
      max: 100.0,
    }
  }
}

impl ops::SubAssign<f32> for Attribute {
  fn sub_assign(&mut self, amount: f32) {
    self.current = utils::clamp(self.current - amount, self.min, self.max);
  }
}

impl ops::AddAssign<f32> for Attribute {
  fn add_assign(&mut self, amount: f32) {
    self.current = utils::clamp(self.current + amount, self.min, self.max);
  }
}

use std::fmt;
impl fmt::Display for Attribute {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.current)
  }
}

#[derive(Debug, Default)]
pub struct Needs {
  pub social: Attribute,
  pub hunger: Attribute,
  pub energy: Attribute,
  pub labor: Attribute,
}

impl Needs {
  pub fn decay(&mut self, amount: f32) {
    self.social -= amount;
    self.hunger -= amount;
    self.energy -= amount;
    self.labor -= amount;
  }
}
