pub mod blackboard;
pub mod htn_planner;
pub mod operator;
pub mod selector;
pub mod sequence;
pub mod task;

use std::collections::HashMap;

use godot::prelude::*;

use crate::htn::{blackboard::Blackboard, operator::Operator};

pub type BlackboardData = HashMap<StringName, bool>;
pub type DecomposeType = (Array<Gd<Operator>>, Gd<Blackboard>);

pub trait Plan {
    fn is_met(&self, blackboard: &Gd<Blackboard>) -> bool;
    fn decompose(&self, blackboard: Gd<Blackboard>) -> DecomposeType;
}

fn is_met(preconditions: &Dictionary<StringName, bool>, blackboard: &Gd<Blackboard>) -> bool {
    for (key, value) in preconditions {
        if let Some(bb_value) = blackboard.bind().data.get(&key) {
            if bb_value != value {
                return false;
            }
        } else {
            return false;
        }
    }

    return true;
}
