use std::sync::Arc;

use godot::prelude::*;

use crate::core::{action::Action, htn::action_library::ActionsRepo};

macro_rules! action_library {
    ( $( $name:literal => $action:expr ),* $(,)? ) => {{
        let mut lib = ActionsRepo::new();
        $( lib.register($name, $action); )*
        lib
    }};
}

#[derive(GodotClass)]
#[class(singleton, base=Node)]
pub struct ActionLibrary {
    lib: ActionsRepo,
    base: Base<Node>,
}

#[godot_api]
impl INode for ActionLibrary {
    fn init(base: Base<Node>) -> Self {
        let go_to_location = Action::start().build();
        let take_cover = Action::start().build();
        let shoot_at_target = Action::start().build();
        let reload = Action::start().build();

        let lib = action_library! {
            "go_to_location" => go_to_location,
            "take_cover" => take_cover,
            "shoot_at_target" => shoot_at_target,
            "reload" => reload
        };

        Self { lib, base }
    }
}

#[godot_api]
impl ActionLibrary {
    pub fn get(&self, key: String) -> Option<Arc<Action>> {
        self.lib.get(&key)
    }
}
