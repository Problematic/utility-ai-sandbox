use serde::{Deserialize, Serialize};

pub mod consideration;
pub mod decision;
pub mod decision_maker;
pub mod response_curve;
pub mod traits;

pub use self::consideration::Consideration;
pub use self::decision::Decision;
pub use self::decision_maker::DecisionMaker;
pub use self::response_curve::ResponseCurve;
pub use self::traits::Input;

#[derive(Debug, Serialize, Deserialize)]
pub struct Weighted<T> {
  pub item: T,
  pub weight: f32,
}

impl<T> std::ops::Deref for Weighted<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.item
  }
}
