use super::location::Location;
use super::needs::Needs;

#[derive(Debug)]
pub struct Agent {
  pub name: String,
  pub location: Location,
  pub needs: Needs,
}

impl Agent {
  #[must_use]
  pub fn new(name: String) -> Self {
    Self {
      name,
      location: Location::Quarters,
      needs: Needs::default(),
    }
  }

  /// 750 kcals is a full meal, and will restore hunger to full
  pub fn eat(&mut self, kcals: f32) {
    let res = (kcals / 750.0) * 100.0;
    self.needs.hunger += res;

    log::info!(
      "{} eats {} kcals, restoring {} hunger (new total: {})",
      self.name,
      kcals,
      res,
      self.needs.hunger
    );
  }

  /// 8 hours is fully rested, and will restore energy to full
  pub fn sleep(&mut self, duration: f32) {
    let res = duration * 12.5;
    self.needs.energy += res;

    log::info!(
      "{} sleeps for {} hours, restoring {} energy (new total: {})",
      self.name,
      duration,
      res,
      self.needs.energy
    );
  }

  /// Currently any chat will restore social to full
  ///
  /// TODO: adjust by relationship to other agent
  pub fn talk_to(&mut self, other: &Agent) {
    let res = 100.0;
    self.needs.social += res;

    log::info!(
      "{} talks to {}, restoring {} social (new total: {})",
      self.name,
      other.name,
      res,
      self.needs.social
    );
  }

  pub fn work(&mut self, duration: f32) {
    let res = duration * 12.5;
    self.needs.labor += res;

    log::info!(
      "{} works for {} hours, restoring {} labor (new total: {}",
      self.name,
      duration,
      res,
      self.needs.labor
    );
  }
}

impl Agent {
  pub fn tick(&mut self, dt: f32) {
    self.needs.decay(dt);
  }
}

use std::fmt;
impl fmt::Display for Agent {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.name)
  }
}
