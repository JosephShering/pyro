use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct Operator {
    #[export]
    pub script: GString,

    base: Base<Resource>,
}

#[godot_api]
impl IResource for Operator {}

#[godot_api]
impl Operator {}
