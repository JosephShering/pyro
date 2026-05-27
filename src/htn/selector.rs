use godot::prelude::*;

use crate::htn::{Plan, blackboard::Blackboard, is_met, operator::Operator, sequence::Sequence};

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct Selector {
    #[export]
    pub preconditions: Dictionary<StringName, bool>,

    #[export]
    pub sequences: Array<Gd<Sequence>>,

    base: Base<Resource>,
}

#[godot_dyn]
impl Plan for Selector {
    fn decompose(&self, mut blackboard: Gd<Blackboard>) -> Array<Gd<Operator>> {
        //TODO -- figure out where we need to duplicate the blackboard so it
        for sequence in self.sequences.iter_shared() {
            if !sequence.bind().is_met(blackboard.clone()) {
                return Array::new();
            }

            let child_operators = sequence.bind().decompose(blackboard.clone());

            if child_operators.is_empty() {
                godot_print!("No operators in selector");
                // return Array::new();
                continue;
            } else {
                return child_operators;
            }
        }

        return Array::new();
    }

    fn is_met(&self, blackboard: Gd<Blackboard>) -> bool {
        is_met(&self.preconditions, blackboard)
    }
}
