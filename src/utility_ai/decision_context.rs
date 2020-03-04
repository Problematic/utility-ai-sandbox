pub trait DecisionContext {
  type Agent;

  #[must_use]
  fn agent(&self) -> &Self::Agent;

  #[must_use]
  fn target(&self) -> Option<&Self::Agent>;
}
