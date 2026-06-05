use std::{cmp::Ordering, collections::HashMap};

pub fn selector(preconditions: Vec<Condition>, tasks: Vec<Box<dyn Task>>) -> Selector {
    Selector {
        preconditions,
        tasks,
    }
}

pub fn sequence(preconditions: Vec<Condition>, tasks: Vec<Box<dyn Task>>) -> Sequence {
    Sequence {
        preconditions,
        tasks,
    }
}

pub fn action(preconditions: Vec<Condition>, action: String) -> Action {
    Action {
        preconditions,
        action,
    }
}

enum Value {
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::Float(l0), Self::Float(r0)) => l0 == r0,
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Bool(a), Self::Bool(b)) => a.partial_cmp(b),
            (Self::Int(a), Self::Int(b)) => a.partial_cmp(b),
            (Self::Float(a), Self::Float(b)) => a.partial_cmp(b),
            (Self::String(a), Self::String(b)) => a.partial_cmp(b),
            (Self::Int(a), Self::Float(b)) => (*a as f32).partial_cmp(b),
            (Self::Float(a), Self::Int(b)) => a.partial_cmp(&(*b as f32)),
            _ => None,
        }
    }
}

enum ComparisonOp {
    E,
    NE,
    LT,
    GT,
    GTE,
    LTE,
}

struct Condition {
    blackboard_key: String,
    op: ComparisonOp,
    value: Value,
}

enum DecompositionError {
    NoMetConditions,
}

type DecomposeReturn = Result<Vec<String>, DecompositionError>;

trait Task {
    fn is_met(&self, data: &HashMap<String, Value>) -> bool;
    fn decompose(&self, data: HashMap<String, Value>) -> DecomposeReturn;
}

fn is_met(data: &HashMap<String, Value>, preconditions: &Vec<Condition>) -> bool {
    for precondition in preconditions.iter() {
        let key = &precondition.blackboard_key;
        if let Some(left_value) = data.get(key) {
            let right_value = &precondition.value;
            let is_true = match precondition.op {
                ComparisonOp::E => left_value == right_value,
                ComparisonOp::NE => left_value != right_value,
                ComparisonOp::LT => left_value < right_value,
                ComparisonOp::GT => left_value > right_value,
                ComparisonOp::GTE => left_value >= right_value,
                ComparisonOp::LTE => left_value <= right_value,
            };

            if !is_true {
                return false;
            }
        }
    }

    return true;
}

struct Selector {
    preconditions: Vec<Condition>,
    tasks: Vec<Box<dyn Task>>,
}

impl Task for Selector {
    fn is_met(&self, data: &HashMap<String, Value>) -> bool {
        is_met(data, &self.preconditions)
    }

    fn decompose(&self, data: HashMap<String, Value>) -> DecomposeReturn {
        for task in self.tasks.iter() {
            if task.is_met(&data) {
                return task.decompose(data);
            }
        }
        return Err(DecompositionError::NoMetConditions);
    }
}

struct Sequence {
    preconditions: Vec<Condition>,
    tasks: Vec<Box<dyn Task>>,
}

impl Task for Sequence {
    fn is_met(&self, data: &HashMap<String, Value>) -> bool {
        is_met(data, &self.preconditions)
    }

    fn decompose(&self, data: HashMap<String, Value>) -> DecomposeReturn {
        for task in self.tasks.iter() {
            if !task.is_met(&data) {
                return task.decompose(data);
            }
        }

        return Err(DecompositionError::NoMetConditions);
    }
}

struct Action {
    preconditions: Vec<Condition>,
    action: String,
}

impl Task for Action {
    fn is_met(&self, data: &HashMap<String, Value>) -> bool {
        is_met(data, &self.preconditions)
    }

    fn decompose(&self, _data: HashMap<String, Value>) -> DecomposeReturn {
        return Ok(vec![self.action.clone()]);
    }
}
