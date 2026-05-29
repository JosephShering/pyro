use godot::{global::godot_str, prelude::*};

use crate::htn::BlackboardData;

pub trait Operative {
    fn enter(&mut self, blackboard: BlackboardData);
    fn tick(&mut self, blackboard: BlackboardData, delta: f32);
    fn exit(&mut self, blackboard: BlackboardData);
}

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
