use std::collections::HashMap;

use godot::prelude::*;

use crate::ai::Value;

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
    pub fn get(&mut self, key: StringName) -> Variant {
        match self.data.get(&key.to_string()) {
            Some(value) => match value {
                Value::Int(int) => int.to_variant(),
                Value::Float(float) => float.to_variant(),
                Value::Bool(bool) => bool.to_variant(),
                Value::String(string) => string.to_variant(),
                Value::Vector3(x, y, z) => Vector3::new(*x, *y, *z).to_variant(),
                Value::Vector2(x, y) => Vector2::new(*x, *y).to_variant(),
            },
            None => Variant::nil(),
        }
    }

    #[func]
    pub fn set(&mut self, key: StringName, value: Variant) {
        let v = match value.get_type() {
            VariantType::INT => Value::Int(value.to::<i32>()),
            VariantType::FLOAT => Value::Float(value.to::<f32>()),
            VariantType::BOOL => Value::Bool(value.to::<bool>()),
            VariantType::STRING | VariantType::STRING_NAME => {
                Value::String(value.to::<GString>().to_string())
            }
            VariantType::VECTOR3 => {
                let vec3 = value.to::<Vector3>();
                Value::Vector3(vec3.x, vec3.y, vec3.z)
            }
            VariantType::VECTOR2 => {
                let vec2 = value.to::<Vector2>();
                Value::Vector2(vec2.x, vec2.y)
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
