use super::{
    operator_blocks::boolean_operations::{AndBlock, NotCondition, OrBlock},
    *,
};
pub trait Condition {
    fn evaluate(&self, state: &State) -> bool;
}

impl Condition for bool {
    fn evaluate(&self, _: &State) -> bool {
        *self
    }
}
#[derive(Serialize, Deserialize)]
pub enum ConditionEnum {
    NotCondition(Box<NotCondition>),
    AndBlock(Box<AndBlock>),
    OrBlock(Box<OrBlock>),
    Bool(bool),
}
impl Condition for ConditionEnum {
    fn evaluate(&self, state: &State) -> bool {
        match self {
            ConditionEnum::NotCondition(v) => v.evaluate(state),
            ConditionEnum::AndBlock(v) => v.evaluate(state),
            ConditionEnum::OrBlock(v) => v.evaluate(state),
            ConditionEnum::Bool(b) => *b,
        }
    }
}
impl From<bool> for ConditionEnum {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}
