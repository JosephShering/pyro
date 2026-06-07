use godot::prelude::*;

use crate::glue::utility_ai::{action::Action, blackboard::PyroUtilBlackboard};

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct Brain {
    #[export]
    blackboard: OnEditor<Gd<PyroUtilBlackboard>>,

    #[export]
    actions: Array<Gd<Action>>,

    pub window: Dictionary<GString, f32>,
    base: Base<Node>,
}

#[godot_api]
impl INode for Brain {}

#[godot_api]
impl Brain {
    #[func]
    pub fn run(&mut self) -> GString {
        let scores: Vec<(GString, f32)> = self
            .actions
            .iter_shared()
            .map(|action| {
                let action_name = action.bind().action_name.clone();
                let score = action.bind().run(&self.blackboard);

                (action_name, score)
            })
            .collect();

        self.window.clear();
        for (name, score) in &scores {
            self.window.set(name, *score);
        }

        scores
            .into_iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(key, _)| key)
            .unwrap_or_default()
    }
}
