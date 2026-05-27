use godot::prelude::*;

use crate::htn::{blackboard::Blackboard, operator::Operator, selector::Selector};

#[derive(GodotClass)]
#[class(init, base=Node)]
struct Planner {
    #[export]
    pub compound_task: OnEditor<Gd<Selector>>,

    base: Base<Node>,
}

#[godot_api]
impl INode for Planner {}

#[godot_api]
impl Planner {
    #[func]
    pub fn plan(&mut self, blackboard: Gd<Blackboard>) -> Array<Gd<Operator>> {
        // let tasks = &self.compound_task.bind().tasks;
        // if tasks.len() <= 0 {
        //     godot_error!("Compound Task must have child tasks to plan");
        //     return Array::new();
        // }

        // //We disregard the preconditions on the root and just go right into it
        // let data = &blackboard.bind().data;

        // for task in tasks.iter_shared() {
        //     let preconditions = &task.bind().preconditions;
        // }

        return Array::new();
    }
}
