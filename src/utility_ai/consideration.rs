use godot::{classes::Curve, prelude::*};

use crate::utility_ai::blackboard::PyroUtilBlackboard;

#[derive(GodotClass)]
#[class(base=Resource)]
pub struct Consideration {
    #[export]
    key: GString,

    #[export]
    max: f32,

    #[export]
    curve: Option<Gd<Curve>>,

    base: Base<Resource>,
}

#[godot_api]
impl IResource for Consideration {
    fn init(base: Base<Resource>) -> Self {
        Self {
            key: GString::from(""),
            max: 1.0,
            curve: None,
            base,
        }
    }
}

#[godot_api]
impl Consideration {
    pub fn get_value(&self, blackboard: &Gd<PyroUtilBlackboard>) -> f32 {
        match blackboard.bind().data.get(&self.key) {
            Some(value) => {
                let normalized_value = value.clamp(0.0, self.max) / self.max;
                match self.curve.as_ref() {
                    Some(curve) => curve.sample_baked(normalized_value),
                    None => {
                        godot_print!("Consideration has no curve");

                        0.0
                    }
                }
            }
            None => {
                let key = &self.key;
                godot_print!("No key {key} in blackboard");

                0.0
            }
        }
    }
}
