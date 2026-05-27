use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node)]
struct StateMachine {
    base: Base<Node>,
}

#[godot_api]
impl INode for StateMachine {}

#[godot_api]
impl StateMachine {}
