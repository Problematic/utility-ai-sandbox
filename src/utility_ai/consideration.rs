use super::response_curve::ResponseCurve;
use super::traits::Scorable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Consideration<TInput> {
  pub name: String,
  input: TInput,
  response_curve: ResponseCurve,
}

impl<'a, TInput> Scorable<'a> for Consideration<TInput>
where
  TInput: Scorable<'a>,
{
  type Context = TInput::Context;

  fn score(&self, context: &TInput::Context) -> f32 {
    self.response_curve.evaluate(self.input.score(context))
  }
}
