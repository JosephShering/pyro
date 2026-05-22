use godot::prelude::*;

use crate::utility_ai::action::Action;

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct Brain {
    actions: Array<Gd<Action>>,

    pub window: Dictionary<GString, f32>,
    base: Base<Node>,
}

#[godot_api]
impl INode for Brain {}

#[godot_api]
impl Brain {
    #[func]
    fn run(&mut self) -> GString {
        let scores: Vec<(GString, f32)> = self
            .actions
            .iter_shared()
            .map(|action| {
                let action_name = action.bind().get_action_name().clone();
                let score = action.bind().run().clone();

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
