use std::collections::HashMap;

use godot::{
    classes::{CharacterBody3D, ICharacterBody3D},
    prelude::*,
};

use crate::glue::htn::blackboard::Blackboard;

#[derive(GodotClass)]
#[class(singleton, init, base=Node)]
pub struct NPCBlackboards {
    blackboards: HashMap<String, Gd<Blackboard>>,
}

impl NPCBlackboards {
    pub fn register(&mut self, key: String) {
        let blackboard = Blackboard::new_gd();
        self.blackboards.insert(key.clone(), blackboard);
    }

    pub fn with_blackboard<R>(&self, key: &str, f: impl FnOnce(&Gd<Blackboard>) -> R) -> Option<R> {
        match self.blackboards.get(key) {
            Some(mut blackboard) => Some(f(&mut blackboard)),
            None => {
                godot_warn!("No blackboard found for {key}");
                None
            }
        }
    }

    pub fn with_blackboard_mut<R>(
        &mut self,
        key: &str,
        f: impl FnOnce(&mut Gd<Blackboard>) -> R,
    ) -> Option<R> {
        match self.blackboards.get_mut(key) {
            Some(mut blackboard) => Some(f(&mut blackboard)),
            None => {
                godot_warn!("No blackboard found for {key}");
                None
            }
        }
    }

    pub fn cleanup(&mut self, key: String) -> Option<()> {
        self.blackboards.remove(&key)?;

        Some(())
    }
}

#[derive(GodotClass)]
#[class(init, base=CharacterBody3D)]
pub struct NPC {
    base: Base<CharacterBody3D>,
}

#[godot_api]
impl ICharacterBody3D for NPC {
    fn ready(&mut self) {}
}
