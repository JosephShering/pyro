mod enemies;
mod htn;
mod interaction;
mod level;
mod player;
mod statechart;
mod utility_ai;
use godot::prelude::*;

struct Pyro;

#[gdextension]
unsafe impl ExtensionLibrary for Pyro {}
