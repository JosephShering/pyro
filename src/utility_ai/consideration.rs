use godot::{classes::Curve, prelude::*};

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct Consideration {
    #[export]
    curve: OnEditor<Gd<Curve>>,

    base: Base<Resource>,
}

#[godot_api]
impl IResource for Consideration {}

#[godot_api]
impl Consideration {
    pub fn get_value(&self, blackboard: Dictionary<GString, f32>) -> f32 {
        self.calculate(blackboard, self.curve.clone())
    }

    #[func(virtual)]
    fn calculate(&self, _blackboard: Dictionary<GString, f32>, _curve: Gd<Curve>) -> f32 {
        0.0
    }
}
