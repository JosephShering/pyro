use godot::prelude::*;

use crate::ai::{actor::Thinker, utility_ai::action::UtilityAction};

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct Brain {
    #[export]
    actions: Array<Gd<UtilityAction>>,

    pub window: Dictionary<GString, f32>,
    base: Base<Resource>,
}

#[godot_api]
impl Brain {
    pub fn think(&self, id: &str) -> Vec<String> {
        let scores: Vec<(GString, f32)> = self
            .actions
            .iter_shared()
            .map(|action| {
                let action_name = action.bind().action_name.clone();
                let score = action.bind().run(id);

                (action_name, score)
            })
            .collect();

        vec![
            scores
                .into_iter()
                .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .map(|(key, _)| key)
                .map(|key| key.into())
                .unwrap_or_default(),
        ]
    }
}

#[godot_dyn]
impl Thinker for Brain {
    fn think(&self, id: &str) -> Option<Vec<String>> {
        let plan = self.think(id);
        if plan.is_empty() { None } else { Some(plan) }
    }
}
