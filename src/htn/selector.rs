use godot::prelude::*;

use crate::htn::{
    BlackboardData, DecomposeType, Plan, blackboard::Blackboard, is_met, sequence::Sequence,
};

#[derive(GodotClass)]
#[class(init, tool, base=Resource)]
pub struct Selector {
    #[export]
    pub preconditions: Dictionary<StringName, bool>,

    #[export]
    pub sequences: Array<Gd<Sequence>>,

    base: Base<Resource>,
}

#[godot_dyn]
impl Plan for Selector {
    fn decompose(&self, blackboard: BlackboardData) -> DecomposeType {
        for sequence in self.sequences.iter_shared() {
            let is_met = sequence.bind().is_met(&blackboard);
            if !is_met {
                return (Array::new(), blackboard);
            }

            let new_blackboard = blackboard.duplicate_deep();
            let (child_operators, bb) = sequence.bind().decompose(new_blackboard);

            if child_operators.is_empty() {
                return (Array::new(), blackboard);
            } else {
                return (child_operators, bb);
            }
        }

        return (Array::new(), blackboard);
    }

    fn is_met(&self, blackboard: &BlackboardData) -> bool {
        is_met(&self.preconditions, blackboard)
    }
}
