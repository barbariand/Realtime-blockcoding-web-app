use std::{cmp::Ordering, collections::HashMap, ops::Deref};

pub struct State {
    variables: HashMap<String, Value>,
}
impl Deref for State {
    type Target = HashMap<String, Value>;
    fn deref(&self) -> &Self::Target {
        &self.variables
    }
}
#[derive(PartialEq, Eq, PartialOrd)]
pub enum Value {
    String(String),
    Number(i64),
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Value::String(v1), Value::String(v2)) => v1.cmp(v2),
            (Value::String(_v1), Value::Number(_v2)) => Ordering::Greater,
            (Value::Number(_v1), Value::String(_v2)) => Ordering::Less,
            (Value::Number(v1), Value::Number(v2)) => v1.cmp(v2),
        }
    }
}
impl Into<Value> for String {
    fn into(self) -> Value {
        Value::String(self)
    }
}
impl Into<Value> for i64 {
    fn into(self) -> Value {
        Value::Number(self)
    }
}
