use std::collections::HashMap;

use godot::prelude::*;

use crate::glue::action::HTNAction;

#[derive(GodotConvert, Var, Export, Default, Debug, Clone, PartialEq)]
#[godot(via = u8)]
pub enum ActionStatus {
    #[default]
    Success,
    Failed,
    OnGoing,
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
    pub fn get_mut(&mut self, name: &str) -> Option<&mut Gd<HTNAction>> {
        self.actions.get_mut(name)
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
