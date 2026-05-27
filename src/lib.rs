pub mod enemies;
pub mod htn;
pub mod interaction;
pub mod level;
pub mod player;
pub mod statechart;
pub mod utility_ai;
use godot::prelude::*;

struct Pyro;

#[gdextension]
unsafe impl ExtensionLibrary for Pyro {}
