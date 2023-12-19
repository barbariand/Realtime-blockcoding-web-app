use serde::{Deserialize, Serialize};

use crate::coding_blocks::conditons::ConditionEnum;

use super::{Condition, *};
#[derive(Serialize, Deserialize)]
pub struct IfBlock {
    condition: ConditionEnum,
    blocks: Blocks,
}
impl Block for IfBlock {
    fn preform(&self, state: &mut State) -> Result<(), BlockExecutionError> {
        if self.condition.evaluate(state) {
            return self.blocks.execute(state);
        }
        Ok(())
    }
}
impl IfBlock {
    fn new(blocks: Blocks, condition: ConditionEnum) -> Self {
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
        println!("hello1");
        let if_block = IfBlock::new(Blocks(vec![assign_block.into()]), condition.into());
        println!("Hello");
        if_block.execute(&mut state).unwrap();
        println!("hello-1");
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
        let if_block = IfBlock::new(Blocks(vec![assign_block.into()]), condition.into());

        if_block.execute(&mut state).unwrap();
        assert!(state.get("if_executed").is_none());
    }
}
