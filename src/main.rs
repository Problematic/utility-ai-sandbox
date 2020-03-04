#![warn(clippy::pedantic)]

mod game;
mod utility_ai;
mod utils;

use game::*;
use utility_ai::*;

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

pub struct DecisionContext {
  pub agent: Agent,
  pub targets: Vec<Agent>,
  // pub urgent_events: ...,
  // pub blackboard: Blackboard<TKey>,
}

impl DecisionContext {
  #[must_use]
  pub fn agent(&self) -> &Agent {
    &self.agent
  }
}

// pub struct DecisionActionPair<T>(Decision, T);
// type Action = Vec<State>
// type DecisionMaker = (Decision, Action)
// Vec<DecisionMaker>

fn main() {
  pretty_env_logger::init();
}

#[cfg(test)]
mod tests {
  use super::*;
  use maplit::hashmap;
  use std::collections::HashMap;

  #[test]
  fn test_scoring() {
    let consideration_hunger_level = Input::<DecisionContext> {
      name: "Hunger Level (self)",
      score: Box::new(|ctx, _params| {
        let hunger = &ctx.agent().needs.hunger;
        hunger.current / hunger.max()
      }),
    };
    let consideration_distance_to_cafeteria = Input::<DecisionContext> {
      name: "Distance to cafeteria (self)",
      score: Box::new(|ctx, params| {
        if let Some(InputParam::Float(max_range)) = params.get(&"max_range") {
          let dist_to_cafeteria = &ctx.agent().location.travel_time(Location::Cafeteria);
          dist_to_cafeteria / max_range
        } else {
          0.0
        }
      }),
    };
    let mut agent = Agent::new(String::from("Crash Test Dummy"));
    agent.tick(50.0);
    let context = DecisionContext {
      agent,
      targets: vec![],
    };
    let eat_a_meal = Decision::<DecisionContext> {
      name: "Eat a Meal",
      targetable: false,
      considerations: vec![
        Consideration {
          name: "Am I hungry?",
          input: consideration_hunger_level,
          params: HashMap::default(),
          response_curve: ResponseCurve::Linear {
            slope: -1.0,
            x_shift: 0.0,
            y_shift: 1.0,
          },
        },
        Consideration {
          name: "Am I near the cafeteria?",
          input: consideration_distance_to_cafeteria,
          params: hashmap! {
            "max_range" => InputParam::Float(4.0)
          },
          response_curve: ResponseCurve::Linear {
            slope: -1.0,
            x_shift: 0.0,
            y_shift: 1.0,
          },
        },
      ],
    };

    assert!((eat_a_meal.score(&context) - 0.492_187_5).abs() < std::f32::EPSILON);
  }
}
