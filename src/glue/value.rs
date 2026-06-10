use godot::meta::ToGodot;
use godot::prelude::*;

use crate::core::Value;

impl ToGodot for Value {
    type Pass = ByValue;

    fn to_godot(&self) -> <Self::Pass>::Output<'_, Self::Via> {
        match self {
            Value::String(string) => string,
            Value::Bool(bool) => bool,
            Value::Float(float) => float,
            Value::Int(int) => int,
        }
    }
}
