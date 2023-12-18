use crate::coding_blocks::conditons::Condition;

pub struct NotCondition<C: Condition> {
    condition: C,
}
impl<C: Condition> Condition for NotCondition<C> {
    fn evaluate(&self, state: &crate::coding_blocks::state::State) -> bool {
        !self.condition.evaluate(state)
    }
}
pub struct AndBlock<L: Condition, R: Condition> {
    left: L,
    right: R,
}
impl<L: Condition, R: Condition> Condition for AndBlock<L, R> {
    fn evaluate(&self, state: &crate::coding_blocks::state::State) -> bool {
        self.left.evaluate(state) && self.right.evaluate(state)
    }
}
pub struct OrBlock<L: Condition, R: Condition> {
    left: L,
    right: R,
}
impl<L: Condition, R: Condition> Condition for OrBlock<L, R> {
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
            condition: true_condition,
        };
        let not_block_false = NotCondition {
            condition: false_condition,
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
            left: true,
            right: true,
        };
        let and_block_tf = AndBlock {
            left: true,
            right: false,
        };
        let and_block_ft = AndBlock {
            left: false,
            right: true,
        };
        let and_block_ff = AndBlock {
            left: false,
            right: false,
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
            left: true,
            right: true,
        };
        let or_block_tf = OrBlock {
            left: true,
            right: false,
        };
        let or_block_ft = OrBlock {
            left: false,
            right: true,
        };
        let or_block_ff = OrBlock {
            left: false,
            right: false,
        };

        assert_eq!(or_block_tt.evaluate(&state), true || true);
        assert_eq!(or_block_tf.evaluate(&state), true || false);
        assert_eq!(or_block_ft.evaluate(&state), false || true);
        assert_eq!(or_block_ff.evaluate(&state), false || false);
    }
}
