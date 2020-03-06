use super::response_curve::ResponseCurve;
use super::traits::Score;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Consideration<TInput> {
  pub name: String,
  input: TInput,
  response_curve: ResponseCurve,
}

impl<'a, TInput> Score<'a> for Consideration<TInput>
where
  TInput: Score<'a>,
{
  type Context = TInput::Context;

  fn score(&self, context: &TInput::Context) -> f32 {
    self.response_curve.evaluate(self.input.score(context))
  }
}
