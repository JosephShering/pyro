use std::{collections::HashMap, sync::LazyLock};

use godot::{
    classes::{FileAccess, file_access::ModeFlags, notify::ObjectNotification},
    prelude::*,
};

use crate::{core::htn::*, glue::npc::NPCBlackboards};
// use crate::core::htn::{Task, parser::parse, plan, value::Value};

#[derive(GodotClass)]
#[class(base=Resource)]
pub struct HTN {
    #[export(file = "*.htn")]
    file: GString,

    htn: Option<Task>,

    base: Base<Resource>,
}

#[godot_api]
impl IResource for HTN {
    fn init(base: Base<Resource>) -> Self {
        Self {
            file: GString::from(""),
            htn: None,
            base,
        }
    }

    fn on_notification(&mut self, what: ObjectNotification) {
        match what {
            ObjectNotification::POSTINITIALIZE => {
                let htn = HTN::load_file(&self.file);
                match htn {
                    Some(htn) => self.htn = Some(htn),
                    None => {}
                }
            }
            _ => {}
        }
    }
}

#[godot_api]
impl HTN {
    pub fn load_file(file: &GString) -> Option<Task> {
        let file = FileAccess::open(file, ModeFlags::READ)?;
        let text = file.get_as_text();
        let htn = parse(&text.to_string()).ok()?;
        Some(htn)
    }

    pub fn plan(&mut self, key: String) -> Option<Vec<String>> {
        let blackboards = NPCBlackboards::singleton();
        let guard = blackboards.bind();
        let blackboard = guard.get_blackboard(key)?;
        let htn = self.htn.as_ref()?;
        plan(htn, blackboard)
    }
}
