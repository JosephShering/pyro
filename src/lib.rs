mod ai;
mod campfire;
mod interaction;
mod inventory;
mod level;
mod player;
use godot::prelude::*;

struct Pyro;

#[gdextension]
unsafe impl ExtensionLibrary for Pyro {}
