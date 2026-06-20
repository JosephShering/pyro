mod ai;
mod campfire;
mod interaction;
mod inventory;
mod level;
mod player;
mod registry;
use godot::prelude::*;

struct Pyro;

#[gdextension]
unsafe impl ExtensionLibrary for Pyro {}
