use crate::utils;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseCurve {
  Linear,
  InverseLinear,
  CustomLinear {
    slope: f32,
    x_shift: f32,
    y_shift: f32,
  },
  Polynomial {
    slope: f32,
    exponent: f32,
    x_shift: f32,
    y_shift: f32,
  },
  Logistic {
    slope: f32,
    exponent: f32,
    x_shift: f32,
    y_shift: f32,
  },
  Logit {
    slope: f32,
    x_shift: f32,
    y_shift: f32,
  },
  Normal {
    slope: f32,
    exponent: f32,
    x_shift: f32,
    y_shift: f32,
  },
  Sine {
    slope: f32,
    x_shift: f32,
    y_shift: f32,
  },
}

impl ResponseCurve {
  pub fn evaluate(&self, x: f32) -> f32 {
    let x = utils::clamp(x, 0.0, 1.0);

    let y = match self {
      Self::Linear => x,
      Self::InverseLinear => -x + 1.0,
      Self::CustomLinear {
        slope,
        x_shift,
        y_shift,
      } => slope * (x - x_shift) + y_shift,
      Self::Polynomial {
        slope,
        exponent,
        x_shift,
        y_shift,
      } => slope * (x - x_shift).powf(*exponent) + y_shift,
      Self::Logistic {
        slope,
        exponent,
        x_shift,
        y_shift,
      } => (slope / (-10.0 * exponent * (x - 0.5 - x_shift)).exp()) + y_shift,
      Self::Logit {
        slope,
        x_shift,
        y_shift,
      } => slope * ((x - x_shift) / (1.0 - (x - x_shift))).ln() / 5.0 + 0.5 + y_shift,
      Self::Normal {
        slope,
        exponent,
        x_shift,
        y_shift,
      } => slope * (-30.0 * exponent * (x - 0.5 - x_shift) * (x - 0.5 - x_shift)).exp() + y_shift,
      Self::Sine {
        slope,
        x_shift,
        y_shift,
      } => 0.5 * slope * (2.0 * std::f32::consts::PI * (x - x_shift)).sin() + 0.5 + y_shift,
    };

    utils::clamp(y, 0.0, 1.0)
  }
}
