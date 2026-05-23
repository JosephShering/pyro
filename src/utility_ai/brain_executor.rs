use godot::prelude::*;

use crate::utility_ai::brain::Brain;

#[derive(GodotClass)]
#[class(base=Node)]
struct BrainExecutor {
    #[export]
    brain: Option<Gd<Brain>>,

    #[export]
    thoughts_per_minute: i32,

    #[var]
    choice: GString,

    time: f32,

    base: Base<Node>,
}

#[godot_api]
impl INode for BrainExecutor {
    fn init(base: Base<Node>) -> Self {
        Self {
            brain: None,
            thoughts_per_minute: 60,
            choice: GString::new(),
            time: 0.0,
            base,
        }
    }

    fn physics_process(&mut self, delta: f32) {
        self.time += delta;

        while self.time > self.timeout() {
            self.time -= self.timeout();
            let brain = self.brain.as_mut().unwrap();
            let choice = brain.bind_mut().run();
            godot_print!("{choice}");
        }
    }
}

#[godot_api]
impl BrainExecutor {
    fn timeout(&self) -> f32 {
        60.0 / self.thoughts_per_minute as f32
    }
}
