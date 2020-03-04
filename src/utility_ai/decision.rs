use super::Consideration;

/// `Decision`s are actions linked to a game-specific code function, an
/// abstraction that lets the AI system ignore the implications of an
/// action and just tell an agent "go do [something]".
///
/// - Execute a skill
/// - Perform movement to a location
/// - Call an animation
/// - Run a script action
///
/// Examples:
/// - "Warrior Axe Melee"
/// - "Move to Safe Spot"
/// - "Play Emote [wave]"
pub struct Decision<TContext, TParamKey = &'static str> {
  pub name: &'static str, // "Eat a Meal"
  pub targetable: bool,
  pub considerations: Vec<Consideration<TContext, TParamKey>>,
}

impl<TContext, TParamKey> Decision<TContext, TParamKey> {
  #[must_use]
  #[allow(clippy::cast_precision_loss)]
  pub fn score(&self, context: &TContext, initial: f32, threshold: f32) -> f32 {
    if self.considerations.is_empty() {
      return 0.0;
    }

    let mut score = initial;
    for consideration in &self.considerations {
      if score < threshold {
        break;
      }

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
