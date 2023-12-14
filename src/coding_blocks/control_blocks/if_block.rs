use super::{Condition, *};
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
