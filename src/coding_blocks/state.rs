use std::{
    cmp::Ordering,
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};

use super::GetValue;

pub struct State {
    variables: HashMap<String, Value>,
}
impl State {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
}
impl Deref for State {
    type Target = HashMap<String, Value>;
    fn deref(&self) -> &Self::Target {
        &self.variables
    }
}
impl DerefMut for State {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.variables
    }
}
#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
pub enum Value {
    String(String),
    Number(f64),
}
impl GetValue for Value {
    fn get_value(&self, _: &State) -> Value {
        self.clone()
    }
}
impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match (self, other) {
            (Value::String(v1), Value::String(v2)) => v1.cmp(v2),
            (Value::String(_v1), Value::Number(_v2)) => Ordering::Greater,
            (Value::Number(_v1), Value::String(_v2)) => Ordering::Less,
            (Value::Number(v1), Value::Number(v2)) => v1.partial_cmp(v2)?,
        })
    }
}
impl From<String> for Value {
    fn from(val: String) -> Self {
        Value::String(val)
    }
}
impl From<f64> for Value {
    fn from(val: f64) -> Self {
        Value::Number(val)
    }
}
