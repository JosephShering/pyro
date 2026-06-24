use godot::prelude::*;

use crate::ai::blackboard::Blackboard;

#[derive(GodotConvert, Var, Export, Default, Debug, Clone, PartialEq)]
#[godot(via = u8)]
pub enum ActionUpdateStatus {
    #[default]
    Success,
    Failed,
    OnGoing,
}

#[derive(GodotConvert, Var, Export, Default, Debug, Clone, PartialEq)]
#[godot(via = u8)]
pub enum ActionEnterStatus {
    #[default]
    Success,
    Failed,
}

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct AIAction {
    #[export]
    pub key: GString,

    base: Base<Node>,
}

#[godot_api]
impl INode for AIAction {
    fn ready(&mut self) {
        if self.key.is_empty() {
            let node_name = self.base().get_name();
            godot_error!("Key for HTNAction {node_name} is empty");
        }
    }
}

#[godot_api]
impl AIAction {
    #[constant]
    const SUCCESS: i64 = 0;

    #[constant]
    const FAILED: i64 = 1;

    #[constant]
    const ONGOING: i64 = 2;

    #[func(virtual)]
    pub fn enter(&mut self, _data: Gd<Blackboard>) -> ActionEnterStatus {
        ActionEnterStatus::Success
    }

    #[func(virtual)]
    pub fn update(&mut self, _data: Gd<Blackboard>, _delta: f32) -> ActionUpdateStatus {
        ActionUpdateStatus::Success
    }

    #[func(virtual)]
    pub fn exit(&mut self, _data: Gd<Blackboard>) {}
}
