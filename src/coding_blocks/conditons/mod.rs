use super::*;
pub trait Condition {
    fn evaluate(&self, state: &State) -> bool;
}

impl Condition for dyn Fn(&State) -> bool {
    fn evaluate(&self, state: &State) -> bool {
        self(state)
    }
}
impl Condition for bool {
    fn evaluate(&self, _: &State) -> bool {
        *self
    }
}
