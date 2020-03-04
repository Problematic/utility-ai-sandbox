#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum InputParam {
  String(String),
  Float(f32),
  Bool(bool),
}
