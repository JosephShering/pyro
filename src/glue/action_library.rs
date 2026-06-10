use godot::prelude::*;

macro_rules! action_library {
    ( $( $name:literal => $action:expr ),* $(,)? ) => {{
        let mut lib = ActionsRepo::new();
        $( lib.register($name, $action); )*
        lib
    }};
}

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct ActionNode {
    #[export]
    key: GString,

    base: Base<Node>,
}

#[godot_api]
impl ActionNode {
    #[func(virtual)]
    pub fn enter(&mut self, data: Dictionary<GString, Variant>) {}

    #[func]
    pub fn update(&mut self, data: Dictionary<GString, Variant>, _delta: f32) {}

    #[func(virtual)]
    pub fn exit(&mut self, data: Dictionary<GString, Variant>) {}
}

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct ActionLibrary {
    actions: Vec<Gd<ActionNode>>,
    base: Base<Node>,
}

#[godot_api]
impl INode for ActionLibrary {
    fn ready(&mut self) {
        self.get_actions();
    }
}

#[godot_api]
impl ActionLibrary {
    pub fn get(&self, name: &str) -> Option<&Gd<ActionNode>> {
        self.actions.iter().find(|action| action.bind().key == name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Gd<ActionNode>> {
        self.actions
            .iter_mut()
            .find(|action| action.bind().key == name)
    }

    fn get_actions(&mut self) {
        self.base().get_children().iter_shared().map(|child| {
            match child.try_cast::<ActionNode>() {
                Ok(action) => {
                    self.actions.push(action);
                }
                Err(_) => {}
            }
        });
    }
}
