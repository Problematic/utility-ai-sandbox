#![warn(clippy::pedantic)]

mod game;
mod utils;

use game::*;

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
  pub current_agent: Agent,
  pub targets: Vec<Agent>,
  // pub urgent_events: ...,
  // pub blackboard: Blackboard<TKey>,
}

pub struct Consideration<TContext> {
  pub name: &'static str,
  // pub response_curve: ResponseCurve,
  pub score: Box<dyn Fn(&TContext) -> f32>,
}

pub struct Decision<TContext> {
  pub name: &'static str, // "Eat a Meal"
  pub considerations: Vec<Consideration<TContext>>,
}

impl<TContext> Decision<TContext> {
  #[must_use]
  #[allow(clippy::cast_precision_loss)]
  pub fn score(&self, context: &TContext) -> f32 {
    let mut score = 1.0;
    for consideration in &self.considerations {
      score *= (consideration.score)(context);
    }

    let mod_factor = 1.0 - (1.0 / self.considerations.len() as f32);
    let make_up_value = (1.0 - score) * mod_factor;

    score + (make_up_value * score)
  }
}

// pub struct DecisionActionPair<T>(Decision, T);

// type Action = Vec<State>
// type DecisionMaker = (Decision, Action)
// Vec<DecisionMaker>

fn main() {
  pretty_env_logger::init();

  let mut agent = Agent::new(String::from("Crash Test Dummy"));
  agent.tick(50.0);

  let context = DecisionContext {
    current_agent: agent,
    targets: vec![],
  };

  let eat_a_meal = Decision::<DecisionContext> {
    name: "Eat a Meal",
    considerations: vec![
      Consideration {
        name: "Am I hungry?",
        score: Box::new(|ctx| {
          let hunger = &ctx.current_agent.needs.hunger;

          hunger.current / hunger.max()
        }),
      },
      Consideration {
        name: "Am I near the cafeteria?",
        score: Box::new(|ctx| {
          use std::f32::EPSILON;

          let dist_to_cafeteria = &ctx.current_agent.location.travel_time(Location::Cafeteria);

          if dist_to_cafeteria < &EPSILON {
            1.0
          } else {
            1.0 - (dist_to_cafeteria / Location::max_distance())
          }
        }),
      },
    ],
  };

  println!("Eat a Meal: {}", eat_a_meal.score(&context));
}
