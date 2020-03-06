#![warn(clippy::pedantic)]

mod game;
mod utility_ai;
mod utils;

use crate::game::*;
use crate::utility_ai::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum MyGameAction {
  Eat,
  Sleep,
  Work,
  Socialize,
}

impl MyGameAction {
  pub fn execute(self, agent: &mut Agent, target: Option<&Agent>) {
    match self {
      Self::Eat => {
        agent.eat(750.0);
      }
      Self::Sleep => {
        agent.sleep(8.0);
      }
      Self::Work => {
        agent.work(8.0);
      }
      Self::Socialize => {
        if let Some(target) = target {
          agent.talk_to(target);
        } else {
          log::warn!("{} is talking to themselves again.", agent.name);
        }
      }
    }
  }
}

#[derive(Debug)]
pub struct MyGameContext<'a> {
  pub agent: &'a Agent,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AgentNeed {
  Hunger,
  Social,
  Labor,
  Energy,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MyGameInput {
  NeedLevel(AgentNeed),
  DistanceTo { location: Location, max_range: f32 },
}

impl<'a> Input<'a> for MyGameInput {
  type Context = MyGameContext<'a>;

  fn score(&self, context: &Self::Context) -> f32 {
    match self {
      Self::NeedLevel(need) => match need {
        AgentNeed::Hunger => {
          let need = &context.agent.needs.hunger;
          1.0 - (need.current / need.max())
        }
        AgentNeed::Social => {
          let need = &context.agent.needs.social;
          1.0 - (need.current / need.max())
        }
        AgentNeed::Energy => {
          let need = &context.agent.needs.energy;
          1.0 - (need.current / need.max())
        }
        AgentNeed::Labor => {
          let need = &context.agent.needs.labor;
          1.0 - (need.current / need.max())
        }
      },
      Self::DistanceTo {
        location,
        max_range,
      } => {
        let dist = location.travel_time(context.agent.location);
        dist / max_range
      }
    }
  }
}

fn main() {
  pretty_env_logger::init();

  let raw = include_str!("../resources/test_data.json");
  let decision_maker =
    serde_json::from_str::<DecisionMaker<MyGameInput, MyGameAction>>(raw).unwrap();

  // dbg!(&decision_maker);

  let mut agent = Agent::new(String::from("Crash Test Dummy"));
  agent.tick(80.0);

  // dbg!(&agent);

  let context = &MyGameContext { agent: &agent };

  let (decision, score) = decision_maker.evaluate(context);

  if let Some(decision) = decision {
    log::debug!("Selected '{}' with a score of {}", decision.name, score);

    decision.action().execute(&mut agent, None);
  }
}
