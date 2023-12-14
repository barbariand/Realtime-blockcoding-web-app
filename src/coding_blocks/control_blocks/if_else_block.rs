use super::{Condition, *};
pub struct IfElseBlock<T: Condition> {
    condition: T,
    ifblocks: Blocks,
    elseblocks: Blocks,
}
impl<T: Condition> Block for IfElseBlock<T> {
    fn preform(&self, state: &mut State) -> Result<(), BlockExecutionError> {
        if self.condition.evaluate(state) {
            return self.ifblocks.execute(state);
        } else {
            return self.elseblocks.execute(state);
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
