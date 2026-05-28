use godot::prelude::*;

use crate::htn::{DecomposeType, Plan, blackboard::Blackboard, is_met};

#[derive(GodotClass)]
#[class(init, tool, base=Resource)]
pub struct Sequence {
    #[export]
    preconditions: Dictionary<StringName, bool>,

    #[export]
    tasks: Array<DynGd<Resource, dyn Plan>>,

    base: Base<Resource>,
}

#[godot_api]
impl IResource for Sequence {}

#[godot_api]
impl Sequence {
    pub fn decompose(&self, blackboard: Gd<Blackboard>) -> DecomposeType {
        let mut operators = Array::new();
        let mut new_blackboard = blackboard.duplicate_resource();

        for task in self.tasks.iter_shared() {
            let is_met = task.dyn_bind().is_met(&blackboard);
            if !is_met {
                return (operators, blackboard);
            }

            let (child_operators, bb) = task.dyn_bind().decompose(blackboard.clone());
            if child_operators.is_empty() {
                return (Array::new(), blackboard);
            } else {
                new_blackboard = bb;
                operators.extend_array(&child_operators);
            }
        }

        return (operators, new_blackboard);
    }

    pub fn is_met(&self, blackboard: &Gd<Blackboard>) -> bool {
        is_met(&self.preconditions, blackboard)
    }
}

// #[godot_dyn]
// impl Plan for Sequence {
// }
