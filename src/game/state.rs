#![allow(dead_code)]

use super::Location;
use std::time::Duration;

pub enum State<TAction> {
  Idle(Option<Duration>),
  Goto(Location),
  Perform(TAction),
}
