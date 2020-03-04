use super::Decision;

pub struct DMEntry<TContext, TParamKey = &'static str>(pub Decision<TContext, TParamKey>, pub f32);

#[allow(clippy::module_name_repetitions)]
pub struct DecisionMaker<TContext, TParamKey = &'static str> {
  pub name: &'static str,
  //pub score_strategy: ScoreStrategy,
  pub decisions: Vec<DMEntry<TContext, TParamKey>>,
}

impl<TContext, TParamKey> DecisionMaker<TContext, TParamKey> {
  pub fn evaluate(&self, context: &TContext) -> Option<&Decision<TContext, TParamKey>> {
    let mut threshold = 0.0;
    let mut res = None;

    for DMEntry(decision, bonus) in &self.decisions {
      if *bonus < threshold {
        continue;
      }
      let score = decision.score(context, *bonus, threshold) * *bonus;

      if score > threshold {
        threshold = score;

        res = Some(decision);
      }
    }

    res
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Default)]
  struct TestContext;

  #[test]
  fn test_empty_evaluation() {
    let dm = DecisionMaker::<TestContext> {
      name: "test",
      decisions: vec![],
    };

    assert!(dm.evaluate(&TestContext::default()).is_none());
  }
}
