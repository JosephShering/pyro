use godot::prelude::*;

use crate::utility_ai::consideration::Consideration;

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct Action {
    #[export]
    key: GString,

    #[export]
    blackboard: Dictionary<GString, f32>,

    #[export]
    considerations: Array<Gd<Consideration>>,

    base: Base<Resource>,
}

#[godot_api]
impl IResource for Action {}

#[godot_api]
impl Action {
    pub fn get_action_name(&self) -> &GString {
        &self.key
    }

    pub fn run(&self) -> f32 {
        self.considerations
            .iter_shared()
            .map(|consideration| consideration.bind().get_value(self.blackboard.clone()))
            .product()
    }
}
