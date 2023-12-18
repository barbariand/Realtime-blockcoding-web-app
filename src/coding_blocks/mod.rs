pub mod conditons;
pub mod error;
use self::{
    control_blocks::{ForLoopBlock, IfBlock, IfElseBlock, IntoTimes},
    state::{State, Value},
    variable_blocks::AssignStateBlock,
};
use conditons::Condition;
use error::BlockExecutionError;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
pub mod control_blocks;
pub mod operator_blocks;
pub mod state;
pub mod variable_blocks;

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

trait SerializableBlock: Block + Serialize + for<'a> Deserialize<'a> {}
#[derive(Serialize, Deserialize)]
pub struct Blocks(Vec<Box<BlockEnum<dyn IntoTimes, dyn GetValue, dyn Condition>>>);
impl ExecuteBlock for Blocks {
    fn execute(&self, state: &mut State) -> Result<(), BlockExecutionError> {
        for block in self.0.iter() {
            block.preform(state)?;
        }
        Ok(())
    }
}

impl Blocks {
    fn new() -> Self {
        Self(Vec::new())
    }
    fn push_end(&mut self, block: dyn Block) {
        self.0.push(block)
    }
    fn push_index(&mut self, block: dyn Block, index: usize) {
        self.0.insert((self.0.len() + 1).min(index), block)
    }
}
trait GetValue {
    fn get_value(&self, state: &State) -> Value;
}
enum BlockEnum<T: IntoTimes, G: GetValue, C: Condition> {
    ForLoop(ForLoopBlock<T>),
    IfBlock(IfBlock<C>),
    IfElseBlock(IfElseBlock<C>),
    AsignBlockTesting(AssignStateBlock<G>),
}
