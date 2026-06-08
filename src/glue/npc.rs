use nanoid::nanoid;
use statig::prelude::*;
use std::{cell::RefCell, collections::HashMap};

use godot::{
    classes::{CharacterBody3D, ICharacterBody3D},
    prelude::*,
};

use super::action_library::ActionLibrary;
use super::htn::HTN;
use crate::core::{action::Action, *};

#[derive(GodotClass)]
#[class(singleton, init, base=Node)]
pub struct NPCBlackboards {
    blackboards: HashMap<String, RefCell<HashMap<String, Value>>>,
}

impl NPCBlackboards {
    pub fn register(&mut self, key: String) {
        let new_data: HashMap<String, Value> = HashMap::new();
        self.blackboards.insert(key, RefCell::new(new_data));
    }

    pub fn with_blackboard<R>(
        &self,
        key: &str,
        f: impl FnOnce(&mut HashMap<String, Value>) -> R,
    ) -> Option<R> {
        self.blackboards
            .get(key)
            .map(|cell| f(&mut cell.borrow_mut()))
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

    #[export]
    thoughts_per_second: f32,
    time: f32,

    id: String,

    action: Option<Action>,

    base: Base<CharacterBody3D>,
}

#[godot_api]
impl ICharacterBody3D for NPC {
    fn ready(&mut self) {
        let mut blackboards = NPCBlackboards::singleton();

        let id = nanoid!();
        blackboards.bind_mut().register(id.clone());
        self.id = id;

        // self.actor = Some(Actor {
        //     actor_id: self.id.clone(),
        // });
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
    fn think(&mut self) -> Option<()> {
        let id = self.id.clone();
        let actions = self.htn.bind_mut().plan(id.as_str())?;

        let action_name = actions.first()?;
        let gd_action_library = ActionLibrary::singleton();

        let action = gd_action_library.bind().get(action_name.to_string())?;

        let mut blackboards = NPCBlackboards::singleton();
        blackboards.bind_mut().with_blackboard(&id, |data| {
            self.actor?.
        });

        Some(())
    }
}

enum ActorEvent {
    Tick(f32),
    Success,
    Failed,
}

struct Actor {
    actor_id: String,
}

#[state_machine(initial = "State::processing()")]
impl Actor {
    #[state(entry_action = "enter", exit_action = "exit")]
    fn processing(&mut self, event: &ActorEvent) -> Outcome<State> {
        match event {
            ActorEvent::Tick(_delta) => {
                godot_print!("ticking");

                Handled
            }
            _ => Super,
        }
    }

    #[action]
    fn enter(&mut self) {}

    #[action]
    fn exit(&mut self) {}
}
