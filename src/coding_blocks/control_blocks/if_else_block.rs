use serde::{Deserialize, Serialize};

use crate::coding_blocks::conditons::ConditionEnum;

use super::{Condition, *};
#[derive(Serialize, Deserialize)]
pub struct IfElseBlock {
    condition: ConditionEnum,
    ifblocks: Blocks,
    elseblocks: Blocks,
}
impl Block for IfElseBlock {
    fn preform(&self, state: &mut State) -> Result<(), BlockExecutionError> {
        if self.condition.evaluate(state) {
            self.ifblocks.execute(state)
        } else {
            self.elseblocks.execute(state)
        }
    }
}
impl IfElseBlock {
    fn new(ifblocks: Blocks, elseblocks: Blocks, condition: ConditionEnum) -> Self {
        Self {
            condition,
            elseblocks,
            ifblocks,
        }
    }
}
#[cfg(test)]
mod if_else_block_tests {
    use super::*;
    use crate::coding_blocks::{
        state::State, variable_blocks::AssignStateBlock, BlockExecutionError, Value,
    };

    #[test]
    fn test_if_else_block_true_condition() {
        let mut state = State::new();
        let condition = true;
        let if_assign_block =
            AssignStateBlock::new("if_executed".to_string(), Value::String("if".to_string()));
        let else_assign_block =
            AssignStateBlock::new("if_executed".to_string(), Value::String("else".to_string()));
        let if_else_block = IfElseBlock::new(
            Blocks(vec![if_assign_block.into()]),
            Blocks(vec![else_assign_block.into()]),
            condition.into(),
        );

        if_else_block.execute(&mut state).unwrap();
        assert_eq!(
            state.get("if_executed").unwrap(),
            &Value::String("if".to_string())
        );
    }

    #[test]
    fn test_if_else_block_false_condition() {
        let mut state = State::new();
        let condition = false;
        let if_assign_block =
            AssignStateBlock::new("if_executed".to_string(), Value::String("if".to_string()));
        let else_assign_block =
            AssignStateBlock::new("if_executed".to_string(), Value::String("else".to_string()));
        let if_else_block = IfElseBlock::new(
            Blocks(vec![if_assign_block.into()]),
            Blocks(vec![else_assign_block.into()]),
            condition.into(),
        );

        if_else_block.execute(&mut state).unwrap();
        assert_eq!(
            state.get("if_executed").unwrap(),
            &Value::String("else".to_string())
        );
    }
}
