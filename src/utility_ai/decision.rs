use super::Consideration;

pub struct Decision<TContext, TParamKey = &'static str> {
  pub name: &'static str, // "Eat a Meal"
  pub targetable: bool,
  pub considerations: Vec<Consideration<TContext, TParamKey>>,
}

impl<TContext, TParamKey> Decision<TContext, TParamKey> {
  #[must_use]
  #[allow(clippy::cast_precision_loss)]
  pub fn score(&self, context: &TContext) -> f32 {
    let mut score = 1.0;
    for consideration in &self.considerations {
      let val = consideration
        .response_curve
        .evaluate((consideration.input.score)(context, &consideration.params));

      log::debug!("{}: {}", consideration.name, val);

      score *= val;
    }

    let mod_factor = 1.0 - (1.0 / self.considerations.len() as f32);
    let make_up_value = (1.0 - score) * mod_factor;

    score + (make_up_value * score)
  }
}
