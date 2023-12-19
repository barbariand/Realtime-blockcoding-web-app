use serde::{Deserialize, Serialize};

use crate::coding_blocks::{
    conditons::Condition,
    state::{self, State},
    ValueEnum,
};

use super::super::GetValue;
use std::{cmp::Ordering, marker::PhantomData};

pub trait Comparator {
    fn cmp(ord: Ordering) -> bool;
    fn get_cmp(left: &ValueEnum, right: &ValueEnum, state: &State) -> bool {
        Self::cmp(
            left.get_value(state)
                .partial_cmp(&right.get_value(state))
                .expect("this should never not be Some beacuse of Value implementation"),
        )
    }
    fn new(left: ValueEnum, right: ValueEnum) -> ComparisonBlock {
        ComparisonBlock {
            left,
            right,
            op: Self::get_op(),
        }
    }
    fn get_op() -> ComparisonEnum;
}
#[derive(Serialize, Deserialize)]
pub struct ComparisonBlock {
    right: ValueEnum,
    left: ValueEnum,
    op: ComparisonEnum,
}

impl Condition for ComparisonBlock {
    fn evaluate(&self, state: &State) -> bool {
        self.op.cmp(&self.left, &self.right, state)
    }
}

struct LessThan;
impl Comparator for LessThan {
    fn cmp(ord: Ordering) -> bool {
        ord.is_lt()
    }

    fn get_op() -> ComparisonEnum {
        ComparisonEnum::LessThan
    }
}

#[cfg(test)]
mod less_than_tests {
    use super::*;
    use crate::coding_blocks::{state::State, Condition, Value};

    #[test]
    fn test_less_than_positive() {
        let state = State::new();
        let less_than_block = LessThan::new(Value::Number(5.0).into(), Value::Number(10.0).into());
        assert!(less_than_block.evaluate(&state)); // 5.0 < 10.0 should be true
    }

    #[test]
    fn test_less_than_negative() {
        let state = State::new();
        let less_than_block = LessThan::new(Value::Number(10.0).into(), Value::Number(5.0).into());
        assert!(!less_than_block.evaluate(&state)); // 10.0 < 5.0 should be false
    }
}

struct GreaterThan;
impl Comparator for GreaterThan {
    fn cmp(ord: Ordering) -> bool {
        ord.is_gt()
    }

    fn get_op() -> ComparisonEnum {
        ComparisonEnum::GreaterThan
    }
}

#[cfg(test)]
mod greater_than_tests {
    use super::*;
    use crate::coding_blocks::{state::State, Condition, Value};

    #[test]
    fn test_greater_than_positive() {
        let state = State::new();
        let greater_than_block =
            GreaterThan::new(Value::Number(10.0).into(), Value::Number(5.0).into());
        assert!(greater_than_block.evaluate(&state)); // 10.0 > 5.0 should be true
    }

    #[test]
    fn test_greater_than_negative() {
        let state = State::new();
        let greater_than_block =
            GreaterThan::new(Value::Number(5.0).into(), Value::Number(10.0).into());
        assert!(!greater_than_block.evaluate(&state)); // 5.0 > 10.0 should be false
    }
}

struct Equal;
impl Comparator for Equal {
    fn cmp(ord: Ordering) -> bool {
        ord.is_eq()
    }

    fn get_op() -> ComparisonEnum {
        ComparisonEnum::Equal
    }
}

#[cfg(test)]
mod equal_tests {
    use super::*;
    use crate::coding_blocks::{state::State, Condition, Value};

    #[test]
    fn test_equal_positive() {
        let state = State::new();
        let equal_block = Equal::new(Value::Number(7.0).into(), Value::Number(7.0).into());
        assert!(equal_block.evaluate(&state)); // 7.0 == 7.0 should be true
    }

    #[test]
    fn test_equal_negative() {
        let state = State::new();
        let equal_block = Equal::new(Value::Number(7.0).into(), Value::Number(8.0).into());
        assert!(!equal_block.evaluate(&state)); // 7.0 == 8.0 should be false
    }
}
#[derive(Serialize, Deserialize)]
enum ComparisonEnum {
    LessThan,
    Equal,
    GreaterThan,
}
impl ComparisonEnum {
    fn cmp(&self, left: &ValueEnum, right: &ValueEnum, state: &State) -> bool {
        match self {
            ComparisonEnum::LessThan => LessThan::get_cmp(left, right, state),
            ComparisonEnum::Equal => Equal::get_cmp(left, right, state),
            ComparisonEnum::GreaterThan => GreaterThan::get_cmp(left, right, state),
        }
    }
}
