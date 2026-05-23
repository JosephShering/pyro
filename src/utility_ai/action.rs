use godot::prelude::*;

use crate::utility_ai::{blackboard::Blackboard, consideration::Consideration};

#[derive(GodotClass)]
#[class(base=Resource)]
pub struct Action {
    #[export]
    pub action_name: GString,

    #[export]
    considerations: Array<Gd<Consideration>>,

    base: Base<Resource>,
}

#[godot_api]
impl IResource for Action {
    fn init(base: Base<Resource>) -> Self {
        Self {
            action_name: GString::from(""),
            considerations: Array::new(),
            base,
        }
    }
}

#[godot_api]
impl Action {
    pub fn run(&self, blackboard: &Gd<Blackboard>) -> f32 {
        let score: f32 = self
            .considerations
            .iter_shared()
            // .filter(|consideration| consideration.clone().try_cast::<Consideration>().is_ok())
            .map(|consideration| consideration.bind().get_value(&blackboard))
            .product();

        let num_considerations = self.considerations.len();
        let mod_factor = 1.0 - (1.0 / num_considerations as f32);
        score + ((1.0 - score) * mod_factor * score)
    }
}
