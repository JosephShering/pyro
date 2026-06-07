use godot::{
    classes::{CollisionObject3D, Texture2D},
    prelude::*,
};

pub trait IInteractable {
    fn interact(&mut self);
    fn on_hover(&mut self) {}
    fn on_blur(&mut self) {}
    fn interact_icon(&mut self) -> Option<Texture2D> {
        None
    }
    fn interact_text(&mut self) -> Option<GString> {
        None
    }
}

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct Interactable {
    #[export]
    interactable: OnEditor<Gd<CollisionObject3D>>,
    base: Base<Node>,
}

#[godot_api]
impl INode for Interactable {}
