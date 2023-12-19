pub mod conditons;
pub mod error;
use self::{
    control_blocks::{ForLoopBlock, IfBlock, IfElseBlock},
    operator_blocks::two_variable_math::SimpleMathOperation,
    state::{State, Value},
    variable_blocks::{AssignStateBlock, GetValueBlock},
};
use conditons::Condition;
use error::BlockExecutionError;
use serde::{Deserialize, Serialize};
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
#[derive(Serialize, Deserialize)]
pub struct Blocks(Vec<BlockEnum>);
impl Block for Blocks {
    fn preform(&self, state: &mut State) -> Result<(), BlockExecutionError> {
        for block in self.0.iter() {
            block.execute(state)?;
        }
        Ok(())
    }
}

impl Blocks {
    fn new() -> Self {
        Self(Vec::new())
    }
    fn push_end(&mut self, block: BlockEnum) {
        self.0.push(block)
    }
    fn push_index(&mut self, block: BlockEnum, index: usize) {
        self.0.insert((self.0.len() + 1).min(index), block)
    }
}
trait GetValue {
    fn get_value(&self, state: &State) -> Value;
}
#[derive(Serialize, Deserialize)]
pub enum BlockEnum {
    ForLoopBlock(Box<ForLoopBlock>),
    IfBlock(Box<IfBlock>),
    IfElseBlock(Box<IfElseBlock>),
    AssignStateBlock(Box<AssignStateBlock>),
}
impl Block for BlockEnum {
    fn preform(&self, state: &mut State) -> Result<(), BlockExecutionError> {
        match self {
            BlockEnum::ForLoopBlock(v) => v.execute(state),
            BlockEnum::IfBlock(v) => v.execute(state),
            BlockEnum::IfElseBlock(v) => v.execute(state),
            BlockEnum::AssignStateBlock(v) => v.execute(state),
        }
    }
}
impl From<ForLoopBlock> for BlockEnum {
    fn from(value: ForLoopBlock) -> Self {
        BlockEnum::ForLoopBlock(Box::new(value))
    }
}
impl From<IfBlock> for BlockEnum {
    fn from(value: IfBlock) -> Self {
        BlockEnum::IfBlock(Box::new(value))
    }
}
impl From<IfElseBlock> for BlockEnum {
    fn from(value: IfElseBlock) -> Self {
        BlockEnum::IfElseBlock(Box::new(value))
    }
}
impl From<AssignStateBlock> for BlockEnum {
    fn from(value: AssignStateBlock) -> Self {
        BlockEnum::AssignStateBlock(Box::new(value))
    }
}
#[derive(Serialize, Deserialize)]
pub enum ValueEnum {
    Value(Box<Value>),
    GetValueBlock(Box<GetValueBlock>),
    SimpleMathOperation(Box<SimpleMathOperation>),
}
impl GetValue for ValueEnum {
    fn get_value(&self, state: &State) -> Value {
        match self {
            ValueEnum::Value(v) => v.get_value(state),
            ValueEnum::GetValueBlock(v) => v.get_value(state),
            ValueEnum::SimpleMathOperation(v) => v.get_value(state),
        }
    }
}
impl From<Value> for ValueEnum {
    fn from(value: Value) -> Self {
        ValueEnum::Value(Box::new(value))
    }
}
impl From<GetValueBlock> for ValueEnum {
    fn from(value: GetValueBlock) -> Self {
        Self::GetValueBlock(Box::new(value))
    }
}
impl From<SimpleMathOperation> for ValueEnum {
    fn from(value: SimpleMathOperation) -> Self {
        Self::SimpleMathOperation(Box::new(value))
    }
}
