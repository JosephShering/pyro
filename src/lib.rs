mod component;
mod enemies;
mod interaction;
mod player;
mod utility_ai;
use godot::prelude::*;

struct Pyro;

#[gdextension]
unsafe impl ExtensionLibrary for Pyro {}
