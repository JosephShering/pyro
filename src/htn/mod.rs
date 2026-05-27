pub mod blackboard;
pub mod operator;
pub mod planner;
pub mod selector;
pub mod sequence;
pub mod task;

use godot::prelude::*;

pub trait Plan {
    fn is_met(&self, blackboard: Gd<blackboard::Blackboard>) -> bool;
    fn decompose(&self, blackboard: Gd<blackboard::Blackboard>) -> Array<Gd<operator::Operator>>;
}

fn is_met(
    preconditions: &Dictionary<StringName, bool>,
    blackboard: Gd<blackboard::Blackboard>,
) -> bool {
    for (key, value) in preconditions {
        if let Some(bb_value) = &blackboard.bind().data.get(&key)
            && bb_value != &value
        {
            return false;
        }
    }

    return true;
}
