use std::collections::HashMap;

use crate::ai::htn::core::value::Value;

#[derive(Clone, Debug, PartialEq)]
pub enum ComparisonOp {
    E,
    NE,
    LT,
    GT,
    GTE,
    LTE,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Condition {
    Compare {
        blackboard_key: String,
        op: ComparisonOp,
        value: Value,
    },
    All(Vec<Condition>),
    Any(Vec<Condition>),
    Not(Box<Condition>),
}

impl Condition {
    pub fn compare(key: impl Into<String>, op: ComparisonOp, value: Value) -> Self {
        Self::Compare {
            blackboard_key: key.into(),
            op,
            value,
        }
    }
}

pub fn eval(cond: &Condition, data: &HashMap<String, Value>) -> bool {
    match cond {
        Condition::Compare {
            blackboard_key,
            op,
            value,
        } => {
            let Some(left) = data.get(blackboard_key) else {
                return false;
            };
            match op {
                ComparisonOp::E => left == value,
                ComparisonOp::NE => left != value,
                ComparisonOp::LT => left < value,
                ComparisonOp::GT => left > value,
                ComparisonOp::GTE => left >= value,
                ComparisonOp::LTE => left <= value,
            }
        }
        Condition::All(cs) => cs.iter().all(|c| eval(c, data)),
        Condition::Any(cs) => cs.iter().any(|c| eval(c, data)),
        Condition::Not(c) => !eval(c, data),
    }
}
