use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct Blackboard {
    #[export]
    pub data: Dictionary<StringName, bool>,
    base: Base<Resource>,
}

#[godot_api]
impl IResource for Blackboard {}

#[godot_api]
impl Blackboard {}
