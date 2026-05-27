use godot::prelude::*;

use crate::htn::{Plan, blackboard::Blackboard, is_met, operator::Operator};

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct Task {
    #[export]
    preconditions: Dictionary<StringName, bool>,

    #[export]
    effects: Dictionary<StringName, bool>,

    #[export]
    operator: OnEditor<Gd<Operator>>,

    base: Base<Resource>,
}

#[godot_api]
impl IResource for Task {}

#[godot_dyn]
impl Plan for Task {
    fn decompose(&self, _blackboard: Gd<Blackboard>) -> Array<Gd<Operator>> {
        let mut arr = Array::new();
        arr.push(&self.operator.clone());

        return arr;
    }

    fn is_met(&self, blackboard: Gd<Blackboard>) -> bool {
        is_met(&self.preconditions, blackboard)
    }
}
