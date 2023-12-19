use serde::{Deserialize, Serialize};

use crate::coding_blocks::conditons::{Condition, ConditionEnum};
#[derive(Serialize, Deserialize)]
pub struct NotCondition {
    condition: ConditionEnum,
}
impl Condition for NotCondition {
    fn evaluate(&self, state: &crate::coding_blocks::state::State) -> bool {
        !self.condition.evaluate(state)
    }
}
#[derive(Serialize, Deserialize)]
pub struct AndBlock {
    left: ConditionEnum,
    right: ConditionEnum,
}
impl Condition for AndBlock {
    fn evaluate(&self, state: &crate::coding_blocks::state::State) -> bool {
        self.left.evaluate(state) && self.right.evaluate(state)
    }
}
#[derive(Serialize, Deserialize)]
pub struct OrBlock {
    left: ConditionEnum,
    right: ConditionEnum,
}
impl Condition for OrBlock {
    fn evaluate(&self, state: &crate::coding_blocks::state::State) -> bool {
        self.left.evaluate(state) || self.right.evaluate(state)
    }
}
#[cfg(test)]
mod not_block_tests {
    use super::*;
    use crate::coding_blocks::state::State;

    #[test]
    fn test_not_block() {
        let mut state = State::new();
        let true_condition = true;
        let false_condition = false;

        let not_block_true = NotCondition {
            condition: true_condition.into(),
        };
        let not_block_false = NotCondition {
            condition: false_condition.into(),
        };

        assert_eq!(not_block_true.evaluate(&state), !true_condition);
        assert_eq!(not_block_false.evaluate(&state), !false_condition);
    }
}
#[cfg(test)]
mod and_block_tests {
    use super::*;
    use crate::coding_blocks::state::State;

    #[test]
    fn test_and_block() {
        let mut state = State::new();

        let and_block_tt = AndBlock {
            left: true.into(),
            right: true.into(),
        };
        let and_block_tf = AndBlock {
            left: true.into(),
            right: false.into(),
        };
        let and_block_ft = AndBlock {
            left: false.into(),
            right: true.into(),
        };
        let and_block_ff = AndBlock {
            left: false.into(),
            right: false.into(),
        };

        assert_eq!(and_block_tt.evaluate(&state), true && true);
        assert_eq!(and_block_tf.evaluate(&state), true && false);
        assert_eq!(and_block_ft.evaluate(&state), false && true);
        assert_eq!(and_block_ff.evaluate(&state), false && false);
    }
}
#[cfg(test)]
mod or_block_tests {
    use super::*;
    use crate::coding_blocks::state::State;

    #[test]
    fn test_or_block() {
        let mut state = State::new();

        let or_block_tt = OrBlock {
            left: true.into(),
            right: true.into(),
        };
        let or_block_tf = OrBlock {
            left: true.into(),
            right: false.into(),
        };
        let or_block_ft = OrBlock {
            left: false.into(),
            right: true.into(),
        };
        let or_block_ff = OrBlock {
            left: false.into(),
            right: false.into(),
        };

        assert_eq!(or_block_tt.evaluate(&state), true || true);
        assert_eq!(or_block_tf.evaluate(&state), true || false);
        assert_eq!(or_block_ft.evaluate(&state), false || true);
        assert_eq!(or_block_ff.evaluate(&state), false || false);
    }
}
