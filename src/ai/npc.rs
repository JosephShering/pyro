use godot::classes::{CharacterBody3D, ICharacterBody3D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=CharacterBody3D)]
pub struct NPC {
    base: Base<CharacterBody3D>,
}

#[godot_api]
impl ICharacterBody3D for NPC {
    fn ready(&mut self) {}
}
