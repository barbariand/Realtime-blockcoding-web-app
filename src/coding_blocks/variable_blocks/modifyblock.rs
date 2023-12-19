use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::coding_blocks::{
    error::BlockExecutionError,
    state::{State, Value},
    Block, GetValue, ValueEnum,
};
#[derive(Serialize, Deserialize)]
pub struct AssignStateBlock {
    key: String,
    value: ValueEnum,
}
impl AssignStateBlock {
    pub fn new<V: Into<ValueEnum>>(key: String, value: V) -> Self {
        Self {
            key,
            value: value.into(),
        }
    }
}

impl Block for AssignStateBlock {
    fn preform(&self, state: &mut State) -> Result<(), BlockExecutionError> {
        let value = self.value.get_value(state);
        state.insert(self.key.clone(), value);
        Ok(())
    }
}
#[cfg(test)]
mod AsignBlockTesting {
    use crate::coding_blocks::ExecuteBlock;

    use super::*;
    #[test]
    fn testing_asign() {
        let mut state = State::new();
        AssignStateBlock::new("key".to_string(), Value::String("value".to_string()))
            .execute(&mut state)
            .unwrap();
        assert!(match state.get("key").expect("we put it in there") {
            Value::String(s) => s == "value",
            Value::Number(_) => false,
        })
    }
}
#[derive(Serialize, Deserialize)]
pub struct GetValueBlock {
    key: String,
}
impl GetValue for GetValueBlock {
    fn get_value(&self, state: &State) -> Value {
        state
            .get(&self.key)
            .unwrap_or(&Value::String(String::new()))
            .get_value(state)
    }
}
#[cfg(test)]
mod GetValueTesting {
    use super::*;
    use crate::coding_blocks::{Blocks, ExecuteBlock};
    #[test]
    fn test_getting_value() {
        let mut state = State::new();
    }
}
