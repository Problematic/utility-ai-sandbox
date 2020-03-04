use super::ResponseCurve;
use std::collections::HashMap;

#[allow(dead_code)]
pub enum InputParam {
  String(String),
  Float(f32),
  Bool(bool),
}

pub struct Consideration<TContext, TParamKey = &'static str> {
  pub name: &'static str,
  pub input: Input<TContext, TParamKey>,
  pub params: HashMap<TParamKey, InputParam>,
  pub response_curve: ResponseCurve,
}

pub struct Input<TContext, TParamKey = &'static str> {
  pub name: &'static str,
  pub score: Box<dyn Fn(&TContext, &HashMap<TParamKey, InputParam>) -> f32>,
}
