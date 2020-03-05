use super::consideration::Consideration;
use super::traits::Scorable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Decision<TInput> {
  pub name: String,
  considerations: Vec<Consideration<TInput>>,
}

impl<'a, TInput> Scorable<'a> for Decision<TInput>
where
  TInput: Scorable<'a>,
{
  type Context = TInput::Context;

  #[allow(clippy::cast_precision_loss)]
  fn score(&self, context: &TInput::Context) -> f32 {
    if self.considerations.is_empty() {
      return 0.0;
    }

    let initial = 1.0;
    let mut score = initial;
    for consideration in &self.considerations {
      score *= consideration.score(context);
    }

    let mod_factor = 1.0 - (1.0 / self.considerations.len() as f32);
    let make_up_value = (1.0 - score) * mod_factor;

    score + (make_up_value * score)
  }
}
