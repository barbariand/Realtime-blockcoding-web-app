use super::{Condition, *};
pub struct IfElseBlock<T: Condition> {
    condition: T,
    ifblocks: Blocks,
    elseblocks: Blocks,
}
impl<T: Condition> Block for IfElseBlock<T> {
    fn preform(&self, state: &mut State) -> Result<(), BlockExecutionError> {
        if self.condition.evaluate(state) {
            self.ifblocks.execute(state)
        } else {
            self.elseblocks.execute(state)
        }
    }
}
impl<T: Condition> IfElseBlock<T> {
    fn new(ifblocks: Blocks, elseblocks: Blocks, condition: T) -> Self {
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
            Blocks(vec![Box::new(if_assign_block)]),
            Blocks(vec![Box::new(else_assign_block)]),
            condition,
        );

        if_else_block.preform(&mut state).unwrap();
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
            Blocks(vec![Box::new(if_assign_block)]),
            Blocks(vec![Box::new(else_assign_block)]),
            condition,
        );

        if_else_block.preform(&mut state).unwrap();
        assert_eq!(
            state.get("if_executed").unwrap(),
            &Value::String("else".to_string())
        );
    }
}
