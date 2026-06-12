use std::collections::HashMap;

use godot::prelude::*;

use crate::core::Value;

#[derive(GodotClass, Debug)]
#[class(init, base=RefCounted)]
pub struct Blackboard {
    data: HashMap<String, Value>,
    base: Base<RefCounted>,
}

#[godot_api]
impl IRefCounted for Blackboard {}

#[godot_api]
impl Blackboard {
    pub fn get_data(&self) -> &HashMap<String, Value> {
        &self.data
    }

    #[func]
    pub fn get(&mut self, key: GString) -> Variant {
        match self.data.get(&key.to_string()) {
            Some(value) => match value {
                Value::Int(int) => int.to_variant(),
                Value::Float(float) => float.to_variant(),
                Value::Bool(bool) => bool.to_variant(),
                Value::String(string) => string.to_variant(),
            },
            None => Variant::nil(),
        }
    }

    #[func]
    pub fn set(&mut self, key: GString, value: Variant) {
        let v = match value.get_type() {
            VariantType::INT => Value::Int(value.to::<i32>()),
            VariantType::FLOAT => Value::Float(value.to::<f32>()),
            VariantType::BOOL => Value::Bool(value.to::<bool>()),
            VariantType::STRING | VariantType::STRING_NAME => {
                Value::String(value.to::<GString>().to_string())
            }
            VariantType::NIL => {
                self.data.remove(&key.to_string());
                return;
            }
            other => {
                godot_warn!("Blackboard set: unsupported variant type {:?}", other);
                return;
            }
        };
        self.data.insert(key.to_string(), v);
    }

    #[func]
    pub fn has(&mut self, key: GString) -> bool {
        match self.data.get(&key.to_string()) {
            Some(_) => true,
            None => false,
        }
    }
}
