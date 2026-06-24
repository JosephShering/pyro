use godot::{classes::Curve, prelude::*};

use crate::ai::NPCBlackboards;

#[derive(GodotClass)]
#[class(base=Resource)]
pub struct Consideration {
    #[export]
    key: StringName,

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
            key: StringName::default(),
            max: 1.0,
            curve: None,
            base,
        }
    }
}

#[godot_api]
impl Consideration {
    pub fn get_value(&self, actor_key: &str) -> f32 {
        let blackboards = NPCBlackboards::singleton();
        let bb_key = self.key.clone();

        let Some(float_value) = blackboards.bind().with_blackboard(actor_key, |blackboard| {
            let value = blackboard.bind().get(bb_key.clone());
            if value.booleanize() == false {
                godot_error!("Could not find value for key {}", self.key);
                return 0.0;
            }

            let Ok(float_value) = value.try_to::<f32>() else {
                godot_error!("Could not convert {} to a float", self.key);
                return 0.0;
            };

            let normalized_value = float_value.clamp(0.0, self.max) / self.max;
            match self.curve.as_ref() {
                Some(curve) => curve.sample_baked(normalized_value),
                None => {
                    godot_print!("Consideration has no curve");

                    0.0
                }
            }
        }) else {
            return 0.0 as f32;
        };

        float_value
    }
}
