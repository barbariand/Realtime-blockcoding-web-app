use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::coding_blocks::{
    state::{State, Value},
    GetValue, ValueEnum,
};

trait MathOp {
    fn calc(left: &ValueEnum, right: &ValueEnum, state: &State) -> Value;
    fn new(left: ValueEnum, right: ValueEnum) -> SimpleMathOperation {
        SimpleMathOperation {
            left,
            right,
            op: Self::get_op(),
        }
    }
    fn get_op() -> MathOpEnum;
}

pub struct Plus;
impl MathOp for Plus {
    fn calc(left: &ValueEnum, right: &ValueEnum, state: &State) -> Value {
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
    fn get_op() -> MathOpEnum {
        MathOpEnum::Plus
    }
}
struct Minus;
impl MathOp for Minus {
    fn calc(left: &ValueEnum, right: &ValueEnum, state: &State) -> Value {
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
    fn get_op() -> MathOpEnum {
        MathOpEnum::Minus
    }
}
struct Division;
impl MathOp for Division {
    fn calc(left: &ValueEnum, right: &ValueEnum, state: &State) -> Value {
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
    fn get_op() -> MathOpEnum {
        MathOpEnum::Division
    }
}
struct Multiply;
impl MathOp for Multiply {
    fn calc(left: &ValueEnum, right: &ValueEnum, state: &State) -> Value {
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
    fn get_op() -> MathOpEnum {
        MathOpEnum::Multiply
    }
}
struct Pow;
impl MathOp for Pow {
    fn calc(left: &ValueEnum, right: &ValueEnum, state: &State) -> Value {
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
    fn get_op() -> MathOpEnum {
        MathOpEnum::Pow
    }
}
struct Log;
impl MathOp for Log {
    fn calc(left: &ValueEnum, right: &ValueEnum, state: &State) -> Value {
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
    fn get_op() -> MathOpEnum {
        MathOpEnum::Log
    }
}
struct Mod;
impl MathOp for Mod {
    fn calc(left: &ValueEnum, right: &ValueEnum, state: &State) -> Value {
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
    fn get_op() -> MathOpEnum {
        MathOpEnum::Mod
    }
}
#[derive(Serialize, Deserialize)]
pub struct SimpleMathOperation {
    left: ValueEnum,
    right: ValueEnum,
    op: MathOpEnum,
}

impl GetValue for SimpleMathOperation {
    fn get_value(&self, state: &State) -> Value {
        self.op.calc(&self.left, &self.right, state)
    }
}
#[derive(Serialize, Deserialize)]
enum MathOpEnum {
    Plus,
    Minus,
    Division,
    Multiply,
    Pow,
    Log,
    Mod,
}
impl MathOpEnum {
    fn calc(&self, left: &ValueEnum, right: &ValueEnum, state: &State) -> Value {
        match self {
            MathOpEnum::Plus => Plus::calc(left, right, state),
            MathOpEnum::Minus => Minus::calc(left, right, state),
            MathOpEnum::Division => Division::calc(left, right, state),
            MathOpEnum::Multiply => Multiply::calc(left, right, state),
            MathOpEnum::Pow => Pow::calc(left, right, state),
            MathOpEnum::Log => Log::calc(left, right, state),
            MathOpEnum::Mod => Mod::calc(left, right, state),
        }
    }
}
