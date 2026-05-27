use godot::prelude::*;

use crate::htn::{Plan, blackboard::Blackboard, is_met, operator::Operator};

#[derive(GodotClass)]
#[class(init, base=Resource)]
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
impl Sequence {}

#[godot_dyn]
impl Plan for Sequence {
    fn decompose(&self, mut blackboard: Gd<Blackboard>) -> Array<Gd<Operator>> {
        let mut operators = Array::new();

        for task in self.tasks.iter_shared() {
            let child_operators = task.dyn_bind().decompose(blackboard.clone());
            if child_operators.is_empty() {
                return Array::new();
            } else {
                operators.extend_array(&child_operators);
            }
        }

        return operators;
    }

    fn is_met(&self, blackboard: Gd<Blackboard>) -> bool {
        is_met(&self.preconditions, blackboard)
    }
}
