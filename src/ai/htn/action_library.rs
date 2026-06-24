use super::htn_action::HTNAction;
use godot::prelude::*;
use std::collections::HashMap;

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
pub struct ActionLibrary {
    actions: HashMap<String, Gd<HTNAction>>,
    base: Base<Node>,
}

#[godot_api]
impl INode for ActionLibrary {
    fn ready(&mut self) {
        self.gather_actions();
    }
}

#[godot_api]
impl ActionLibrary {
    pub fn get(&mut self, name: &str) -> Option<Gd<HTNAction>> {
        self.actions.get(name).cloned()
    }

    fn gather_actions(&mut self) {
        self.base().get_children().iter_shared().for_each(|child| {
            match child.try_cast::<HTNAction>() {
                Ok(action) => {
                    let key = action.bind().key.to_string();
                    self.actions.insert(key, action);
                }
                Err(_) => {}
            }
        })
    }
}
