use crate::ai::{NPCBlackboards, actor::Thinker};

use super::{parser::parse, task::Task, task::plan};
use godot::{
    classes::{FileAccess, file_access::ModeFlags},
    prelude::*,
};

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct HTN {
    #[export(file = "*.txt")]
    #[var(set = set_file)]
    file: GString,

    htn: Option<Task>,

    base: Base<Resource>,
}

#[godot_api]
impl IResource for HTN {}

#[godot_api]
impl HTN {
    #[func]
    fn set_file(&mut self, file: GString) {
        self.htn = Self::load_file(&file);
        if self.htn.is_none() {
            godot_warn!("Could not load HTN file: {file}");
        }
        self.file = file;
    }

    pub fn load_file(file: &GString) -> Option<Task> {
        let file = FileAccess::open(file, ModeFlags::READ)?;
        let text = file.get_as_text();
        let htn = parse(&text.to_string()).ok()?;
        Some(htn)
    }

    pub fn plan(&self, key: &str) -> Option<Vec<String>> {
        let blackboards = NPCBlackboards::singleton();
        let guard = blackboards.bind();

        guard.with_blackboard(key, |data| {
            let htn = self.htn.as_ref()?;
            let actions = plan(htn, data.bind().get_data())?;

            Some(actions)
        })?
    }
}

#[godot_dyn]
impl Thinker for HTN {
    fn think(&self, id: &str) -> Option<Vec<String>> {
        self.plan(id)
    }
}
