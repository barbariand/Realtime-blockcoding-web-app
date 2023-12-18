use serde::{Deserialize, Serialize};

use super::{Condition, *};
#[derive(Serialize, Deserialize)]
pub struct IfBlock<T: Condition> {
    condition: T,
    blocks: Blocks,
}
impl<T: Condition> Block for IfBlock<T> {
    fn preform(&self, state: &mut State) -> Result<(), BlockExecutionError> {
        if self.condition.evaluate(state) {
            return self.blocks.execute(state);
        }
        Ok(())
    }
}
impl<T: Condition> IfBlock<T> {
    fn new(blocks: Blocks, condition: T) -> Self {
        Self { blocks, condition }
    }
}
#[cfg(test)]
mod if_block_tests {
    use super::*;
    use crate::coding_blocks::{
        state::State, variable_blocks::AssignStateBlock, BlockExecutionError, Value,
    };

    #[test]
    fn test_if_block_true_condition() {
        let mut state = State::new();
        let condition = true;
        let assign_block = AssignStateBlock::new(
            "if_executed".to_string(),
            Value::String("executed".to_string()),
        );
        let if_block = IfBlock::new(Blocks(vec![Box::new(assign_block)]), condition);

        if_block.preform(&mut state).unwrap();
        assert_eq!(
            state.get("if_executed").unwrap(),
            &Value::String("executed".to_string())
        );
    }

    #[test]
    fn test_if_block_false_condition() {
        let mut state = State::new();
        let condition = false;
        let assign_block = AssignStateBlock::new(
            "if_executed".to_string(),
            Value::String("executed".to_string()),
        );
        let if_block = IfBlock::new(Blocks(vec![Box::new(assign_block)]), condition);

        if_block.preform(&mut state).unwrap();
        assert!(state.get("if_executed").is_none());
    }
}
