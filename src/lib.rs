mod core;
mod glue;
use godot::prelude::*;

struct Pyro;

#[gdextension]
unsafe impl ExtensionLibrary for Pyro {}
