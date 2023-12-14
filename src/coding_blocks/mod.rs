mod conditons;
pub mod error;
use self::state::State;
use conditons::Condition;
use error::BlockExecutionError;
pub mod control_blocks;
pub mod operator_blocks;
pub mod state;

pub trait ExecuteBlock {
    fn execute(&self, state: &mut State) -> Result<(), BlockExecutionError>;
}
impl<T: Block> ExecuteBlock for T {
    fn execute(&self, state: &mut State) -> Result<(), BlockExecutionError> {
        self.preform(state)
    }
}
trait Block {
    fn preform(&self, state: &mut State) -> Result<(), BlockExecutionError>;
}
pub struct Blocks(Vec<Box<dyn Block>>);
impl ExecuteBlock for Blocks {
    fn execute(&self, state: &mut State) -> Result<(), BlockExecutionError> {
        for block in self.0.iter() {
            block.preform(state)?
        }
        Ok(())
    }
}
