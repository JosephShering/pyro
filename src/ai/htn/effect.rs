use std::collections::HashMap;

use crate::ai::Value;

#[derive(Clone, Debug, PartialEq)]
pub enum ArithmeticOp {
    Eq,
    Add,
    Sub,
    Mult,
    Div,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Effect {
    blackboard_key: String,
    op: ArithmeticOp,
    value: Value,
}

impl Effect {
    pub fn new(key: impl Into<String>, op: ArithmeticOp, value: Value) -> Self {
        Self {
            blackboard_key: key.into(),
            op,
            value,
        }
    }

    pub fn apply(&self, data: &mut HashMap<String, Value>) {
        let key = &self.blackboard_key;
        let new_value = match self.op {
            ArithmeticOp::Eq => Some(self.value.clone()),
            ArithmeticOp::Add => data.get(key).map(|x| x + &self.value),
            ArithmeticOp::Sub => data.get(key).map(|x| x - &self.value),
            ArithmeticOp::Mult => data.get(key).map(|x| x * &self.value),
            ArithmeticOp::Div => data.get(key).map(|x| x / &self.value),
        };

        if let Some(v) = new_value {
            data.insert(key.clone(), v);
        }
    }
}
