use std::{collections::HashMap, sync::Arc};

use super::action::Action;

#[derive(Default)]
pub struct ActionsRepo {
    actions: HashMap<&'static str, Arc<Action>>,
}

impl ActionsRepo {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, name: &'static str, action: Action) {
        self.actions.insert(name, Arc::new(action));
    }

    pub fn get(&self, name: &str) -> Option<Arc<Action>> {
        self.actions.get(name).cloned()
    }
}
