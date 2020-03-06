use super::consideration::Consideration;
use super::traits::Score;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Decision<TInput, TAction>
where
  TAction: Copy,
{
  pub name: String,
  weight: f32,
  considerations: Vec<Consideration<TInput>>,
  action: TAction,
}

impl<TInput, TAction> Decision<TInput, TAction>
where
  TAction: Copy,
{
  pub fn weight(&self) -> f32 {
    self.weight
  }

  pub fn action(&self) -> TAction {
    self.action
  }
}

impl<'a, TInput, TAction> Score<'a> for Decision<TInput, TAction>
where
  TInput: Score<'a>,
  TAction: Copy,
{
  type Context = TInput::Context;

  #[allow(clippy::cast_precision_loss)]
  fn score(&self, context: &TInput::Context) -> f32 {
    if self.considerations.is_empty() {
      return 0.0;
    }

    let initial = 1.0;
    let mut result = initial;
    for consideration in &self.considerations {
      let score = consideration.score(context);
      log::debug!("{}: {}", consideration.name, score);

      result *= consideration.score(context);
    }

    let mod_factor = 1.0 - (1.0 / self.considerations.len() as f32);
    let make_up_value = (1.0 - result) * mod_factor;

    result + (make_up_value * result)
  }
}
