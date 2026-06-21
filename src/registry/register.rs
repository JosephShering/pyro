use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node)]
struct Register {
    base: Base<Node>,
}

#[godot_api]
impl INode for Register {
    fn enter_tree(&mut self) {}
    fn exit_tree(&mut self) {}
}

#[godot_api]
impl Register {}
