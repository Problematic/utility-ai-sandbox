#![warn(clippy::pedantic)]

mod game;
mod utils;

use game::*;
use std::time::{Duration, Instant};

#[derive(Default, Debug)]
pub struct State {
  pub agents: Vec<Agent>,
}

impl State {
  pub fn tick(&mut self, dt: f32) {
    for agent in &mut self.agents {
      agent.tick(dt);
    }
  }
}

fn main() {
  pretty_env_logger::init();

  let mut agents = Vec::new();

  for i in 1..=5 {
    let agent = Agent::new(format!("Vaultizen #{:03}", i));
    agents.push(agent);
  }

  let mut state = State { agents };

  let mut now = Instant::now();
  loop {
    let dt = now.elapsed().as_secs_f32();
    now = Instant::now();

    log::trace!("dt: {}", dt);

    state.tick(dt);

    log::info!("{:#?}", state);

    std::thread::sleep(Duration::from_secs_f32(1.0 / 5.0));
  }
}
