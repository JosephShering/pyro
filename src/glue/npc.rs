use nanoid::nanoid;
use std::collections::HashMap;

use godot::{
    classes::{CharacterBody3D, ICharacterBody3D},
    prelude::*,
};

use super::htn::HTN;
use crate::core::*;

#[derive(GodotClass)]
#[class(singleton, init, base=Node)]
pub struct NPCBlackboards {
    blackboards: HashMap<String, HashMap<String, Value>>,
}

impl NPCBlackboards {
    pub fn register(&mut self, key: String) {
        let new_data: HashMap<String, Value> = HashMap::new();
        self.blackboards.insert(key, new_data);
    }

    pub fn get_blackboard(&self, key: String) -> Option<&HashMap<String, Value>> {
        self.blackboards.get(&key)
    }

    // pub fn get(&self, key: String, key: &str) -> Option<&Value> {
    //     let blackboard = self.blackboards.get(&key)?;
    //     blackboard.get(key)
    // }

    // pub fn set(&mut self, rid: &Rid, key: &str, value: Value) -> Option<Value> {
    //     let blackboard = self.blackboards.get_mut(rid)?;
    //     blackboard.insert(key.into(), value)
    // }

    // pub fn update<F>(&mut self, rid: &Rid, key: &str, f: F) -> Option<()>
    // where
    //     F: FnOnce(&Value) -> Value,
    // {
    //     let blackboard = self.blackboards.get_mut(rid)?;
    //     let value = blackboard.get(key)?;
    //     let new_value = f(value);

    //     blackboard.insert(key.into(), new_value);

    //     Some(())
    // }

    pub fn cleanup(&mut self, key: String) -> Option<()> {
        self.blackboards.remove(&key)?;

        Some(())
    }
}

#[derive(GodotClass)]
#[class(init, base=CharacterBody3D)]
pub struct NPC {
    #[export]
    htn: OnEditor<Gd<HTN>>,

    #[export]
    thoughts_per_second: f32,
    time: f32,

    id: String,

    base: Base<CharacterBody3D>,
}

#[godot_api]
impl ICharacterBody3D for NPC {
    fn ready(&mut self) {
        let mut blackboards = NPCBlackboards::singleton();

        let id = nanoid!();
        blackboards.bind_mut().register(id.clone());
        self.id = id;
    }

    fn physics_process(&mut self, delta: f32) {
        self.time += delta;

        let timeout = 1.0 / self.thoughts_per_second;

        while self.time >= timeout {
            self.time -= timeout;

            self.think();
        }
    }
}

#[godot_api]
impl NPC {
    fn think(&mut self) {
        let id = self.id.clone();
        let actions = self.htn.bind_mut().plan(id);
        println!("{:?}", actions);
        //return states where we will go one by one and validate the blackboard.
    }
}
