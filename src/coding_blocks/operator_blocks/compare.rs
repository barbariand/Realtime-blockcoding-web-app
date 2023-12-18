use serde::{Deserialize, Serialize};

use crate::coding_blocks::{conditons::Condition, state::State};

use super::super::GetValue;
use std::{cmp::Ordering, marker::PhantomData};

pub trait Comparator<L: GetValue, R: GetValue> {
    fn cmp(ord: Ordering) -> bool;
    fn get_cmp(left: &L, right: &R, state: &State) -> bool {
        Self::cmp(
            left.get_value(state)
                .partial_cmp(&right.get_value(state))
                .expect("this should never not be Some beacuse of Value implementation"),
        )
    }
}
#[derive(Serialize, Deserialize)]
pub struct ComparisonBlock<C: Comparator<L, R>, L: GetValue, R: GetValue> {
    right: R,
    left: L,
    phantomcomp: PhantomData<C>,
}

impl<L: GetValue, R: GetValue, C: Comparator<L, R>> Condition for ComparisonBlock<C, L, R> {
    fn evaluate(&self, state: &State) -> bool {
        C::get_cmp(&self.left, &self.right, state)
    }
}
impl<C: Comparator<L, R>, L: GetValue, R: GetValue> ComparisonBlock<C, L, R> {
    fn new(left: L, right: R) -> Self {
        Self {
            left,
            right,
            phantomcomp: PhantomData,
        }
    }
}

struct LessThan;
impl<L: GetValue, R: GetValue> Comparator<L, R> for LessThan {
    fn cmp(ord: Ordering) -> bool {
        ord.is_lt()
    }
}
impl LessThan {
    fn new<R: GetValue, L: GetValue>(left: L, right: R) -> ComparisonBlock<LessThan, L, R> {
        ComparisonBlock::new(left, right)
    }
}
#[cfg(test)]
mod less_than_tests {
    use super::*;
    use crate::coding_blocks::{state::State, Condition, Value};

    #[test]
    fn test_less_than_positive() {
        let state = State::new();
        let less_than_block = LessThan::new(Value::Number(5.0), Value::Number(10.0));
        assert!(less_than_block.evaluate(&state)); // 5.0 < 10.0 should be true
    }

    #[test]
    fn test_less_than_negative() {
        let state = State::new();
        let less_than_block = LessThan::new(Value::Number(10.0), Value::Number(5.0));
        assert!(!less_than_block.evaluate(&state)); // 10.0 < 5.0 should be false
    }
}

struct GreaterThan;
impl<L: GetValue, R: GetValue> Comparator<L, R> for GreaterThan {
    fn cmp(ord: Ordering) -> bool {
        ord.is_gt()
    }
}
impl GreaterThan {
    fn new<R: GetValue, L: GetValue>(left: L, right: R) -> ComparisonBlock<GreaterThan, L, R> {
        ComparisonBlock::new(left, right)
    }
}
#[cfg(test)]
mod greater_than_tests {
    use super::*;
    use crate::coding_blocks::{state::State, Condition, Value};

    #[test]
    fn test_greater_than_positive() {
        let state = State::new();
        let greater_than_block = GreaterThan::new(Value::Number(10.0), Value::Number(5.0));
        assert!(greater_than_block.evaluate(&state)); // 10.0 > 5.0 should be true
    }

    #[test]
    fn test_greater_than_negative() {
        let state = State::new();
        let greater_than_block = GreaterThan::new(Value::Number(5.0), Value::Number(10.0));
        assert!(!greater_than_block.evaluate(&state)); // 5.0 > 10.0 should be false
    }
}

struct Equal;
impl<L: GetValue, R: GetValue> Comparator<L, R> for Equal {
    fn cmp(ord: Ordering) -> bool {
        ord.is_eq()
    }
}
impl Equal {
    fn new<R: GetValue, L: GetValue>(left: L, right: R) -> ComparisonBlock<Equal, L, R> {
        ComparisonBlock::new(left, right)
    }
}
#[cfg(test)]
mod equal_tests {
    use super::*;
    use crate::coding_blocks::{state::State, Condition, Value};

    #[test]
    fn test_equal_positive() {
        let state = State::new();
        let equal_block = Equal::new(Value::Number(7.0), Value::Number(7.0));
        assert!(equal_block.evaluate(&state)); // 7.0 == 7.0 should be true
    }

    #[test]
    fn test_equal_negative() {
        let state = State::new();
        let equal_block = Equal::new(Value::Number(7.0), Value::Number(8.0));
        assert!(!equal_block.evaluate(&state)); // 7.0 == 8.0 should be false
    }
}
