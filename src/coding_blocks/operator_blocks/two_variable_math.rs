use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::coding_blocks::{
    state::{State, Value},
    GetValue,
};

trait MathOp<L: GetValue, R: GetValue> {
    fn calc(left: &L, right: &R, state: &State) -> Value;
}

struct Plus;
impl<L: GetValue, R: GetValue> MathOp<L, R> for Plus {
    fn calc(left: &L, right: &R, state: &State) -> Value {
        Value::Number(
            match left.get_value(state) {
                Value::String(_) => 0.0,
                Value::Number(i) => i,
            } + match right.get_value(state) {
                Value::String(_) => 0.0,
                Value::Number(i) => i,
            },
        )
    }
}
struct Minus;
impl<L: GetValue, R: GetValue> MathOp<L, R> for Minus {
    fn calc(left: &L, right: &R, state: &State) -> Value {
        Value::Number(
            match left.get_value(state) {
                Value::String(_) => 0.0,
                Value::Number(i) => i,
            } - match right.get_value(state) {
                Value::String(_) => 0.0,
                Value::Number(i) => i,
            },
        )
    }
}
struct Division;
impl<L: GetValue, R: GetValue> MathOp<L, R> for Division {
    fn calc(left: &L, right: &R, state: &State) -> Value {
        Value::Number(
            match left.get_value(state) {
                Value::String(_) => 0.0,
                Value::Number(i) => i,
            } / match right.get_value(state) {
                Value::String(_) => 0.0,
                Value::Number(i) => i,
            },
        )
    }
}
struct Multiply;
impl<L: GetValue, R: GetValue> MathOp<L, R> for Multiply {
    fn calc(left: &L, right: &R, state: &State) -> Value {
        Value::Number(
            match left.get_value(state) {
                Value::String(_) => 0.0,
                Value::Number(i) => i,
            } * match right.get_value(state) {
                Value::String(_) => 0.0,
                Value::Number(i) => i,
            },
        )
    }
}
struct Pow;
impl<L: GetValue, R: GetValue> MathOp<L, R> for Pow {
    fn calc(left: &L, right: &R, state: &State) -> Value {
        Value::Number(
            match left.get_value(state) {
                Value::String(_) => 0.0,
                Value::Number(i) => i,
            }
            .powf(match right.get_value(state) {
                Value::String(_) => 0.0,
                Value::Number(i) => i,
            }),
        )
    }
}
struct Log;
impl<L: GetValue, R: GetValue> MathOp<L, R> for Log {
    fn calc(left: &L, right: &R, state: &State) -> Value {
        Value::Number(
            (match left.get_value(state) {
                Value::String(_) => 0.0,
                Value::Number(i) => i,
            })
            .log(match right.get_value(state) {
                Value::String(_) => 0.0,
                Value::Number(i) => i,
            }),
        )
    }
}
struct Mod;
impl<L: GetValue, R: GetValue> MathOp<L, R> for Mod {
    fn calc(left: &L, right: &R, state: &State) -> Value {
        Value::Number(
            match left.get_value(state) {
                Value::String(_) => 0.0,
                Value::Number(i) => i,
            }
            .rem_euclid(match right.get_value(state) {
                Value::String(_) => 0.0,
                Value::Number(i) => i,
            }),
        )
    }
}
#[derive(Serialize, Deserialize)]
struct SimpleMathOperation<M: MathOp<L, R>, L: GetValue, R: GetValue> {
    left: L,
    right: R,
    phantomop: PhantomData<M>,
}
impl<M: MathOp<L, R>, L: GetValue, R: GetValue> GetValue for SimpleMathOperation<M, L, R> {
    fn get_value(&self, state: &State) -> Value {
        M::calc(&self.left, &self.right, state)
    }
}
