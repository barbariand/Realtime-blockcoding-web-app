use serde::{Deserialize, Serialize};

use crate::coding_blocks::{GetValue, ValueEnum};

use super::*;
#[derive(Serialize, Deserialize)]
pub struct ForLoopBlock {
    times: ValueEnum,
    blocks: Blocks,
}
impl Block for ForLoopBlock {
    fn preform(&self, state: &mut State) -> Result<(), BlockExecutionError> {
        for _ in 0..(match self.times.get_value(state) {
            crate::coding_blocks::state::Value::String(_) => 0,
            crate::coding_blocks::state::Value::Number(f) => f.ceil() as i64,
        }) {
            self.blocks.execute(state)?
        }
        Ok(())
    }
}
impl ForLoopBlock {
    fn new<V: Into<ValueEnum>>(blocks: Blocks, times: V) -> Self {
        ForLoopBlock {
            times: times.into(),
            blocks,
        }
    }
}

#[cfg(test)]
mod forloop_block_tests {
    use super::*;
    use crate::coding_blocks::{
        state::State, variable_blocks::AssignStateBlock, BlockExecutionError, Value,
    };

    #[test]
    fn test_for_loop_block() {
        let mut state = State::new();
        let assign_block =
            AssignStateBlock::new("looped".to_string(), Value::String("yes".to_string()));
        let for_loop_block =
            ForLoopBlock::new::<Value>(Blocks(vec![assign_block.into()]), 3.0.into());

        for_loop_block.execute(&mut state).unwrap();
        assert_eq!(
            state.get("looped").unwrap(),
            &Value::String("yes".to_string())
        );
    }
}
