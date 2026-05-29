pub mod blackboard;
pub mod htn_planner;
pub mod operator;
pub mod selector;
pub mod sequence;
pub mod task;

use godot::prelude::*;

use crate::htn::operator::operator::Operator;

pub type BlackboardData = Dictionary<StringName, bool>;
pub type DecomposeType = (Array<Gd<Operator>>, BlackboardData);

pub trait Plan {
    fn is_met(&self, blackboard: &BlackboardData) -> bool;
    fn decompose(&self, blackboard: BlackboardData) -> DecomposeType;
}

fn is_met(preconditions: &Dictionary<StringName, bool>, blackboard: &BlackboardData) -> bool {
    for (key, value) in preconditions {
        if let Some(bb_value) = blackboard.get(&key) {
            if bb_value != value {
                return false;
            }
        } else {
            return false;
        }
    }

    return true;
}
