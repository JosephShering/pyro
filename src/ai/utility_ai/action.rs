use godot::prelude::*;

use crate::ai::utility_ai::consideration::Consideration;

#[derive(GodotClass)]
#[class(base=Resource)]
pub struct UtilityAction {
    #[export]
    pub action_name: GString,

    #[export]
    considerations: Array<Gd<Consideration>>,

    base: Base<Resource>,
}

#[godot_api]
impl IResource for UtilityAction {
    fn init(base: Base<Resource>) -> Self {
        Self {
            action_name: GString::default(),
            considerations: Array::new(),
            base,
        }
    }
}

#[godot_api]
impl UtilityAction {
    pub fn run(&self, actor_key: &str) -> f32 {
        let score: f32 = self
            .considerations
            .iter_shared()
            .map(|consideration| consideration.bind().get_value(actor_key))
            .product();

        let num_considerations = self.considerations.len();
        let mod_factor = 1.0 - (1.0 / num_considerations as f32);
        score + ((1.0 - score) * mod_factor * score)
    }
}
