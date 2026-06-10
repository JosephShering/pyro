use nanoid::nanoid;
use statig::prelude::*;
use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
};

use godot::{
    classes::{CharacterBody3D, ICharacterBody3D},
    prelude::*,
};

use super::htn::HTN;
use crate::{
    core::{
        action::{Action, ActionStatus},
        *,
    },
    glue::{action_library::ActionLibrary, actor::Actor},
};

#[derive(GodotClass)]
#[class(singleton, init, base=Node)]
pub struct NPCBlackboards {
    #[export]
    action_library: OnEditor<Gd<ActionLibrary>>,

    blackboards: HashMap<String, RefCell<HashMap<String, Value>>>,
}

impl NPCBlackboards {
    pub fn register(&mut self, key: String) {
        let new_data: HashMap<String, Value> = HashMap::new();
        self.blackboards.insert(key.clone(), RefCell::new(new_data));
    }

    pub fn with_blackboard<R>(
        &self,
        key: &str,
        f: impl FnOnce(&mut HashMap<String, Value>) -> R,
    ) -> Option<R> {
        match self.blackboards.get(key) {
            Some(blackboard) => Some(f(&mut blackboard.borrow_mut())),
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
    #[export]
    htn: OnEditor<Gd<HTN>>,

    id: String,

    actor: OnEditor<Gd<Actor>>,

    base: Base<CharacterBody3D>,
}

#[godot_api]
impl ICharacterBody3D for NPC {
    fn ready(&mut self) {
        let mut blackboards = NPCBlackboards::singleton();

        let id = nanoid!();
        blackboards.bind_mut().register(id.clone());
        self.id = id;

        // let id = self.id.clone();
        // self.
        //     .signals()
        //     .tree_exited()
        //     .connect_self(|this| {
        //         let mut blackboards = NPCBlackboards::singleton();
        //         blackboards.bind_mut().cleanup(this.id);
        //     });
    }
}
