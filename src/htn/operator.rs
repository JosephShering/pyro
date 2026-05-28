use godot::{global::godot_str, prelude::*};

#[derive(GodotClass)]
#[class(init, tool, base=Resource)]
pub struct Operator {
    #[export]
    pub script: GString,

    #[export]
    pub name: GString,

    base: Base<Resource>,
}

#[godot_api]
impl IResource for Operator {
    fn to_string(&self) -> GString {
        return godot_str!("Operator: {}", &self.name);
    }
}

#[godot_api]
impl Operator {}
