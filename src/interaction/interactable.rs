use godot::{classes::CollisionObject3D, prelude::*};

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct Interactable {
    #[export]
    interactable: OnEditor<Gd<CollisionObject3D>>,
    base: Base<Node>,
}

#[godot_api]
impl INode for Interactable {}

#[godot_api]
impl Interactable {
    pub fn interact(&mut self) {}
}
