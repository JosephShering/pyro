use godot::prelude::*;
use std::collections::HashMap;

use crate::ai::ai_action::AIAction;

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct ActionLibrary {
    actions: HashMap<String, Gd<AIAction>>,
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
    pub fn get(&mut self, name: &str) -> Option<Gd<AIAction>> {
        self.actions.get(name).cloned()
    }

    fn gather_actions(&mut self) {
        self.base().get_children().iter_shared().for_each(|child| {
            match child.try_cast::<AIAction>() {
                Ok(action) => {
                    let key = action.bind().key.to_string();
                    self.actions.insert(key, action);
                }
                Err(_) => {}
            }
        })
    }
}
