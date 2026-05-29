use godot::prelude::*;

use crate::htn::{BlackboardData, operator::operator::Operative};

#[derive(GodotClass)]
#[class(init, base=Resource)]
struct NavigateToPoint {
    base: Base<Resource>,
}

#[godot_dyn]
impl Operative for NavigateToPoint {
    fn enter(&mut self, blackboard: BlackboardData) {}

    fn tick(&mut self, blackboard: BlackboardData, delta: f32) {}

    fn exit(&mut self, blackboard: BlackboardData) {}
}
