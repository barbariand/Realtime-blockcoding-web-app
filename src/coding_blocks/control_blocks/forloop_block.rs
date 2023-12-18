use serde::{Deserialize, Serialize};

use super::*;
#[derive(Serialize, Deserialize)]
pub struct ForLoopBlock<T: IntoTimes> {
    times: T,
    blocks: Blocks,
}
impl<T: IntoTimes> Block for ForLoopBlock<T> {
    fn preform(&self, state: &mut State) -> Result<(), BlockExecutionError> {
        for _ in 0..(self.times.into_times(state)) {
            self.blocks.execute(state)?
        }
        Ok(())
    }
}
#[allow(clippy::wrong_self_convention)]
pub trait IntoTimes {
    fn into_times(&self, state: &mut State) -> i32;
}
impl IntoTimes for i32 {
    fn into_times(&self, _: &mut State) -> i32 {
        *self
    }
}
impl IntoTimes for dyn Fn(&mut State) -> i32 {
    fn into_times(&self, state: &mut State) -> i32 {
        self(state)
    }
}
impl<T: IntoTimes> ForLoopBlock<T> {
    fn new(blocks: Blocks, times: T) -> Self {
        Self { times, blocks }
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
        let for_loop_block = ForLoopBlock::new(Blocks(vec![Box::new(assign_block)]), 3);

        for_loop_block.preform(&mut state).unwrap();
        assert_eq!(
            state.get("looped").unwrap(),
            &Value::String("yes".to_string())
        );
    }
}
