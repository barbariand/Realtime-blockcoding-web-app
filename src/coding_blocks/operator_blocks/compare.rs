use std::{cmp::Ordering, marker::PhantomData};

use crate::coding_blocks::state::Value;

trait Comparator<L: GetValue, R: GetValue> {
    fn cmp(ord: Ordering) -> bool;
    fn get_cmp(left: &L, right: R) -> bool {
        Self::cmp(left.value().cmp(right.value()))
    }
}
trait GetValue {
    fn value(&self) -> &Value;
}
struct Comparisons<C: Comparator<L, R>, L: GetValue, R: GetValue> {
    right: R,
    left: L,
    phantomcomp: PhantomData<C>,
}
struct LessThan;
impl<L: GetValue, R: GetValue> Comparator<L, R> for LessThan {
    fn cmp(ord: Ordering) -> bool {
        ord.is_lt()
    }
}
impl LessThan {
    fn new<R: GetValue, L: GetValue>(right: R, left: L) -> Comparisons<LessThan, L, R> {
        Comparisons {
            right,
            left,
            phantomcomp: PhantomData::default(),
        }
    }
}
struct GreaterThan;
impl<L: GetValue, R: GetValue> Comparator<L, R> for GreaterThan {
    fn cmp(ord: Ordering) -> bool {
        ord.is_gt()
    }
}
impl GreaterThan {
    fn new<R: GetValue, L: GetValue>(right: R, left: L) -> Comparisons<GreaterThan, L, R> {
        Comparisons {
            right,
            left,
            phantomcomp: PhantomData::default(),
        }
    }
}
struct Equal;
impl<L: GetValue, R: GetValue> Comparator<L, R> for Equal {
    fn cmp(ord: Ordering) -> bool {
        ord.is_eq()
    }
}
impl Equal {
    fn new<R: GetValue, L: GetValue>(right: R, left: L) -> Comparisons<Equal, L, R> {
        Comparisons {
            right,
            left,
            phantomcomp: PhantomData::default(),
        }
    }
}
