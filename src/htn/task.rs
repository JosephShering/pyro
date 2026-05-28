use godot::prelude::*;

use crate::htn::{DecomposeType, Plan, blackboard::Blackboard, is_met, operator::Operator};

#[derive(GodotClass)]
#[class(init, tool, base=Resource)]
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
    fn decompose(&self, mut blackboard: Gd<Blackboard>) -> DecomposeType {
        let mut arr = Array::new();
        arr.push(&self.operator.clone());

        for (effect_key, effect_value) in self.effects.iter_shared() {
            blackboard.bind_mut().data.set(&effect_key, effect_value);
        }

        return (arr, blackboard);
    }

    fn is_met(&self, blackboard: &Gd<Blackboard>) -> bool {
        is_met(&self.preconditions, blackboard)
    }
}
