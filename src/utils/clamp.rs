pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
  assert!(min < max);

  if x.is_nan() || x.is_infinite() {
    0.0
  } else if x < min {
    min
  } else if x > max {
    max
  } else {
    x
  }
}
