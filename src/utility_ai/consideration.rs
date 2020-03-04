use super::{InputParam, ResponseCurve};
use std::collections::HashMap;

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
