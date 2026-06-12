use godot::prelude::*;

use crate::glue::interaction::interactable::IInteractable;

#[derive(GodotClass)]
#[class(init, base=Node3D)]
struct Campfire {
    #[export]
    seats: Array<Gd<Node3D>>,

    resvervations: Vec<bool>,

    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for Campfire {
    fn ready(&mut self) {
        self.resvervations = self.seats.iter_shared().map(|_| false).collect();
    }
}

#[godot_dyn]
impl IInteractable for Campfire {
    // fn interact(&mut self) {}
}
