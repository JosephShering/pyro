use godot::{classes::Label, prelude::*};

use crate::ai::utility_ai::brain::Brain;

#[derive(GodotClass)]
#[class(init, base=Node)]
struct BrainDebugger {
    #[export]
    brain: OnEditor<Gd<Brain>>,

    #[export]
    label: OnEditor<Gd<Label>>,

    base: Base<Node>,
}

#[godot_api]
impl INode for BrainDebugger {
    fn physics_process(&mut self, _delta: f32) {
        let text: String = self
            .brain
            .bind()
            .window
            .iter_shared()
            .map(|(key, value)| format!("{}: {}\n", key, value))
            .collect();

        self.label.set_text(&text);
    }
}

#[godot_api]
impl BrainDebugger {}
