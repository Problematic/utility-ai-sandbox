use super::decision::Decision;
use super::traits::Score;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DecisionMaker<TInput> {
  decisions: Vec<Decision<TInput>>,
}

impl<'a, TInput> DecisionMaker<TInput>
where
  TInput: Score<'a>,
{
  #[must_use]
  pub fn evaluate(&self, context: &TInput::Context) -> (Option<&Decision<TInput>>, f32) {
    let mut threshold = 0.0;
    let mut selected = None;

    for decision in &self.decisions {
      if decision.weight() <= threshold {
        continue;
      }

      let score = decision.weight() * decision.score(context);
      log::debug!("{}: {}", decision.name, score);

      if score > threshold {
        threshold = score;
        selected = Some(decision);
      }
    }

    (selected, threshold)
  }
}
