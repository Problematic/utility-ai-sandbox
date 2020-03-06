pub trait Score<'a>
where
  Self::Context: 'a,
{
  type Context;

  #[must_use]
  fn score(&self, context: &Self::Context) -> f32;
}
